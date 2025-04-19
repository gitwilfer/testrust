use crate::Domain::entities::user::User;
use crate::Infrastructure::Persistence::models::user_model::UserModel;
use chrono::{DateTime, Utc, NaiveDateTime};

pub fn user_to_model(user: &User) -> UserModel {
    UserModel {
        id: user.id,
        username: user.username.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        created_by: user.created_by,
        created_at: user.created_at.naive_utc(),
        updated_by: user.updated_by,
        updated_at: user.updated_at.map(|dt| dt.naive_utc()),
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
        created_at: DateTime::<Utc>::from_naive_utc_and_offset(model.created_at, Utc),
        updated_by: model.updated_by,
        updated_at: model.updated_at.map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc)),
        status: model.status,
    }
}