use chrono::NaiveDateTime;
use sqlx::{Pool, Sqlite};
use yew::html::ImplicitClone;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub sort: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub pubdate: Option<NaiveDateTime>,
    pub series_index: f64,
    pub author_sort: Option<String>,
    pub isbn: Option<String>,
    pub lccn: Option<String>,
    pub path: String,
    pub flags: i64,
    pub uuid: Option<String>,
    pub has_cover: Option<bool>,
    pub last_modified: NaiveDateTime,
    pub has_comments: i32,
}

impl Book {
    // instance methods
    pub fn has_comments(&self) -> bool {
        self.has_comments == 1
    }
    pub fn has_cover(&self) -> bool {
        self.has_cover.unwrap_or(false)
    }

    // static methods
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "
    SELECT books.id,
        title,
        sort,
        timestamp,
        pubdate,
        series_index,
        author_sort,
        isbn,
        lccn,
        path,
        flags,
        uuid,
        has_cover,
        last_modified,
        comments.text IS NOT NULL AS has_comments
    FROM books
    LEFT JOIN comments ON book = books.id;"
        )
        .fetch_all(pool)
        .await
    }
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "
    SELECT books.id,
        title,
        sort,
        timestamp,
        pubdate,
        series_index,
        author_sort,
        isbn,
        lccn,
        path,
        flags,
        uuid,
        has_cover,
        last_modified,
        comments.text IS NOT NULL AS has_comments
    FROM books
    LEFT JOIN comments ON book = books.id
    WHERE books.id = ?;",
            id
        )
        .fetch_one(pool)
        .await
    }
}

impl Clone for Book {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            sort: self.sort.clone(),
            timestamp: self.timestamp,
            pubdate: self.pubdate,
            series_index: self.series_index,
            author_sort: self.author_sort.clone(),
            isbn: self.isbn.clone(),
            lccn: self.lccn.clone(),
            path: self.path.clone(),
            flags: self.flags,
            uuid: self.uuid.clone(),
            has_cover: self.has_cover,
            last_modified: self.last_modified,
            has_comments: self.has_comments,
        }
    }
}

impl ImplicitClone for Book {}
