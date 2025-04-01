/* 

// Este módulo contiene el código para la interfaz de línea de comandos.
// Define los comandos y la lógica para interactuar con la aplicación.
use clap::{Parser, Subcommand};
use crate::application::use_cases::user::{CreateUserUseCase, FindUserByIdUseCase, FindUserByUsernameUseCase, FindAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase};
use std::sync::Arc;
use uuid::Uuid;
use crate::application;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Crea un nuevo usuario
    CreateUser {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        first_name: String,
        #[arg(short, long)]
        last_name: String,
        #[arg(short, long)]
        password: String,
    },
    /// Lista todos los usuarios
    ListUsers,
    /// Busca un usuario por id
    FindUserById {
        #[arg(short, long)]
        id: String,
    },
    /// Busca un usuario por nombre de usuario
    FindUserByUsername {
        #[arg(short, long)]
        username: String,
    },
    /// Actualiza un usuario
    UpdateUser {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        username: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
        #[arg(short, long)]
        first_name: Option<String>,
        #[arg(short, long)]
        last_name: Option<String>,
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Elimina un usuario
    DeleteUser {
        #[arg(short, long)]
        id: String,
    },
    /// Run Api
    RunApi,
}

pub async fn execute_command(
    command: Commands,
) {
    let (create_user_use_case, find_user_by_id_use_case, find_user_by_username_use_case, find_all_users_use_case, update_user_use_case, delete_user_use_case) = application::create_use_cases();
    match command {
        Commands::CreateUser { username, email, first_name, last_name, password } => {
            println!("Create user: {} {} {} {} {}", username, email, first_name, last_name, password);
        }
        Commands::ListUsers => {
            println!("List users");
        }
        Commands::FindUserById { id } => {
            println!("Find user by id: {}", id);
        }
        Commands::FindUserByUsername { username } => {
            println!("Find user by username: {}", username);
        }
        Commands::UpdateUser { id, username, email, first_name, last_name, password } => {
            println!("Update user: {} {} {} {} {} {}", id, username.unwrap_or("".to_string()), email.unwrap_or("".to_string()), first_name.unwrap_or("".to_string()), last_name.unwrap_or("".to_string()), password.unwrap_or("".to_string()));
        }
        Commands::DeleteUser { id } => {
            println!("Delete user: {}", id);
        }
        Commands::RunApi => {
            println!("Run Api");
        }
    }
}

*/