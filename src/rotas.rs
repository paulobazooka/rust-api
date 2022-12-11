use actix_web::{get, post, put, delete, HttpResponse, Responder, web};
use actix_web::web::Path;

use crate::db_utils;
use crate::models::{Info, Usuario, UsuarioDTO};

const JSON: &'static str = "application/json; charset=utf-8";


#[utoipa::path(get, path = "/", responses((status = 200, description = "Index")))]
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(JSON)
        .json(Info::new())
}


#[utoipa::path(get, path = "/usuario", responses((status = 200, description = "Obter usuários", body = [Usuario])))]
#[get("/usuario")]
pub async fn get_usuarios() -> impl Responder {

    let usuarios: Vec<Usuario> = db_utils::obter_usuarios();

    // let mut usuarios:Vec<Usuario> = Vec::new();
    // usuarios.push(Usuario::new(UsuarioDTO { nome: "João".to_string(), email: "joao@email.com".to_string() }));
    // usuarios.push(Usuario::new(UsuarioDTO { nome: "Maria".to_string(), email: "maria@email.com".to_string() }));
    // usuarios.push(Usuario::new(UsuarioDTO { nome: "José".to_string(), email: "jose@email.com".to_string() }));
    // usuarios.push(Usuario::new(UsuarioDTO { nome: "Samanta".to_string(), email: "samanta@email.com".to_string() }));
    // usuarios.push(Usuario::new(UsuarioDTO { nome: "Filomena".to_string(), email: "filomena@email.com".to_string() }));
    // usuarios.push(Usuario::new(UsuarioDTO { nome: "Aristides".to_string(), email: "aristides@email.com".to_string() }));

    HttpResponse::Ok()
        .content_type(JSON)
        .json(web::Json(usuarios))
}


#[utoipa::path(post, path = "/usuario", request_body = UsuarioDTO, responses((status = 200, description = "Inserir usuário", body = Usuario)))]
#[post("/usuario")]
pub async fn set_usuario(novo_usuario: web::Json<UsuarioDTO>) -> impl Responder {
    let novo_usuario = db_utils::save_usuario(novo_usuario.into_inner());

    HttpResponse::Created()
        .content_type(JSON)
        .json(novo_usuario)
}

#[utoipa::path(get, path = "/usuario/{id}", responses((status = 200, description = "Obter usuário", body = Usuario)))]
#[get("/usuario/{id}")]
pub async fn get_usuario(id: Path<String>) -> impl Responder {

    let users = db_utils::obter_usuario_pelo_id(id.into_inner());

    HttpResponse::Ok()
        .content_type(JSON)
        .json(users)
}

#[utoipa::path(put, path = "/usuario", request_body = Usuario, responses((status = 200, description = "Atualizar usuário", body = Usuario)))]
#[put("/usuario")]
pub async fn update_usuario(usuario_atualizado: web::Json<Usuario>) -> impl Responder {
    let usuario_updated = db_utils::update_usuario(usuario_atualizado.into_inner());

    HttpResponse::Ok()
        .content_type(JSON)
        .json(usuario_updated)
}

#[utoipa::path(delete, path = "/usuario/{id}", responses((status = 200, description = "Remover usuário")))]
#[delete("/usuario/{id}")]
pub async fn delete_usuario(id: Path<String>) -> impl Responder {

    let registros_removidos = db_utils::delete_usuario_pelo_id(id.into_inner());

    HttpResponse::Ok()
        .content_type(JSON)
        .json(registros_removidos)
}