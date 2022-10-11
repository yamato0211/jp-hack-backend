use juniper::GraphQLInputObject;
use uuid::Uuid;
use crate::db::users;


pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<users::User> for User {
    fn from(user: users::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String
}

impl From<NewUser> for users::UserNewForm {
    fn from(new_user: NewUser) -> Self {
        Self {
            name: new_user.name,
            email: new_user.email,
            password: new_user.password
        }
    }
}