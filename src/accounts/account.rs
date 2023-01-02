use super::AuthUser;
use crate::errors::Error;
use axum::{response::IntoResponse, Extension};

struct User {
    username: String,
}

#[allow(clippy::unused_async)]
pub async fn get_current_user(
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("Logged in as {auth_user:#?}");

    let user = User {
        username: "example".into(),
    };

    Ok(view::render(&user))
}

mod view {
    use super::User;
    use axum::{
        http::{header, HeaderMap, HeaderValue},
        response::IntoResponse,
    };

    pub(super) fn render(user: &User) -> impl IntoResponse {
        let body = format!(
            r#"
            <h1>Welcome, {}!</h1>
        "#,
            user.username
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        (headers, body).into_response()
    }
}
