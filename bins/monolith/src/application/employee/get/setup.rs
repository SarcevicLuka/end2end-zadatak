use std::sync::Arc;
use actix_web::web;
use infrastructure::db::Postgres;

use super::{
    domain::GetEmployee, 
    http::handle_get_employee, 
    infrastructure::PgRepository
};

pub fn routes(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    let service = GetEmployee {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };

    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("employee/{employee_id}")
        .route(web::get().to(handle_get_employee::<
            GetEmployee<PgRepository>    
        >))
    );
}