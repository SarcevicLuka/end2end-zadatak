use async_trait::async_trait;
use error::Error;
use support::store::models::employee::Employee;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetEmployeeContract {
    async fn get_employee(&self, employee_id: &str) -> Result<Employee, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_employee(&self, data: &str) -> Result<Employee, Error>;
}