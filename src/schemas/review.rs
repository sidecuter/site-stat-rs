use crate::config::AppConfig;
use crate::entity::review;
use crate::errors::{ApiError, ApiResult};
use crate::schemas::Filter;
use crate::traits::Paginate;
use crate::{impl_paginate, impl_responder};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::web;
use chrono::NaiveDateTime;
use mime::Mime;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Select,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Problem {
    Way,
    Work,
    Plan,
    Other,
}

#[derive(ToSchema, Debug, MultipartForm)]
pub struct ReviewFormIn {
    #[schema(value_type = Uuid, example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: Text<Uuid>,
    #[schema(value_type = String)]
    pub text: Text<String>,
    #[schema(value_type = Problem)]
    pub problem: Text<Problem>,
    #[multipart(limit = "20 MiB")]
    #[schema(value_type = Option<String>, format = Binary, content_media_type = "application/octet-stream")]
    pub image: Option<TempFile>,
}

#[derive(Debug, Clone)]
pub struct ReviewIn {
    pub user_id: Uuid,
    pub text: String,
    pub problem: Problem,
    pub image_name: Option<String>,
}

#[derive(Serialize, ToSchema, Debug, Clone)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct ReviewOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: Uuid,
    #[schema(example = "Some cool review")]
    pub text: String,
    pub problem: Problem,
    #[schema(example = "0b696946f48a47b0b0ddd93276d29d65.png")]
    pub image_name: Option<String>,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub creation_date: NaiveDateTime,
}

#[allow(clippy::fallible_impl_from)]
impl From<String> for Problem {
    fn from(value: String) -> Self {
        match &value as &str {
            "way" => Self::Way,
            "plan" => Self::Plan,
            "other" => Self::Other,
            "work" => Self::Work,
            _ => panic!("Unexpected behavior"),
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Way => String::from("way"),
            Self::Work => String::from("work"),
            Self::Plan => String::from("plan"),
            Self::Other => String::from("other"),
        };
        write!(f, "{val}")
    }
}

impl ReviewFormIn {
    fn create_filename() -> String {
        Uuid::new_v4().to_string().replace('-', "")
    }

    fn get_file_ext(mimetype: &Mime) -> Option<String> {
        static ALLOWED_TYPES: [&str; 5] = ["png", "jpeg", "heif", "gif", "webp"];
        match mimetype.subtype().as_str().to_lowercase() {
            filetype if filetype.len() <= 4 && ALLOWED_TYPES.contains(&filetype.as_str()) => {
                Some(filetype)
            }
            _ => None,
        }
    }

    /// Saves file if it valid
    /// 
    /// # Errors
    /// 
    /// Wrong mime, no mime, unsupported image type, Non utf-8 image name, IO errors
    /// 
    /// # Panics
    /// It can't panic, cause unwrap used after check
    pub async fn save_image(self, config: &AppConfig) -> ApiResult<Option<String>> {
        Ok(if let Some(img) = self.image {
            if let Some(mime) = img.content_type.clone() {
                if mime.type_() != mime::IMAGE {
                    Err(ApiError::UnsupportedMediaType(
                        "This endpoint accepts only images".to_owned(),
                    ))?;
                }
            } else {
                Err(ApiError::UnprocessableData(
                    "File has no mime type".to_owned(),
                ))?;
            }
            let img_id = Self::create_filename();
            let img_ext = if let Some(img_ext) = Self::get_file_ext(&img.content_type.unwrap()) {
                img_ext
            } else {
                Err(ApiError::UnsupportedMediaType(
                    "Only support this 5 image types: png, jpeg, heif, gif, webp".to_owned(),
                ))?
            };
            let img_name = format!("{img_id}.{img_ext}");
            let path = Path::new(&config.static_path)
                .join(&config.files_dir)
                .join(img_name.clone())
                .to_str()
                .ok_or_else(|| ApiError::UnprocessableData(
                    "File name is not a valid UTF-8 sequence".to_owned(),
                ))?
                .to_owned();
            tracing::info!("saving to {path}");
            web::block(move || {
                if Path::new(&path).exists() {
                    std::fs::remove_file(path.clone())?;
                }
                let mut target_file = std::fs::File::create(path)?;
                let mut img_file = img.file.reopen()?;
                std::io::copy(&mut img_file, &mut target_file)
            })
            .await?
            .map_err(|e: std::io::Error| {
                tracing::error!("{e}");
                e
            })?;
            Some(img_name)
        } else {
            None
        })
    }
}

impl Default for ReviewIn {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            text: String::from("Some cool review"),
            problem: Problem::Other,
            image_name: None,
        }
    }
}

impl From<review::Model> for ReviewOut {
    fn from(value: review::Model) -> Self {
        Self {
            user_id: value.user_id,
            text: value.text,
            problem: value.problem_id.into(),
            image_name: value.image_name,
            creation_date: value.creation_date,
        }
    }
}

impl IntoActiveModel<review::ActiveModel> for ReviewIn {
    fn into_active_model(self) -> review::ActiveModel {
        review::ActiveModel {
            user_id: Set(self.user_id),
            text: Set(self.text),
            problem_id: Set(self.problem.to_string()),
            image_name: Set(self.image_name),
            creation_date: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
    }
}

impl_paginate!(ReviewOut, review);
impl_responder!(ReviewOut);
