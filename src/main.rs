#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

mod components;
use components::pages::blogs::Blogs;
use components::pages::home::Home;
use components::ui::topBar::TopNavigationBar;
use components::ui::footerBar::FooterNavigationBar;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blogs { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            TopNavigationBar {}
            Router::<Route> {}
            FooterNavigationBar {}
        }

    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

// #[component]
// fn Home() -> Element {
//     // let mut text = use_signal(|| String::from("..."));

//     // rsx! {
//     //     Link { to: Route::Blogs { id: count() }, "Go to blog" }
//     //     div {
//     //         style: "--dimension: 80px; --color: #415d775e;",
//     //         class: "home-container",
//     //         h1 { "Hello this is dioxus counter: {count}" }
//     //         button { onclick: move |_| count += 1, "Up high!" }
//     //         button { onclick: move |_| count -= 1, "Down low!" }
//     //         button {
//     //             onclick: move |_| async move {
//     //                 if let Ok(data) = get_server_data().await {
//     //                     tracing::info!("Client received: {}", data);
//     //                     text.set(data.clone());
//     //                     post_server_data(data).await.unwrap();
//     //                 }
//     //             },
//     //             "Get Server Data"
//     //         }
//     //         p { "Server data: {text}"}
//     //     }
//     // }
//     rsx!(
//       div {
//         style: "--dimension: 80px; --color: #415d775e;",
//         class: "home-container",
//         div {
//           class: "center-content",
//         }
//         div {
//           class: "center-content p1",
//         }
//       }
//     )
// }

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
