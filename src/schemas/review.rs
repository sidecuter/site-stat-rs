use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::entity::review;
use crate::traits::Paginate;
use actix_web::body::BoxBody;
use actix_web::Responder;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, IntoActiveModel, QueryOrder, Select, QueryFilter, ColumnTrait};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::schemas::Filter;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Problem {
    Way,
    Work,
    Plan,
    Other
}

impl From<String> for Problem {
    fn from(value: String) -> Self {
        match value {
            String::from("way") => Problem::Way,
            String::from("plan") => Problem::Plan,
            String::from("other") => Problem::Other,
            String::from("work") => Problem::Work,
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

#[derive(Serialize, Deserialize, ToSchema, Debug, MultipartForm)]
pub struct ReviewIn {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    pub text: String,
    pub problem: Problem,
    #[multipart(limit = "20 MiB")]
    pub image: Option<TempFile>
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ReviewOut {
    #[schema(example = "0b696946-f48a-47b0-b0dd-d93276d29d65")]
    pub user_id: uuid::Uuid,
    pub text: String,
    pub problem: Problem,
    #[schema(example = ".png")]
    pub image_ext: Option<String>,
    #[schema(example = "0b696946f48a47b0b0ddd93276d29d65")]
    pub image_id: Option<String>,
    #[schema(example = "2025-01-07T20:10:34.956397956")]
    pub creation_date: NaiveDateTime,
}

impl Default for ReviewIn {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            text: String::from("Some cool review"),
            problem: Problem::Other,
            image: None
        }
    }
}

impl Default for ReviewOut {
    fn default() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            text: String::from("Some cool review"),
            problem: Problem::Other,
            image_ext: Some(String::from(".png")),
            image_id: Some(String::from("0b696946f48a47b0b0ddd93276d29d65")),
            creation_date: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<review::Model> for ReviewOut {
    fn from(value: review::Model) -> Self {
        Self {
            user_id: value.user_id,
            text: value.text,
            problem: value.problem.into(),
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
            problem: value.problem.unwrap().into(),
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
        let (img_id, img_ext) = if let Some(img) = self.image {
            let img_id = uuid::Uuid::new_v4().to_string();
            let img_ext = format!(".{}", img.file_name.unwrap().split(".").last());
            let path = format!("./static/{}{}", img_id, img_ext);
            log::info!("saving to {path}");
            img.file.persist(path).unwrap();
            (Some(img_id), Some(img_ext))
        } else {
            (None, None)
        };
        review::ActiveModel {
            user_id: Set(self.user_id),
            text: Set(self.text),
            problem: Set(self.problem.to_string()),
            image_id: Set(img_id),
            image_ext: Set(img_ext),
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
