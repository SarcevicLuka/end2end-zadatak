use std::sync::Arc;

use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::employee::Employee;

use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn get_employee(
        &self,
        employee_id: &str,
    ) -> Result<Employee, Error> {
        let mut connection = self.pg_pool.connection()?;

        Employee::get_by_id(employee_id, &mut connection)
    }
}