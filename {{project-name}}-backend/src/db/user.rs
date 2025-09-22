use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use {{crate_name}}_lib::ReadUser;
use sqlx::{prelude::FromRow, query, query_as, sqlite::SqliteQueryResult, SqlitePool};

#[derive(FromRow)]
pub struct DbUser {
    pub id: i64,
    pub username: String,
    pub passhash: String,
}

impl DbUser {
    pub async fn insert(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> sqlx::Result<SqliteQueryResult> {
        let salt = SaltString::generate(&mut OsRng);

        let passhash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        query!(
            "INSERT INTO user (username, passhash) VALUES (?, ?)",
            username,
            passhash,
        )
        .execute(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<DbUser>> {
        query_as!(DbUser, "SELECT * FROM user WHERE id = ?", id,)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> sqlx::Result<Option<DbUser>> {
        query_as!(DbUser, "SELECT * FROM user WHERE username = ?", username,)
            .fetch_optional(pool)
            .await
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let passhash = PasswordHash::new(&self.passhash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &passhash)
            .is_ok()
    }
}

impl From<DbUser> for ReadUser {
    fn from(value: DbUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}
