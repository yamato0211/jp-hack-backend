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
        },
    },
};
use juniper::{
    FieldResult,
    graphql_object,
};

use uuid::Uuid;

// 「GraphQLのオブジェクト型」という特徴を付与する.
#[graphql_object(context=Context)]
impl Query {
    fn get_user(context: &Context, id: Uuid) -> FieldResult<User> {
        let user = users::Cruds::find_by_id(&context.pool, id)?;
        Ok(user.into())
    }

    fn hello() -> String {
        String::from("World!!")
    }

    async fn list_user(context: &Context) -> FieldResult<Vec<User>> {
        let users = users::Cruds::all_user(&context.pool)?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }
}

#[graphql_object(context=Context)]
impl Mutation {
    fn create_user(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let user = users::Cruds::insert_user(&context.pool, new_user.into())?;
 
        Ok(user.into())
    }

    fn delete_user(context: &Context, id: Uuid) -> FieldResult<User> {
        let user = users::Cruds::delete_user(&context.pool, id)?;
 
        Ok(user.into())
    }
}