mod account;
mod login;
mod register;

use anyhow::anyhow;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{routing::get, Router};
use axum_login::{
    extractors::AuthContext, memory_store::MemoryStore as AuthMemoryStore,
    secrecy::SecretVec, AuthUser as AuthUserTrait, RequireAuthorizationLayer,
};

pub fn build_router() -> Router {
    Router::new()
        .route("/account", get(account::get_current_user))
        .route_layer(RequireAuthorizationLayer::<AuthUser, Role>::login())
        .route(
            "/register",
            get(register::get_registration_page).post(register::create_user),
        )
        .route("/login", get(login::get_login_page).post(login::login_user))
}

pub type Auth = AuthContext<AuthUser, AuthMemoryStore<AuthUser>, Role>;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: i32,
    pub password_hash: String,
    pub role: Role,
}

impl AuthUserTrait<Role> for AuthUser {
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }

    fn get_role(&self) -> Option<Role> {
        Some(self.role)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug)]
pub enum Role {
    User,
    Mod,
    Admin,
}

async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        let argon = Argon2::default();
        let hash =
            argon
                .hash_password(password.as_bytes(), &salt)
                .map_err(|error| {
                    anyhow!("Failed to generate the password hash: {}", error)
                })?;
        Ok(hash.to_string())
    })
    .await?
}

async fn verify_password(
    password: String,
    password_hash: String,
) -> anyhow::Result<()> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|error| anyhow!("Invalid password hash: {}", error))?;
        let argon = Argon2::default();

        argon
            .verify_password(password.as_bytes(), &hash)
            .map_err(|error| {
                anyhow!("Couldn't verify password hash: {}", error)
            })?;

        Ok(())
    })
    .await?
}
