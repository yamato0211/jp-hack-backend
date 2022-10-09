use crate::{
    schemas::{
        root::{
            Context,
            Mutation,
            Query,
        },
    },
};
use juniper::{
    graphql_object,
};

// 「GraphQLのオブジェクト型」という特徴を付与する.
#[graphql_object(context=Context)]
impl Query {
    fn dummy_query() -> String {
        String::from("It is dummy query.")
    }
}

#[graphql_object(context=Context)]
impl Mutation {
    fn dummy_mutation() -> String {
        String::from("It is dummy mutation.")
    }
}