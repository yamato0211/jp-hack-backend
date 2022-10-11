use crate::schemas::{
    root::Context,
    user::User,
};



use juniper::{
    graphql_object,
    ID,
};

#[graphql_object(context=Context)]
impl User {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn email(&self) -> String {
        self.email.clone()
    }
}