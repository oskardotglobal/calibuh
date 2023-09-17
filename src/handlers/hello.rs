use crate::components::page::Page;
use crate::util::render;
use axum::response::IntoResponse;
use yew::{function_component, html, Html};

#[function_component]
fn Hello() -> Html {
    html! {
        <Page>
            <h1>{"Hello, world!"}</h1>
        </Page>
    }
}

pub async fn hello() -> impl IntoResponse {
    render::<Hello>(true).await
}
