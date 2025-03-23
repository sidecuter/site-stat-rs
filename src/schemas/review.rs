use std::fmt::{Display, Formatter};
use std::path::Path;
use crate::entity::review;
use crate::traits::Paginate;
use actix_web::body::BoxBody;
use actix_web::{web, Responder};
use actix_multipart::form::{
    tempfile::TempFile,
    MultipartForm,
    text::Text,
};
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, IntoActiveModel, QueryOrder, Select, QueryFilter, ColumnTrait};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::schemas::Filter;
use crate::errors::{Result as ApiResult, Error as ApiError};

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
    pub image_id: Option<String>,
    pub image_ext: Option<String>
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ReviewOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: Uuid,
    pub text: String,
    pub problem: Problem,
    #[schema(example = ".png")]
    pub image_ext: Option<String>,
    #[schema(example = "0b696946f48a47b0b0ddd93276d29d65")]
    pub image_id: Option<String>,
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
    pub async fn save_image(self, state: &AppState) -> ApiResult<(Option<String>, Option<String>)> {
        Ok(if let Some(img) = self.image {
            if let Some(mime) = img.content_type {
                if mime.type_() != mime::IMAGE {
                    Err(ApiError::UnsupportedMediaType("This endpoint accepts only images".to_owned()))?;
                }
            } else {
                Err(ApiError::UnprocessableData("File has no mime type".to_owned()))?;
            }
            let img_id = Uuid::new_v4().to_string().replace("-", "");
            let img_ext = format!(".{}", img.file_name.unwrap().split(".").last().unwrap());
            let path = Path::new(&state.files_path)
                .join(format!("{}{}", img_id, img_ext))
                .to_str()
                .ok_or(ApiError::UnprocessableData("File name is not a valid UTF-8 sequence".to_owned()))?
                .to_owned();
            log::info!("saving to {path}");
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
            (Some(img_id), Some(img_ext))
        } else {
            (None, None)
        })
    }
}

impl Default for ReviewFormIn {
    fn default() -> Self {
        Self {
            user_id: Text(Uuid::new_v4()),
            text: Text(String::from("Some cool review")),
            problem: Text(Problem::Other),
            image: None
        }
    }
}

impl Default for ReviewIn {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            text: String::from("Some cool review"),
            problem: Problem::Other,
            image_ext: None,
            image_id: None,
        }
    }
}

impl Default for ReviewOut {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            text: String::from("Some cool review"),
            problem: Problem::Other,
            image_ext: Some(String::from(".png")),
            image_id: Some(Uuid::new_v4().to_string()),
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
            image_ext: value.image_ext,
            image_id: value.image_id,
            creation_date: value.creation_date,
        }
    }
}

impl From<review::ActiveModel> for ReviewOut {
    fn from(value: review::ActiveModel) -> Self {
        Self {
            user_id: value.user_id.unwrap(),
            text: value.text.unwrap(),
            problem: value.problem_id.unwrap().into(),
            image_ext: value.image_ext.unwrap(),
            image_id: value.image_id.unwrap(),
            creation_date: value.creation_date.unwrap(),
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
            image_id: Set(self.image_id),
            image_ext: Set(self.image_ext),
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
