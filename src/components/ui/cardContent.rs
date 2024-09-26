use::dioxus::prelude::*;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct CardContentProps {
  title: String,
  content: String,
  image: String,
  order: String,
  #[props(default)]
  children: Option<Element>,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
  rsx! {
    div {
      class: "card-container",
      img {
        class: "card-image {props.order}",
        src: "{props.image}"
      }
      div {
          class: "card-content",
          div {
            class: "card-caption",
            h2 { "{props.title}" }
            p {
              class: "card-text",
              "{props.content}"
            }
            div {
              {props.children.as_ref()}
            }
          }
      }
    }
  }
}
