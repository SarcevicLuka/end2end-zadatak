use error::Error;
use async_trait::async_trait;
use support::store::models::employee::DisplayEmployee;

use super::{
    contract::{PgRepositoryContract, EditEmployeeContract}, 
    super::create::data::CreateEmployeeUserData
};

pub struct EditEmployee<
    A: PgRepositoryContract
> {
    pub repository: A
}

#[async_trait]
impl <A> EditEmployeeContract for EditEmployee<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn edit_employee(
        &self, 
        data: CreateEmployeeUserData,
        employee_id: &str
    ) -> Result<DisplayEmployee, Error> {
        self
            .repository
            .edit_employee(data.insertable(), employee_id)
            .await
            .map(DisplayEmployee::from)
    }
}