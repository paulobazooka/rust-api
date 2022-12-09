use actix_web::{get, post, HttpResponse, Responder, web};
use serde_json::{json};
use crate::models::{Usuario, UsuarioDTO};


const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");
const JSON: &'static str = "application/json; charset=utf-8";

#[utoipa::path(get, path = "/", responses((status = 200, description = "Index")))]
#[get("/")]
pub async fn index() -> impl Responder {
    let body = json!(
        {
            "version": VERSION,
            "authors": AUTHORS,
            "repository": REPOSITORY,
            "documentation": "http://localhost:8080/swagger-ui/"
        }
    );

    HttpResponse::Ok()
        .content_type(JSON)
        .json(body)
}

#[utoipa::path(get, path = "/usuario", responses((status = 200, description = "Obter usuários")))]
#[get("/usuario")]
pub async fn get_usuarios() -> impl Responder {

    let mut usuarios:Vec<Usuario> = Vec::new();
    usuarios.push(Usuario::new(UsuarioDTO { nome: "João".to_string(), email: "joao@email.com".to_string() }));
    usuarios.push(Usuario::new(UsuarioDTO { nome: "Maria".to_string(), email: "maria@email.com".to_string() }));
    usuarios.push(Usuario::new(UsuarioDTO { nome: "José".to_string(), email: "jose@email.com".to_string() }));
    usuarios.push(Usuario::new(UsuarioDTO { nome: "Samanta".to_string(), email: "samanta@email.com".to_string() }));
    usuarios.push(Usuario::new(UsuarioDTO { nome: "Filomena".to_string(), email: "filomena@email.com".to_string() }));
    usuarios.push(Usuario::new(UsuarioDTO { nome: "Aristides".to_string(), email: "aristides@email.com".to_string() }));

    HttpResponse::Ok()
        .content_type(JSON)
        .json(web::Json(usuarios))
}

#[utoipa::path(post, path = "/usuario", responses((status = 200, description = "Inserir usuário")))]
#[post("/usuario")]
pub async fn set_usuario(novo_usuario: web::Json<UsuarioDTO>) -> impl Responder {
    println!("{:?}", novo_usuario);
    let usuario: Usuario = Usuario::new(novo_usuario.into_inner());
    println!("{:?}", usuario);
    HttpResponse::Created()
        .content_type(JSON)
        .json(usuario)
}