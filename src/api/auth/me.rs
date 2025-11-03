use crate::auth::gather_rights;
use crate::schemas::Status;
use crate::{auth::CurrentUser, errors::ApiResult, schemas::UserResp};
use actix_web::{get, web};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (
            status = 200, description = "User Data", body = UserResp,
            example = json!(UserResp{
                login: "admin".to_string(),
                is_active: true,
                rights_by_goals: {
                    {
                        let mut hashmap = HashMap::new();
                        hashmap.insert(
                            "dashboards".to_string(),
                            vec![
                                "view".to_string()
                            ]
                        );
                        hashmap.insert(
                            "roles".to_string(),
                            vec![
                                "view".to_string(),
                                "create".to_string(),
                                "edit".to_string(),
                                "delete".to_string()
                            ]
                        );
                        hashmap.insert(
                            "resources".to_string(),
                            vec![
                                "view".to_string(),
                                "create".to_string(),
                                "edit".to_string(),
                                "delete".to_string()
                            ]
                        );
                        hashmap.insert(
                            "reviews".to_string(),
                            vec![
                                "view".to_string(),
                                "edit".to_string(),
                                "delete".to_string()
                            ]
                        );
                        hashmap.insert(
                            "tables".to_string(),
                            vec![
                                "view".to_string(),
                                "edit".to_string()
                            ]
                        );
                        hashmap.insert(
                            "users".to_string(),
                            vec![
                                "view".to_string(),
                                "create".to_string(),
                                "edit".to_string(),
                                "delete".to_string()
                            ]
                        );
                        hashmap.insert(
                            "stats".to_string(),
                            vec![
                                "view".to_string(),
                            ]
                        );
                        hashmap.insert(
                            "tasks".to_string(),
                            vec![
                                "view".to_string(),
                                "edit".to_string()
                            ]
                        );
                        hashmap
                    }
                }
            })
        ),
        (
            status = 500, description = "Database error", body = Status,
            example = json!(Status{status: "database error".to_string()})
        )
    ),
    tag = "Auth"
)]
#[get("/me")]
async fn me(db: web::Data<DatabaseConnection>, user: CurrentUser) -> ApiResult<UserResp> {
    let rights_and_goals = gather_rights(db.get_ref(), user.0.id).await?;
    let rights_by_goals: HashMap<String, Vec<String>> =
        rights_and_goals
            .into_iter()
            .fold(HashMap::new(), |mut map, rg| {
                map.entry(rg.goal_name).or_default().push(rg.right_name);
                map
            });
    Ok(UserResp {
        login: user.0.login,
        is_active: user.0.is_active,
        rights_by_goals,
    })
}
