use std::collections::BTreeMap;
use crate::entity::{aud, plan, user_id};
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
