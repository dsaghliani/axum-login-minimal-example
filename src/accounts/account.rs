use super::AuthUser;
use crate::errors::Error;
use axum::Extension;
use maud::Markup;

struct User {
    username: String,
}

#[allow(clippy::unused_async)]
pub async fn get_current_user(
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Markup, Error> {
    tracing::debug!("Logged in as {auth_user:#?}");

    let user = User {
        username: "example".into(),
    };

    Ok(view::render(&user))
}

mod view {
    use super::User;
    use maud::{html, Markup};

    pub(super) fn render(user: &User) -> Markup {
        html! {
            h1 { "Welcome, " (user.username) "." }
        }
    }
}
