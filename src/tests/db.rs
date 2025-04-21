use std::collections::BTreeMap;
use crate::entity::{aud, change_plan, plan, select_aud, site_stat, start_way, user_id};
use actix_web::web::Data;
use sea_orm::{DatabaseConnection, DbBackend, MockDatabase, MockExecResult, MockRow, Value};

pub fn get_db() -> Data<DatabaseConnection> {
    Data::new(MockDatabase::new(DbBackend::Sqlite).into_connection())
}

pub fn add_empty_row(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
}

pub fn add_exec_row(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_exec_results([MockExecResult {
        last_insert_id: 0,
        rows_affected: 1,
    }])
}

pub fn add_user_id(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[user_id::Model {
        user_id: Default::default(),
        creation_date: Default::default(),
    }]])
}

pub fn add_plan(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[plan::Model {
        id: Default::default(),
    }]])
}

pub fn add_aud(mock_database: MockDatabase, count: usize) -> MockDatabase {
    mock_database.append_query_results(vec![[aud::Model {
        id: Default::default(),
    }]; count])
}

pub fn add_count_result(mock_database: MockDatabase, quantity: usize) -> MockDatabase {
    let mut map = BTreeMap::new();
    map.insert("num_items".to_string(), Value::Int(Some(1)));
    mock_database.append_query_results(vec![[map]; quantity])
}

pub fn add_site(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[site_stat::Model {
        id: 0,
        user_id: Default::default(),
        visit_date: Default::default(),
        endpoint: None,
    }]])
}

pub fn add_select_add(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[select_aud::Model {
        id: 0,
        user_id: Default::default(),
        visit_date: Default::default(),
        auditory_id: "".to_string(),
        success: false,
    }]])
}

pub fn add_start_way(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[start_way::Model {
        id: 0,
        user_id: Default::default(),
        start_id: "".to_string(),
        end_id: "".to_string(),
        visit_date: Default::default(),
    }]])
}

pub fn add_change_plan(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[change_plan::Model {
        id: 0,
        user_id: Default::default(),
        visit_date: Default::default(),
        plan_id: "".to_string(),
    }]])
}
