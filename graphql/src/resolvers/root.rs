use crate::{
    db::users,
    schemas::{
        root::{
            Context,
            Mutation,
            Query,
        },
        user::{
            User,
            NewUser,
            LoginUser,
            JWT,
        },
        error::{
            Error,
        }
    },
};
use juniper::{
    FieldResult,
    graphql_object
};

use uuid::Uuid;

fn bearer_authorization(bearer_token: Option<String>) -> Option<String> {
    if bearer_token == None {
        return None;
    }
    let bearer_token = bearer_token.unwrap();
    let token_vec = bearer_token.trim().split(" ").collect::<Vec<&str>>();
    if token_vec.len() != 2 && token_vec[0] != "Bearer" {
        return None;
    }
    let token = token_vec[1];
    let user_id = users::Jwt::decode_jwt(token);
    return user_id;
}

// 「GraphQLのオブジェクト型」という特徴を付与する.
#[graphql_object(context=Context)]
impl Query {
    fn get_user(context: &Context, id: Uuid) -> Result<User, Error> {
        let user_id = bearer_authorization(context.token.clone());
        if user_id == None {
            return Err(Error::AuthorizationError);
        }
        let user = users::Cruds::find_by_id(&context.pool, id).expect("user not found");
        Ok(user.into())
    }

    fn hello(context: &Context) -> Result<String, Error> {
        let user_id = bearer_authorization(context.token.clone());
        if user_id == None {
            return Err(Error::AuthorizationError);
        }
        Ok(String::from("World!!"))
    }

    fn sign_in(context: &Context, login_user: LoginUser) -> FieldResult<JWT>{
        let jwt = users::Cruds::authentication(&context.pool, login_user.into())?;
        Ok(JWT{jwt})
    }

    async fn list_user(context: &Context) -> Result<Vec<User>, Error> {
        let user_id = bearer_authorization(context.token.clone());
        if user_id == None {
            return Err(Error::AuthorizationError);
        }
        let users = users::Cruds::all_user(&context.pool).expect("all user failed");

        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    fn get_me(context: &Context) -> Result<User,Error> {
        let user_id = bearer_authorization(context.token.clone());
        if user_id == None {
            return Err(Error::AuthorizationError);
        }
        let user_id = Uuid::parse_str(&user_id.unwrap()).unwrap();
        let me = users::Cruds::find_by_id(&context.pool, user_id).expect("me not found");
        Ok(me.into())
    }
}

#[graphql_object(context=Context)]
impl Mutation {
    fn sign_up(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let user = users::Cruds::insert_user(&context.pool, new_user.into()).expect("sign up failed");
 
        Ok(user.into())
    }

    fn delete_user(context: &Context, id: Uuid) -> Result<User,Error> {
        let user_id = bearer_authorization(context.token.clone());
        if user_id == None {
            return Err(Error::AuthorizationError);
        }
        let user = users::Cruds::delete_user(&context.pool, id).expect("delete user failed");
 
        Ok(user.into())
    }
}