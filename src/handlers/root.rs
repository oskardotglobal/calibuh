use crate::components::page::Page;
use crate::models::book::Book;
use crate::util::{render_props, HandlerState};
use crate::BookComponent;
use axum::extract::State;
use axum::response::IntoResponse;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
struct AppProps {
    pub data: Vec<Book>,
}

#[function_component]
fn App(props: &AppProps) -> Html {
    html! {
        <Page>
            <h1>{"All Books"}</h1>
            <ul>
               {
                   props.data.iter().map(|book| {
                        html!{<BookComponent book={book.clone()} />}
                    }).collect::<Html>()
               }
            </ul>
        </Page>
    }
}
pub async fn root(State(state): State<HandlerState>) -> impl IntoResponse {
    let data = match Book::get_all(&state.pool).await {
        Ok(records) => records,
        Err(e) => panic!("Couldn't select books, got error: {}", e),
    };

    render_props::<App, AppProps>(AppProps { data }, true).await
}
