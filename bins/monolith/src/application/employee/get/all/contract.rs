use async_trait::async_trait;
use error::Error;
use length_aware_paginator::Response;
use support::store::models::employee::Employee;

use super::data::{PaginatedEmployeesResponse, EmployeesAttributes};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetEmployeesContract {
    async fn get_employees_paginated(&self, attributes: EmployeesAttributes) -> Result<PaginatedEmployeesResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_employees_paginated(&self, attributes: EmployeesAttributes) -> Result<Response<Employee>, Error>;
}