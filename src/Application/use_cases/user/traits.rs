// Archivo: src/application/use_cases/traits.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::application::dtos::user_dto::UserResponseDto;
use crate::application::dtos::create_user_dto::CreateUserDto;
use crate::application::dtos::update_user_dto::UpdateUserDto;
use crate::application::errors::application_error::ApplicationError;

#[async_trait]
pub trait CreateUserUseCase: Send + Sync {
    async fn execute(&self, dto: CreateUserDto) -> Result<UserResponseDto, ApplicationError>;
}

#[async_trait]
pub trait FindUserByIdUseCase: Send + Sync {
    async fn execute(&self, id: Uuid) -> Result<UserResponseDto, ApplicationError>;
}

#[async_trait]
pub trait FindUserByUsernameUseCase: Send + Sync {
    async fn execute(&self, username: &str) -> Result<UserResponseDto, ApplicationError>;
}

#[async_trait]
pub trait FindAllUsersUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<UserResponseDto>, ApplicationError>;
}

#[async_trait]
pub trait UpdateUserUseCase: Send + Sync {
    async fn execute(&self, id: Uuid, dto: UpdateUserDto, modified_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError>;
}

#[async_trait]
pub trait DeleteUserUseCase: Send + Sync {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError>;
}