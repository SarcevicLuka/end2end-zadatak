use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;

use super::contract::GetEmployeeContract;

pub async fn handle_get_employee<T: GetEmployeeContract>(
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let employee_id = part_from_path::<String>(&req, "employee_id")?;

    let response = service.get_employee(&employee_id).await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}