use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use validr::Validation;

use super::{contract::EditEmployeeContract, super::create::data::CreateEmployeeUserData};

pub async fn handle_edit_employee<T: EditEmployeeContract>(
    req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<CreateEmployeeUserData>
) -> Result<HttpResponse, Error> {
    let employee_id = part_from_path::<String>(&req, "employee_id")?;
    let data = data.into_inner().validate()?;

    let response = service.edit_employee(data, &employee_id).await?;

    Ok(HttpResponse::Created()
        .json(response)
    )
}