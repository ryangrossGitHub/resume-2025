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

const CREDENTIALS_COMMAND: &str = " >> credentials ";
const CREDENTIALS_RESPONSE: &str = "Clearance | Masters Computer Science University 2023 | Bachelors Computer Engineering University 2012";

fn main() {
    dioxus::launch(App);
}

#[derive(Props, Clone)]
struct TypedTextProps {
    text: String,
    on_complete: EventHandler<()>,
}

// Manually implement PartialEq for TypedTextProps.
// We only need to compare the 'text' field for re-rendering logic.
impl PartialEq for TypedTextProps {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
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
    let mut is_command_typed = use_signal(|| false);
    let mut is_credentials_typed = use_signal(|| false);
    rsx! {
        div {
            id: "terminal",
            div {
                class: "command",
                TypedText { 
                    text: String::from(" > ") + &String::from(NAME),
                    on_complete: move || {
                        is_command_typed.set(true);
                    }
                }
            }
            if is_command_typed() {
                div {
                    TypedText { 
                        text: "{CREDENTIALS_COMMAND}",
                        on_complete: move || {
                            is_credentials_typed.set(true);
                        }
                    }
                }
            }
            if is_credentials_typed() {
                div {"{CREDENTIALS_RESPONSE}"}
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
    let on_complete = props.on_complete;
    let mut typed_text = use_signal(|| "".to_string());

    use_effect(move || {
        let value = text_to_type.clone();
        let task = spawn(async move {
            for character in value.chars() {
                // Append the next character to the signal
                typed_text.with_mut(|s| s.push(character));
                
                TimeoutFuture::new(10).await;
            }
            // Call the completion callback after the loop finishes.
            on_complete.call(());
        });
        // Cleanup function for when the component unmounts.
        use_drop(move || task.cancel());
    });

    rsx! {
        div { "{typed_text}" }
    }
}

