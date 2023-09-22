use actix_web::web::{ServiceConfig, self};

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/v1")
    );
}