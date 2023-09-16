use std::sync::Arc;

use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::employee::{CreateNewEmployeeData, Employee};

use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn edit_employee(
        &self,
        data: CreateNewEmployeeData,
        employee_id: &str
    ) -> Result<Employee, Error> {
        let mut connection = self.pg_pool.connection()?;

        Employee::edit_by_id(data, employee_id, &mut connection)
    }
}