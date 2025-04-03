use sea_orm::{EntityTrait, IntoActiveModel, QueryOrder, Select, QueryFilter, ColumnTrait, ActiveValue::Set};
use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use std::{fmt::{Display, Formatter}, path::Path};
use actix_web::{body::BoxBody, web, Responder};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use mime::Mime;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::errors::{ApiResult, ApiError};
use crate::app_state::AppState;
use crate::traits::Paginate;
use crate::schemas::Filter;
use crate::entity::review;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Problem {
    Way,
    Work,
    Plan,
    Other
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
    pub image: Option<TempFile>
}

#[derive(Debug, Clone)]
pub struct ReviewIn {
    pub user_id: Uuid,
    pub text: String,
    pub problem: Problem,
    pub image_name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ReviewOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: Uuid,
    pub text: String,
    pub problem: Problem,
    #[schema(example = "0b696946f48a47b0b0ddd93276d29d65.png")]
    pub image_name: Option<String>,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub creation_date: NaiveDateTime,
}

impl From<String> for Problem {
    fn from(value: String) -> Self {
        match &value as &str {
            "way" => Problem::Way,
            "plan" => Problem::Plan,
            "other" => Problem::Other,
            "work" => Problem::Work,
            _ => panic!("Unexpected behavior")
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Problem::Way => String::from("way"),
            Problem::Work => String::from("work"),
            Problem::Plan => String::from("plan"),
            Problem::Other => String::from("other"),
        };
        write!(f, "{}", val)
    }
}

impl ReviewFormIn {
    fn create_filename() -> String {
        Uuid::new_v4().to_string().replace("-", "")
    }

    fn get_file_ext(mimetype: Mime) -> Option<String> {
        match mimetype.subtype().as_str() {
            filetype if filetype.len() <= 4 => Some(filetype.to_string()),
            _ => None
        }
    }

    pub async fn save_image(self, state: &AppState) -> ApiResult<Option<String>> {
        Ok(if let Some(img) = self.image {
            if let Some(mime) = img.content_type.clone() {
                if mime.type_() != mime::IMAGE {
                    Err(ApiError::UnsupportedMediaType("This endpoint accepts only images".to_owned()))?;
                }
            } else {
                Err(ApiError::UnprocessableData("File has no mime type".to_owned()))?;
            }
            let img_id = Self::create_filename();
            let img_ext = if let Some(img_ext) = Self::get_file_ext(img.content_type.unwrap()) {
                img_ext
            } else {
                Err(ApiError::UnsupportedMediaType(
                    "Only support this 5 image types: png, jpeg, heif, gif, webp".to_owned()
                ))?
            };
            let img_name = format!("{img_id}.{img_ext}");
            let path = Path::new(&state.files_path)
                .join(img_name.clone())
                .to_str()
                .ok_or(ApiError::UnprocessableData("File name is not a valid UTF-8 sequence".to_owned()))?
                .to_owned();
            tracing::info!("saving to {path}");
            let mut target_file = web::block(|| {
                if Path::new(&path).exists() {
                    std::fs::remove_file(path.clone()).unwrap();
                };
                std::fs::File::create(path)
            }).await??;
            let mut img_file = web::block(move || {
               img.file.reopen()
            }).await??;
            web::block(move || {
                std::io::copy(&mut img_file, &mut target_file)
            }).await??;
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

impl Default for ReviewOut {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            text: String::from("Some cool review"),
            problem: Problem::Other,
            image_name: Some(format!("{}.png", Uuid::new_v4().to_string().replace("-", ""))),
            creation_date: chrono::Utc::now().naive_utc(),
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

impl Responder for ReviewOut {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
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

impl Paginate<'_, review::Entity, review::Model> for ReviewOut {
    fn get_query(filter: &Filter) -> Select<review::Entity> {
        if let Some(user_id) = filter.user_id {
            review::Entity::find()
                .filter(review::Column::UserId.eq(user_id))
                .order_by_asc(review::Column::Id)
        } else {
            review::Entity::find()
                .order_by_asc(review::Column::UserId)
        }
    }
}
