use std::sync::Arc;
use actix_web::web;
use infrastructure::db::Postgres;

use super::{domain::CreateEmployee, http::handle_add_employee, infrastructure::PgRepository};

pub fn routes(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    let service = CreateEmployee {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };

    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("employee-create")
        .route(web::post().to(handle_add_employee::<
            CreateEmployee<PgRepository>    
        >))
    );
}