use actix_identity::Identity;
use actix_web::HttpResponse;



pub async fn log_out(
    id: Identity
) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}