use uuid::Uuid;
use serde::{ Serialize, Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct UsuarioDTO {
    pub nome: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario{
    pub id: String,
    pub nome: String,
    pub email: String,
}

impl Usuario {

    // Função para construir uma struct Usuario
    pub fn new(usuario: UsuarioDTO) -> Self{
        Self {
            id : Uuid::new_v4().to_string(),
            nome: usuario.nome,
            email: usuario.email
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_nome(&self) -> String {
        self.nome.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn set_nome(&mut self, _nome: String) {
        self.nome = _nome;
    }

    pub fn set_email(&mut self, _email: String) {
        self.email = _email;
    }

    pub fn get_usuario(&self) -> &Usuario {
        self
    }
}