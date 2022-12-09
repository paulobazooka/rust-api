use std::string::ToString;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Info {
    pub version: String,
    pub authors: String,
    pub repository: String,
    pub documentation: String,
}

impl Info {
    pub fn new() -> Self {
        Self {
            version: VERSION.to_string(),
            authors: AUTHORS.to_string(),
            repository: REPOSITORY.to_string(),
            documentation: "http://localhost:8080/swagger-ui/".to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UsuarioDTO {
    #[schema(example = "Nome do usuário")]
    pub nome: String,
    #[schema(example = "Email do usuário")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Usuario {
    #[schema(example = "ID do usuário")]
    pub id: String,
    #[schema(example = "Nome do usuário")]
    pub nome: String,
    #[schema(example = "Nome do usuário")]
    pub email: String,
}

impl Usuario {
    // Função para construir uma struct Usuario
    pub fn new(usuario: UsuarioDTO) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            nome: usuario.nome,
            email: usuario.email,
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