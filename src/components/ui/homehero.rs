use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
  rsx! {
    div {
      class: "hero-container",
      div {
        class: "hero-caption",
        h1 { "Hello Visitor!" }
        p { "This is a complete Personal website. It does exactly built by Rust (🦀). Thank you to visited! 😇" },
        div {
          class: "code",
          style: "display: flex; padding: 0 1rem;",
          pre {
            class: "nums-line",
            "1"
          }
          pre {
            class: "code",
            style: "background-color: #454545; padding: 15px 10px 15px 18px; border-radius: 0px 8px 8px 0px;border: 1px solid #4b4b4b;",
            "Hello I'm in Bangkok, Thailand "
          }
        }
        div {
          class: "caption",
          "Let find out my pratical in this website."
        }
      }
      img {
        class: "hero-image",
        src: "/images/Rust.webp",
        // src: "https://wachiii-dev0.web.app/assets/images/avatars/avatar1.jpg",
      }
    }
  }
}

