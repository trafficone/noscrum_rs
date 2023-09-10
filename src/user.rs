use dynomite::{Item, Attributes};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use leptos::*;

#[derive(Attributes, Debug, Clone, Serialize, Deserialize)]
struct UserEmail{
    address: String,
    confirmed: bool,
    opt_in: bool
}

#[derive(Attributes, Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    username: String,
    password_hash: String
}

#[derive(Item, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[dynomite(partition_key)]
    id: Uuid,
    credentials: UserCredentials,
    email: UserEmail,
}

enum LoginError {
    IncorrectUsernameOrPassword,
    NoUsernameProvided,
    InvalidUserDetails,
    UserIsSuspended
}

impl User{
    fn new(username: String, email_address: String, password: String) ->  Result<User, LoginError> {
        todo!("Create new User (logged out) with the details provided & save ty DynamoDB");
    }

    fn log_in (username: String, password_hash: String) -> Result<User, LoginError> {
        todo!("Query DynamoDB for User");
    }

    fn get_username(&self) -> &str {
        self.credentials.username.as_str()
    }

    fn get_email(&self) -> Option<&str> {
        if self.email.confirmed && self.email.opt_in {
            Some(self.email.address.as_str())
        } else {
            None
        }
    }
}

#[server(Login,"/api")]
async fn process_login(cx: Scope, credentials: UserCredentials) -> Result<User, ServerFnError> {
    todo!("Process login from user");
}

#[server(Logout,"/api")]
async fn process_logout(cx: Scope, user: User) -> Result<(), ServerFnError> {
    todo!("Process logout for user");
}

#[component]
fn login(cx: Scope) -> impl IntoView{
    todo!("login component");
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_login() {
        todo!("")
    }
}