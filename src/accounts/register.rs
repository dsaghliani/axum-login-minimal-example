use super::hash_password;
use super::{Auth, AuthUser};
use crate::accounts::Role;
use crate::errors::Error;
use anyhow::anyhow;
use axum::{
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    password: String,
}

/// Normally, this would store the user data in a database.
///
/// Here, I just used it to determine the hash of "example" so I could hard-code
/// it into `login_user()`.
pub async fn create_user(
    mut auth_ctx: Auth,
    Form(data): Form<CreateUser>,
) -> Result<Redirect, Error> {
    let password_hash = hash_password(data.password).await?;

    tracing::debug!("Password hash: {password_hash}");

    let user = AuthUser {
        id: 0,
        password_hash,
        role: Role::User,
    };

    auth_ctx
        .login(&user)
        .await
        .map_err(|error| anyhow!("Couldn't log the user in: {error:#?}"))?;

    Ok(Redirect::to("/account"))
}

#[allow(clippy::unused_async)]
pub async fn get_registration_page() -> impl IntoResponse {
    view::render()
}

mod view {
    use maud::html;
    use maud::Markup;

    pub(super) fn render() -> Markup {
        html! {
            h1 { "Sign Up" }
            form method="POST" action="/register" {
                div {
                    label for="password" { "Password:" }
                    input type="password" name="password";
                }
                input type="submit" value="Sign Up";
            }
        }
    }
}
