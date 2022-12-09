use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use rust_api::rotas::{index, get_usuarios, set_usuario};
use rust_api::models::*;

const ADDRESS: &'static str = "0.0.0.0";
const PORT: u16 = 8080;

#[derive(OpenApi)]
#[openapi(
    paths(
        rust_api::rotas::index,
        rust_api::rotas::get_usuarios,
        rust_api::rotas::set_usuario),
    components(
        schemas(
            Usuario,
            UsuarioDTO
        )
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_usuarios)
            .service(set_usuario)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
    });

    let api = api.bind((ADDRESS, PORT))
        .expect("Erro ao iniciar o servidor!");

    println!("-- server run {}:{} -- http://localhost:8080", ADDRESS, PORT);
    //println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    api.run().await

}
