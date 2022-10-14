use juniper::*;


#[allow(dead_code)]
pub enum Error {
    AuthorizationError,
}

impl<S: ScalarValue> IntoFieldError<S> for Error {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            Error::AuthorizationError => FieldError::new(
                "Authorization Failed!!",
                graphql_value!({
                    "status": "Authorization Failed"
                }),
            ),
        }
    }
}
