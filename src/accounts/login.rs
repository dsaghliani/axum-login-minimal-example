#![allow(clippy::module_name_repetitions, clippy::unused_async)]

use super::Auth;
use super::{verify_password, AuthUser};
use crate::errors::Error;
use anyhow::anyhow;
use axum::{
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUser {
    password: String,
}

pub async fn login_user(
    mut auth_ctx: Auth,
    Form(data): Form<LoginUser>,
) -> Result<Redirect, Error> {
    let user = AuthUser {
        id: 0,
        password_hash: "$argon2id$v=19$m=4096,t=3,p=1$L0MVanZGzDvqdp+3uJiHDg$d0R/Bac3IXudaqTIp4d4wBJaSCghXkcuU6ESy1c0JVc".into(),
    };

    verify_password(data.password, user.password_hash.clone()).await?;
    auth_ctx
        .login(&user)
        .await
        .map_err(|error| anyhow!("Couldn't log the user in: {error:#?}"))?;

    Ok(Redirect::to("/account"))
}

pub async fn get_login_page() -> impl IntoResponse {
    view::render()
}

mod view {
    use maud::html;
    use maud::Markup;

    pub(super) fn render() -> Markup {
        html! {
            h1 { "Sign In" }
            form method="POST" action="/login" {
                div {
                    label for="password" { "Password:" }
                    input type="password" name="password";
                }
                input type="submit" value="Sign In";
            }
        }
    }
}
