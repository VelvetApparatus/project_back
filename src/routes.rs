use actix_web::web::{ServiceConfig, self, post, scope, get};

use crate::handlers::{auth::{register::reg, log_in::log_in, log_out::log_out}, chat::{show_channels::show_channels, get_messages::get_messages}};

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(
        scope("/auth")
                    .route("/reg", post().to(reg))
                    .route("log_in", post().to(log_in))
                    .route("/log_out", post().to(log_out))
            )
            .service(
        scope("/chat")
                    .route("/channels/get", get().to(show_channels))
                    .route("/messages/get", post().to(get_messages))

            )
    );
}