use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;

use super::{contract::CreateEmployeeContract, data::CreateEmployeeUserData};

pub async fn handle_add_employee<T: CreateEmployeeContract>(
    _req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<CreateEmployeeUserData>
) -> Result<HttpResponse, Error> {
    let data = data.into_inner().validate()?;
    let response = service.add_employee(data).await?;

    Ok(HttpResponse::Created()
        .json(response)
    )
}