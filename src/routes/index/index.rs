use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");
const JSON: &'static str = "application/json; charset=utf-8";

#[utoipa::path(
        get,
        path = "/",
        responses((status=200, description="Index")),
    )]
#[get("/")]
pub async fn index() -> impl Responder {
    let _body = json!(
        {
            "version": VERSION,
            "authors": AUTHORS,
            "repository": REPOSITORY
        }
    );

    HttpResponse::Ok()
        .content_type(JSON)
        .body(_body.to_string())
}