use crate::domain::entities::user::User;
use crate::infrastructure::persistence::models::user_model::UserModel;

pub fn user_to_model(user: &User) -> UserModel {
    UserModel {
        id: user.id,
        username: user.username.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        created_by: user.created_by,
        created_at: user.created_at,
        modified_by: user.modified_by,
        modified_at: user.modified_at,
        status: user.status,
    }
}

pub fn model_to_user(model: &UserModel) -> User {
    User {
        id: model.id,
        username: model.username.clone(),
        first_name: model.first_name.clone(),
        last_name: model.last_name.clone(),
        email: model.email.clone(),
        password: model.password.clone(),
        created_by: model.created_by,
        created_at: model.created_at,
        modified_by: model.modified_by,
        modified_at: model.modified_at,
        status: model.status,
    }
}