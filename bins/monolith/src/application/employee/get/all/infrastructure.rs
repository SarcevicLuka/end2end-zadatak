use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods, sql_function};
use error::Error;
use infrastructure::{db::Postgres, schema::employees};
use length_aware_paginator::{Response, Paginate};
use support::store::models::employee::Employee;

use super::{
    contract::PgRepositoryContract, 
    data::EmployeesAttributes
};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn get_employees_paginated(
        &self, 
        attributes: EmployeesAttributes
    ) -> Result<Response<Employee>, Error> {
        sql_function!(fn lower (a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);
        let mut connection = self.pg_pool.connection()?;

        let mut query = employees::table
            .into_boxed();

        if let Some(value) = &attributes.name {
            let name = format!("%{}%", value.clone());
            debug!("{}", name);
            query = query.filter(lower(employees::first_name).like(name));
        }
    
        if let Some(sort) = &attributes.sort {
            query = match sort.as_ref() {
                "ASC" => query.order(employees::created_at.asc()),
                "DESC" => query.order(employees::created_at.desc()),
                _ => query,
            };
        }

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut connection)
            .map_err(Error::from)
    }
}