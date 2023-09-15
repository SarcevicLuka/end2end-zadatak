use async_trait::async_trait;
use error::Error;
use support::store::models::employee::{CreateNewEmployeeData, Employee};

use super::data::CreateEmployeeUserData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CreateEmployeeContract {
    async fn add_employee(&self, data: CreateEmployeeUserData) -> Result<Employee, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn create_employee(&self, data: CreateNewEmployeeData) -> Result<Employee, Error>;
}