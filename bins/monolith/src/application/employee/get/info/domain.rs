use error::Error;
use async_trait::async_trait;
use support::store::models::employee::Employee;

use super::contract::{PgRepositoryContract, GetEmployeeContract};

pub struct GetEmployee<
    A: PgRepositoryContract
> {
    pub repository: A
}

#[async_trait]
impl <A> GetEmployeeContract for GetEmployee<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_employee(
        &self, 
        employee_id: &str
    ) -> Result<Employee, Error> {
        self
            .repository
            .get_employee(employee_id)
            .await
    }
}