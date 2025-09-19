use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

const NAME: &str = "Ryan Gross";
const EMAIL: &str = "mailto:Ryan.Gross.01@gmail.com";
const EMAIL_DISPLAY: &str = "Ryan.Gross.01@gmail.com";
const GITHUB: &str = "https://github.com/ryangrossGitHub";
const GITHUB_DISPLAY: &str = "ryangrossGitHub";
const BLANK: &str = "_blank";

fn main() {
    dioxus::launch(App);
}

#[derive(Props, PartialEq, Clone)]
struct TypedTextProps {
    text: String,
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Terminal { }
    }
}


#[component]
fn Terminal() -> Element {
    rsx! {
        div {
            id: "terminal",
            div {
                class: "command",
                TypedText { text: String::from(" > ") + &String::from(NAME) }
            }
            div {
                TypedText { text: String::from(" >> credentials")}
            }
        }
        div {
            id: "links",
            a { href: EMAIL, target: "{BLANK}", "{EMAIL_DISPLAY}" }
            a { href: GITHUB, target: "{BLANK}", "{GITHUB_DISPLAY}"}
        }
    }
}


#[component]
fn TypedText(props: TypedTextProps) -> Element {
    let text_to_type = props.text;
    let mut typed_text = use_signal(|| "".to_string());

    use_effect(move || {
        let value = text_to_type.clone();
        spawn(async move {
            for character in value.chars() {
                // Append the next character to the signal
                typed_text.with_mut(|s| s.push(character));
                
                TimeoutFuture::new(10).await;
            }
        });
    });


    rsx! {
        div { "{typed_text}" }
    }
}

