use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::Application::dtos::auth_dto::{LoginDto, TokenDto};
use crate::Application::errors::application_error::ApplicationError;
// Cambio clave: importar el puerto de consulta en lugar del repositorio general
use crate::Application::ports::driven::repositories::UserQueryRepository;
use crate::Application::ports::driven::AuthServicePort;

pub struct LoginUseCase {
    // Cambio: Usar UserQueryRepository en lugar de UserRepositoryPort
    user_query_repository: Arc<dyn UserQueryRepository>,
    auth_service: Arc<dyn AuthServicePort>,
}

impl LoginUseCase {
    pub fn new(
        // Cambio: Recibir el repositorio de consulta
        user_query_repository: Arc<dyn UserQueryRepository>,
        auth_service: Arc<dyn AuthServicePort>,
    ) -> Self {
        LoginUseCase {
            user_query_repository,
            auth_service,
        }
    }

    pub async fn execute(&self, login_dto: LoginDto) -> Result<TokenDto, ApplicationError> {
        // 1. Buscar usuario por username usando el repositorio de consulta
        let user = self.user_query_repository.find_by_username(&login_dto.username).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al buscar usuario: {}", e)))?
            .ok_or_else(|| ApplicationError::AuthenticationError("Credenciales inválidas".to_string()))?;
        
        // 2. Verificar contraseña
        let password_valid = self.auth_service.verify_password(&login_dto.password, &user.password)
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al verificar contraseña: {}", e)))?;
        
        if !password_valid {
            return Err(ApplicationError::AuthenticationError("Credenciales inválidas".to_string()));
        }
        
        // 3. Verificar si el usuario está activo
        if user.status != 1 {
            return Err(ApplicationError::AuthenticationError("Usuario inactivo".to_string()));
        }
        
        // 4. Generar token JWT
        let token = self.auth_service.generate_token(user.id).await
            .map_err(|e| ApplicationError::InfrastructureError(format!("Error al generar token: {}", e)))?;
        
        // 5. Crear y devolver DTO con el token
        Ok(TokenDto {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 86400, // 24 horas (debería venir del servicio de auth)
            user_id: user.id,
        })
    }
}

// Trait para el caso de uso de login
#[async_trait]
impl crate::Application::use_cases::traits::LoginUseCase for LoginUseCase {
    async fn execute(&self, login_dto: LoginDto) -> Result<TokenDto, ApplicationError> {
        self.execute(login_dto).await
    }
}