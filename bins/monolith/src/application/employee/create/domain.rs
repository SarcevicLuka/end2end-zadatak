use error::Error;
use async_trait::async_trait;
use support::store::models::employee::DisplayEmployee;

use super::{
    contract::{PgRepositoryContract, CreateEmployeeContract}, 
    data::CreateEmployeeUserData
};

pub struct CreateEmployee<
    A: PgRepositoryContract
> {
    pub repository: A
}

#[async_trait]
impl <A> CreateEmployeeContract for CreateEmployee<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn add_employee(
        &self, 
        data: CreateEmployeeUserData
    ) -> Result<DisplayEmployee, Error> {
        self
            .repository
            .create_employee(data.insertable())
            .await
            .map(DisplayEmployee::from)
    }
}