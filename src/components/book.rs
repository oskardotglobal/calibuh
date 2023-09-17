use crate::models::book::Book;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BookComponentProps {
    pub book: Book,
}

#[function_component]
pub fn BookComponent(props: &BookComponentProps) -> Html {
    html! {
        <li>
            <h3>{props.book.title.clone()}</h3>

            if props.book.has_cover() {
                <img
                    src={format!("/books/{}/cover.jpg", props.book.id)}
                    alt={format!("Cover for {}", props.book.title.clone())}
                />
            }

            if props.book.has_comments() {
                <div
                    hx-get={format!("/books/{}/comments", props.book.id)}
                    hx-trigger="revealed"
                    hx-swap="outerHTML"
                />
            }

            <ul>
                <li>{"Author: "}{props.book.author_sort.clone().unwrap_or("".to_string())}</li>
                <li>{"Path: "}{props.book.path.clone()}</li>
                <li>{"Last Modified: "}{props.book.last_modified.clone().to_string()}</li>
            </ul>
        </li>
    }
}
