use std::sync::Arc;
use actix_web::web;
use infrastructure::db::Postgres;

use super::{
    domain::GetEmployees, 
    http::handle_get_employees, 
    infrastructure::PgRepository
};

pub fn routes(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    let service = GetEmployees {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };

    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("employees")
        .route(web::get().to(handle_get_employees::<
            GetEmployees<PgRepository>    
        >))
    );
}