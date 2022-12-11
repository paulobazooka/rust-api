use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::{Usuario, UsuarioDTO};
use crate::schema::usuario;
use crate::schema::usuario::dsl::*;

fn connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Método para obter usuários
pub fn obter_usuarios() -> Vec<Usuario> {
    let connection = &mut connection();

    let results = usuario
        .load::<Usuario>(connection)
        .expect("Error loading usuarios");

    results
}

// Método para salvar um usuário
pub fn save_usuario(novo_usuario: UsuarioDTO) -> Usuario{
    let conn = &mut connection();
    let user = Usuario::new(novo_usuario);

    let new_user = diesel::insert_into(usuario::table)
        .values(&user)
        .get_result(conn)
        .expect("Error saving new usuario");

    new_user
}

// Método para obter um usuário pelo id
pub fn obter_usuario_pelo_id(id_usuario: String) -> Vec<Usuario>{
    let conn = &mut connection();

    let results = usuario
        .filter(usuario::id.eq(id_usuario))
        .load::<Usuario>(conn)
        .expect("Error loading usuario");

    results
}

// Método para atualizar um usuário
pub fn update_usuario(usuario_atualizado: Usuario) -> Usuario{
    let conn = &mut connection();

     let usuario_updated = diesel::update(usuario.find(usuario_atualizado.get_id()))
        .set((nome.eq(usuario_atualizado.get_nome()), email.eq(usuario_atualizado.get_email())))
        .get_result(conn)
        .unwrap();

    usuario_updated
}

// Método para remover um usuário pelo id
pub fn delete_usuario_pelo_id(id_usuario: String) -> usize {
    let conn = &mut connection();

    diesel::delete(usuario.filter(id.eq(id_usuario)))
        .execute(conn)
        .expect("Error deleting usuario")

}