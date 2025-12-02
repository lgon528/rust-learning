use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

use crate::database::Database;
use crate::error::{AppError, Result};
use crate::models::post::{
    Post, CreatePostRequest, UpdatePostRequest, PostResponse, PostWithAuthor
};

pub struct PostService {
    db: Database,
}

impl PostService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_post(
        &self,
        author_id: &Uuid,
        request: CreatePostRequest,
    ) -> Result<PostResponse> {
        // Validate request
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

        let post_id = Uuid::new_v4();
        let row: sqlx::postgres::PgRow = sqlx::query(
            r#"
            INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
        )
        .bind(post_id)
        .bind(&request.title)
        .bind(&request.content)
        .bind(author_id)
        .bind(chrono::Utc::now())
        .bind(chrono::Utc::now())
        .fetch_one(&self.db)
        .await?;

        Ok(PostResponse {
            id: row.id,
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn get_post(&self, post_id: &Uuid) -> Result<PostWithAuthor> {
        let row = sqlx::query!(
            r#"
            SELECT
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.username as author_username
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.id = $1
            "#,
            post_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        Ok(PostWithAuthor {
            id: row.id,
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            author_username: row.author_username,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn get_posts(&self, limit: i64, offset: i64) -> Result<Vec<PostWithAuthor>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.username as author_username
            FROM posts p
            JOIN users u ON p.author_id = u.id
            ORDER BY p.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        let posts = rows.into_iter().map(|row| PostWithAuthor {
            id: row.id,
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            author_username: row.author_username,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(posts)
    }

    pub async fn get_user_posts(&self, author_id: &Uuid, limit: i64, offset: i64) -> Result<Vec<PostWithAuthor>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.username as author_username
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.author_id = $1
            ORDER BY p.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            author_id,
            limit,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        let posts = rows.into_iter().map(|row| PostWithAuthor {
            id: row.id,
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            author_username: row.author_username,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(posts)
    }

    pub async fn update_post(
        &self,
        post_id: &Uuid,
        author_id: &Uuid,
        request: UpdatePostRequest,
    ) -> Result<PostResponse> {
        // Validate request
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e)))?;

        // Check if post exists and belongs to user
        let existing_post = sqlx::query!(
            "SELECT id, author_id FROM posts WHERE id = $1",
            post_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        if existing_post.author_id != *author_id {
            return Err(AppError::Authorization("You can only update your own posts".to_string()));
        }

        // Update post
        let row = sqlx::query!(
            r#"
            UPDATE posts
            SET title = COALESCE($1, title),
                content = COALESCE($2, content),
                updated_at = NOW()
            WHERE id = $3 AND author_id = $4
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            request.title,
            request.content,
            post_id,
            author_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(PostResponse {
            id: row.id,
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn delete_post(&self, post_id: &Uuid, author_id: &Uuid) -> Result<()> {
        // Check if post exists and belongs to user
        let existing_post = sqlx::query!(
            "SELECT id, author_id FROM posts WHERE id = $1",
            post_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        if existing_post.author_id != *author_id {
            return Err(AppError::Authorization("You can only delete your own posts".to_string()));
        }

        // Delete post
        let result = sqlx::query!(
            "DELETE FROM posts WHERE id = $1 AND author_id = $2",
            post_id,
            author_id
        )
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Post not found".to_string()));
        }

        Ok(())
    }

    pub async fn search_posts(&self, query: &str, limit: i64, offset: i64) -> Result<Vec<PostWithAuthor>> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query!(
            r#"
            SELECT
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.username as author_username
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.title ILIKE $1 OR p.content ILIKE $1
            ORDER BY p.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        let posts = rows.into_iter().map(|row| PostWithAuthor {
            id: row.id,
            title: row.title,
            content: row.content,
            author_id: row.author_id,
            author_username: row.author_username,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();

        Ok(posts)
    }
}
