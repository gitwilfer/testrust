use crate::domain::entities::user::User;
use crate::infrastructure::persistence::models::user_model::UserModel;

pub fn user_to_model(user: &User) -> UserModel {
    UserModel {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        active: user.active,
        created_at: user.created_at,
        created_by: user.created_by,
        modified_at: user.modified_at,
        modified_by: user.modified_by,
        deleted_at: user.deleted_at,
        deleted_by: user.deleted_by,
    }
}

pub fn model_to_user(model: &UserModel) -> User {
    User {
        id: model.id,
        name: model.name.clone(),
        email: model.email.clone(),
        password: model.password.clone(),
        active: model.active,
        created_at: model.created_at,
        created_by: model.created_by,
        modified_at: model.modified_at,
        modified_by: model.modified_by,
        deleted_at: model.deleted_at,
        deleted_by: model.deleted_by,
    }
}
