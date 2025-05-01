use crate::entity::{aud, change_plan, plan, review, select_aud, site_stat, start_way, user_id};
use actix_web::web::Data;
use sea_orm::{DatabaseConnection, DbBackend, MockDatabase, MockExecResult, MockRow, Value};

macro_rules! btreemap {
    () => {
        std::collections::BTreeMap::new()
    };

    // Список пар ключ:значение
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut temp_map = std::collections::BTreeMap::new();
            $(temp_map.insert($key, $value);)+
            temp_map
        }
    };
}

pub trait FillDb {
    fn add_empty_row(self) -> Self;

    fn add_exec_row(self) -> Self;

    fn add_user_id(self) -> Self;

    fn add_plan(self) -> Self;

    fn add_aud(self, count: usize) -> Self;

    fn add_count(self, quantity: usize) -> Self;

    fn add_site(self) -> Self;

    fn add_select_add(self) -> Self;

    fn add_start_way(self) -> Self;

    fn add_change_plan(self) -> Self;

    fn add_review(self) -> Self;
}

impl FillDb for MockDatabase {
    fn add_empty_row(self) -> Self {
        self.append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
    }

    fn add_exec_row(self) -> Self {
        self.append_exec_results([MockExecResult {
            last_insert_id: 0,
            rows_affected: 1,
        }])
    }

    fn add_user_id(self) -> Self {
        self.append_query_results([[user_id::Model {
            user_id: Default::default(),
            creation_date: Default::default(),
        }]])
    }

    fn add_plan(self) -> Self {
        self.append_query_results([[plan::Model {
            id: Default::default(),
        }]])
    }

    fn add_aud(self, count: usize) -> Self {
        self.append_query_results(vec![
            [aud::Model {
                id: Default::default(),
            }];
            count
        ])
    }

    fn add_count(self, quantity: usize) -> Self {
        self.append_query_results(vec![
            [
                btreemap!["num_items".to_string() => Value::Int(Some(1))]
            ];
            quantity
        ])
    }

    fn add_site(self) -> Self {
        self.append_query_results([[site_stat::Model {
            id: 0,
            user_id: Default::default(),
            visit_date: Default::default(),
            endpoint: None,
        }]])
    }

    fn add_select_add(self) -> Self {
        self.append_query_results([[select_aud::Model {
            id: 0,
            user_id: Default::default(),
            visit_date: Default::default(),
            auditory_id: "".to_string(),
            success: false,
        }]])
    }

    fn add_start_way(self) -> Self {
        self.append_query_results([[start_way::Model {
            id: 0,
            user_id: Default::default(),
            start_id: "".to_string(),
            end_id: "".to_string(),
            visit_date: Default::default(),
        }]])
    }

    fn add_change_plan(self) -> Self {
        self.append_query_results([[change_plan::Model {
            id: 0,
            user_id: Default::default(),
            visit_date: Default::default(),
            plan_id: "".to_string(),
        }]])
    }

    fn add_review(self) -> Self {
        self.append_query_results([[review::Model {
            id: 0,
            user_id: Default::default(),
            creation_date: Default::default(),
            text: "".to_string(),
            image_name: None,
            problem_id: "work".to_string(),
        }]])
    }
}

pub fn get_db() -> Data<DatabaseConnection> {
    Data::new(MockDatabase::new(DbBackend::Sqlite).into_connection())
}
