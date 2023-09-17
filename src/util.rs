use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::env;
use std::path::PathBuf;
use yew::{BaseComponent, Properties, ServerRenderer};

#[derive(Properties, PartialEq)]
struct EmptyProps {}

pub async fn render<ComponentT>(force_doctype: bool) -> impl IntoResponse
where
    ComponentT: BaseComponent<Properties = ()>,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/html"
            .parse()
            .expect("text/html is not a valid content type?"),
    );

    let renderer = ServerRenderer::<ComponentT>::new();
    let mut page = renderer.render().await;

    if force_doctype {
        page = format!("<!DOCTYPE html>\n{}", page);
    }

    (headers, page)
}

pub async fn render_props<ComponentT, PropsT>(
    props: PropsT,
    force_doctype: bool,
) -> impl IntoResponse
where
    ComponentT: BaseComponent<Properties = PropsT>,
    PropsT: Properties + Send + 'static,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/html"
            .parse()
            .expect("text/html is not a valid content type?"),
    );

    let renderer = ServerRenderer::<ComponentT>::with_props(move || props);
    let mut page = renderer.render().await;

    if force_doctype {
        page = format!("<!DOCTYPE html>\n{}", page);
    }

    (headers, page)
}

pub struct HandlerState {
    pub pool: Pool<Sqlite>,
    pub library_path: PathBuf,
}

impl Clone for HandlerState {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            library_path: self.library_path.clone(),
        }
    }
}

impl HandlerState {
    pub async fn new() -> Self {
        dotenv().ok();
        let library_path = env::var("LIBRARY_PATH").expect("LIBRARY_PATH must be set");
        let database_url = format!("sqlite://{}/metadata.db", library_path.clone());

        let pool = SqlitePoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap_or_else(|err| {
                panic!(
                    "Couldn't connect to database at {}, got error: {}",
                    database_url, err
                )
            });

        Self {
            pool,
            library_path: PathBuf::from(library_path.as_str()),
        }
    }
}
