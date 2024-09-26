use::dioxus::prelude::*;

#[component]
pub fn Disclaimer() -> Element {
  rsx! {
    div {
      class: "disclaimer-container",
      div {
        class: "disclaimer-caption",
        h2 { "Disclaimer!" }
        p { "This website is just my personal site. Some information may not be entirely accurate. The purpose is to keep it as a diary 📝, share experiences 💡, and for consideration if anyone is interested in offering an interview 👔 🙏🏻." },
      }
    }
  }
}
