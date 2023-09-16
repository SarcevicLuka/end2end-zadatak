use async_trait::async_trait;
use error::Error;
use support::store::models::employee::{CreateNewEmployeeData, Employee, DisplayEmployee};

use super::super::create::data::CreateEmployeeUserData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EditEmployeeContract {
    async fn edit_employee(&self, data: CreateEmployeeUserData, employee_id: &str) -> Result<DisplayEmployee, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn edit_employee(&self, data: CreateNewEmployeeData, employee_id: &str) -> Result<Employee, Error>;
}