use actix_web::web::Data;
use sea_orm::{DatabaseConnection, DbBackend, MockDatabase, MockExecResult, MockRow};
use crate::entity::{aud, plan, user_id};

pub fn get_db() -> Data<DatabaseConnection> {
    Data::new(MockDatabase::new(DbBackend::Sqlite).into_connection())
}

pub fn add_empty_row(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results::<MockRow, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
}

pub fn add_exec_row(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_exec_results([MockExecResult {
        last_insert_id: 0,
        rows_affected: 1
    }])
}

pub fn add_user_id(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[user_id::Model {
        user_id: Default::default(),
        creation_date: Default::default()
    }]])
}

pub fn add_plan(mock_database: MockDatabase) -> MockDatabase {
    mock_database.append_query_results([[plan::Model {
        id: Default::default()
    }]])
}

pub fn add_aud(mock_database: MockDatabase, count: u8) -> MockDatabase {
    let mut mock_database = mock_database;
    for _ in 0..count {
        mock_database = mock_database.append_query_results([[aud::Model {
            id: Default::default()
        }]]);
    }
    mock_database
}
