mod accounts;

use accounts::AuthUser;
use axum::Router;
use axum_login::{
    axum_sessions::{
        async_session::MemoryStore as SessionMemoryStore, SessionLayer,
    },
    memory_store::MemoryStore as AuthMemoryStore,
    AuthLayer,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[must_use]
pub fn build_router() -> Router {
    let secret: [u8; 64] = rand::random();
    let session_layer = {
        let session_store = SessionMemoryStore::new();
        SessionLayer::new(session_store, &secret).with_secure(false)
    };
    let auth_layer = {
        let store = Arc::new(RwLock::new(HashMap::new()));
        let user_store = AuthMemoryStore::<AuthUser>::new(&store);
        AuthLayer::new(user_store, &secret)
    };

    accounts::build_router()
        .layer(auth_layer)
        .layer(session_layer)
}

mod errors {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Something went wrong with the server.")]
        Anyhow(#[from] anyhow::Error),
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let Self::Anyhow(ref error) = self;
            tracing::error!("Encountered an error: {error:?}.");

            (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
        }
    }
}
