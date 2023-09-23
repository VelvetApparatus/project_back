use actix_web::web::{ServiceConfig, self, post, scope};

use crate::handlers::auth::{register::reg, log_in::log_in, log_out::log_out};

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(
        scope("/auth")
                    .route("/reg", post().to(reg))
                    .route("log_in", post().to(log_in))
                    .route("/log_out", post().to(log_out))
            )
    );
}