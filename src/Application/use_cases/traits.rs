use async_trait::async_trait;
use uuid::Uuid;
use crate::Application::dtos::user_dto::UserResponseDto;
use crate::Application::dtos::create_user_dto::CreateUserDto;
use crate::Application::dtos::update_user_dto::UpdateUserDto;
use crate::Application::dtos::auth_dto::{LoginDto, TokenDto};
use crate::Application::errors::application_error::ApplicationError;

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
#[async_trait]
pub trait UpdateUserUseCase: Send + Sync {
    async fn execute(&self, id: Uuid, dto: UpdateUserDto, modified_by: Option<Uuid>) -> Result<UserResponseDto, ApplicationError>;
}

#[async_trait]
pub trait DeleteUserUseCase: Send + Sync {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError>;
}


#[async_trait]
pub trait LoginUseCase: Send + Sync {
    async fn execute(&self, dto: LoginDto) -> Result<TokenDto, ApplicationError>;
}