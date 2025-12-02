use crate::error::{AppError, Result};
use crate::models::post::{
    CreatePostRequest, UpdatePostRequest, PostResponse, PostWithAuthor
};

#[derive(Clone)]
pub struct PostService {
    db: crate::database::Database,
}

impl PostService {
    pub fn new(db: crate::database::Database) -> Self {
        Self { db }
    }

    pub async fn create_post(&self, author_id: &Uuid, request: CreatePostRequest) -> Result<PostResponse> {
        request.validate()
            .map_err(|e| AppError::Validation(format!("Validation error: {}", e))?;

        let post_id = uuid::Uuid::new_v4();
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
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_post(&self, post_id: &Uuid) -> Result<PostWithAuthor> {
        let row: sqlx::postgres::PgRow = sqlx::query(
            r#"
            SELECT
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.username as author_username
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.id = $1
            "#,
        )
        .bind(post_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        Ok(PostWithAuthor {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            author_username: row.get("author_username"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_posts(&self, limit: i64, offset: i64) -> Result<Vec<PostWithAuthor>> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(
            r#"
            SELECT
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.username as author_username
            FROM posts p
            JOIN users u ON p.author_id = u.id
            ORDER BY p.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        let mut posts = Vec::new();
        for row in rows {
            posts.push(PostWithAuthor {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                author_id: row.get("author_id"),
                author_username: row.get("author_username"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(posts)
    }

    pub async fn get_user_posts(&self, user_id: &Uuid, limit: i64, offset: i64) -> Result<Vec<PostWithAuthor>> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(
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
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        let mut posts = Vec::new();
        for row in rows {
            posts.push(PostWithAuthor {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                author_id: row.get("author_id"),
                author_username: row.get("author_username"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(posts)
    }

    pub async fn search_posts(&self, query: &str, limit: i64, offset: i64) -> Result<Vec<PostWithAuthor>> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(
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
        )
        .bind(format!("%{}%", query))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;
        let mut posts = Vec::new();
        for row in rows {
            posts.push(PostWithAuthor {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                author_id: row.get("author_id"),
                author_username: row.get("author_username"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(posts)
    }

    pub async fn update_post(&self, post_id: &Uuid, author_id: &Uuid, request: UpdatePostRequest) -> Result<PostResponse> {
        let row: sqlx::postgres::PgRow = sqlx::query(
            r#"
            UPDATE posts
            SET title = COALESCE($1, title),
            content = COALESCE($2, content),
            updated_at = NOW()
            WHERE id = $3 AND author_id = $4
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
        )
        .bind(request.title.as_ref())
        .bind(request.content.as_ref())
        .bind(post_id)
        .bind(author_id)
        .fetch_one(&self.db)
        .await?;

        Ok(PostResponse {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn delete_post(&self, post_id: &Uuid, author_id: &Uuid) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM posts WHERE id = $1 AND author_id = $2"
        )
        .bind(post_id)
        .bind(author_id)
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Post not found or access denied".to_string());
        }

        Ok(())
    }
}
