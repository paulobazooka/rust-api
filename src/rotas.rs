use actix_web::{get, post, HttpResponse, Responder, web};
use crate::models::{Usuario, UsuarioDTO, Info};

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


#[utoipa::path(post, path = "/usuario", request_body = UsuarioDTO, responses((status = 200, description = "Inserir usuário", body = Usuario)))]
#[post("/usuario")]
pub async fn set_usuario(novo_usuario: web::Json<UsuarioDTO>) -> impl Responder {
    let usuario: Usuario = Usuario::new(novo_usuario.into_inner());
    HttpResponse::Created()
        .content_type(JSON)
        .json(usuario)
}