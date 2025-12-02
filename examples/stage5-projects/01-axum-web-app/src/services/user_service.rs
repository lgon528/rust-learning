use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

use crate::database::Database;
use crate::error::{AppError, Result};
use crate::models::user::{
    User, CreateUserRequest, UpdateUserRequest, LoginRequest, UserResponse, LoginResponse
};
use crate::auth::JwtService;

#[derive(Clone)]
pub struct UserService {
    db: Database,
    jwt_service: JwtService,
}

impl UserService {
    pub fn new(db: Database, jwt_service: JwtService) -> Self {
        Self { db, jwt_service }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse> {
        // Validate request
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

        // Check if username already exists
        if self.username_exists(&request.username).await? {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        // Check if email already exists
        if self.email_exists(&request.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        // Hash password
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(AppError::PasswordHashing)?;

        // Create user
        let user = User {
            id: Uuid::new_v4(),
            username: request.username,
            email: request.email,
            password_hash,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let row: sqlx::postgres::PgRow = sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, email, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.db)
        .await?;

        Ok(UserResponse {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub fn get_db(&self) -> &Database {
        &self.db
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        // Find user by username
        let user: sqlx::postgres::PgRow = sqlx::query(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE username = $1",
        )
        .bind(&request.username)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

        // Verify password
        if !verify(&request.password, &user.get::<_, String>("password_hash"))
            .map_err(AppError::PasswordHashing)?
        {
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        // Generate JWT token
        let token = self.jwt_service.generate_token(
            &user.get::<_, String>("id"),
            &user.get::<_, String>("username"),
        )?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: "24h".to_string(),
            user: UserResponse {
                id: user.get("id"),
                username: user.get("username"),
                email: user.get("email"),
                created_at: user.get("created_at"),
                updated_at: user.get("updated_at"),
            },
        })
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<UserResponse> {
        let user: sqlx::postgres::PgRow = sqlx::query(
            "SELECT id, username, email, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserResponse {
            id: user.get("id"),
            username: user.get("username"),
            email: user.get("email"),
            created_at: user.get("created_at"),
            updated_at: user.get("updated_at"),
        })
    }

    pub async fn update_user(&self, user_id: &Uuid, request: UpdateUserRequest) -> Result<UserResponse> {
        // Validate request
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

        // Check if user exists
        let mut user = self.get_user(user_id).await?;

        // Update username if provided
        if let Some(ref username) = request.username {
            if self.username_exists(username).await? && username != &user.username {
                return Err(AppError::Conflict("Username already exists".to_string()));
            }
            user.username = username.clone();
        }

        // Update email if provided
        if let Some(ref email) = request.email {
            if self.email_exists(email).await? && email != &user.email {
                return Err(AppError::Conflict("Email already exists".to_string()));
            }
            user.email = email.clone();
        }

        // Update in database
        let row: sqlx::postgres::PgRow = sqlx::query(
            r#"
            UPDATE users
            SET username = COALESCE($1, username),
                email = COALESCE($2, email),
                updated_at = NOW()
            WHERE id = $3
            RETURNING id, username, email, created_at, updated_at
            "#,
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        Ok(UserResponse {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn delete_user(&self, user_id: &Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn username_exists(&self, username: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_one(&self.db)
        .await?;

        Ok(count > 0)
    }

    async fn email_exists(&self, email: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(&self.db)
        .await?;

        Ok(count > 0)
    }
}
