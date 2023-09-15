use actix_web::web;
use infrastructure::db::Postgres;
use std::sync::Arc;

pub fn configure(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    employee(postgres, cfg);
}

fn employee(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    crate::application::employee::create::setup::routes(postgres.clone(), cfg);
    crate::application::employee::get::setup::routes(postgres, cfg);
}