use error::Error;
use async_trait::async_trait;

use super::{
    contract::{PgRepositoryContract, GetEmployeesContract}, 
    data::{PaginatedEmployeesResponse, EmployeesAttributes}
};

pub struct GetEmployees<
    A: PgRepositoryContract
> {
    pub repository: A
}

#[async_trait]
impl <A> GetEmployeesContract for GetEmployees<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_employees_paginated(
        &self, 
        attributes: EmployeesAttributes
    ) -> Result<PaginatedEmployeesResponse, Error> {
        self
            .repository
            .get_employees_paginated(attributes)
            .await
            .map(PaginatedEmployeesResponse::from)
    }
}