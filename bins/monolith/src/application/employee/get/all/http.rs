use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;

use super::{contract::GetEmployeesContract, data::EmployeesAttributes};

pub async fn handle_get_employees<T: GetEmployeesContract>(
    _req: HttpRequest,
    attributes: web::Query<EmployeesAttributes>,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let attributes = attributes.into_inner().validate()?;

    let response = service.get_employees_paginated(attributes).await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}