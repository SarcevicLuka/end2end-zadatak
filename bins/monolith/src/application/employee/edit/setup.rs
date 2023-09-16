use std::sync::Arc;
use actix_web::web;
use infrastructure::db::Postgres;

use super::{domain::EditEmployee, http::handle_edit_employee, infrastructure::PgRepository};

pub fn routes(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    let service = EditEmployee {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };

    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("employees/{employee_id}/edit")
        .route(web::post().to(handle_edit_employee::<
            EditEmployee<PgRepository>    
        >))
    );
}