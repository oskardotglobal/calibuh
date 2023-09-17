use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub children: Html,
}

#[function_component]
pub fn Page(props: &PageProps) -> Html {
    html! {<>
        <html>
        <head>
            <title>{"calibuh"}</title>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />

            <script
                src="https://unpkg.com/htmx.org@1.9.5"
                integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
                crossorigin="anonymous"
                defer={true}
            ></script>

            <link rel="stylesheet" href="/static/css/main.css" />
        </head>
        <body>
            {props.children.clone()}
        </body>
        </html>
    </>}
}
