use std::usize;

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
const CREDENTIALS_RESPONSE: &str = "TS SCI | MS Computer Science George Washington University 2023 | BS Computer Engineering University 2012";

const SKILLS_SOFT_COMMAND: &str = " >> skills --soft ";
const SKILLS_SOFT_RESPONSE: &str = "Agile: 13yrs | Principal Investigator/Product Owner (customer facing): 7yrs | Manager: 4yrs | Proposal Writing: 3yrs";

const SKILLS_HARD_COMMAND: &str = " >> skills --hard --professional ";
const SKILLS_HARD_RESPONSE_SPACE: &str = "_";
const SKILLS_HARD_RESPONSE_SPACE_MULTIPLIER: usize = 6;

struct HardSkill {
    lang: &'static str,
    years: usize,
    color: &'static str
}

static JAVA: HardSkill = HardSkill {
    lang: "Java",
    years: 13,
    color: "red"
};


static SQL: HardSkill = HardSkill {
    lang: "SQL",
    years: 11,
    color: "white"
};

static AWS: HardSkill = HardSkill {
    lang: "AWS",
    years: 10,
    color: "orange"
};

static Docker: HardSkill = HardSkill {
    lang: "Docker",
    years: 6,
    color: "blue"
};

static Python: HardSkill = HardSkill {
    lang: "Python",
    years: 3,
    color: "purple"
};

fn main() {
    dioxus::launch(App);
}

#[derive(Props, Clone)]
struct TypedTextProps {
    text: String,
    on_complete: EventHandler<()>,
    class: Option<String>
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
    let mut is_credentials_cmd_typed = use_signal(|| false);
    let mut is_skills_soft_cmd_typed = use_signal(|| false);
    let mut is_skills_hard_cmd_typed = use_signal(|| false);
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
                            is_credentials_cmd_typed.set(true);
                        }
                    }
                }
            }
            if is_credentials_cmd_typed() {
                div {class: "response", "{CREDENTIALS_RESPONSE}"}
            }
            if is_credentials_cmd_typed() {
                div {
                    TypedText { 
                        text: "{SKILLS_SOFT_COMMAND}",
                        on_complete: move || {
                            is_skills_soft_cmd_typed.set(true);
                        }
                    }
                }
            }
            if is_skills_soft_cmd_typed() {
                div {class: "response", "{SKILLS_SOFT_RESPONSE}"}
            }
            if is_skills_soft_cmd_typed() {
                div {
                    TypedText { 
                        text: "{SKILLS_HARD_COMMAND}",
                        on_complete: move || {
                            is_skills_hard_cmd_typed.set(true);
                        }
                    }
                }
            }
            if is_skills_hard_cmd_typed() {
                div {
                    span { color: JAVA.color, font_weight: "bold", "{JAVA.lang} "} 
                    TypedText { 
                        text: std::iter::repeat(SKILLS_HARD_RESPONSE_SPACE).take(JAVA.years*SKILLS_HARD_RESPONSE_SPACE_MULTIPLIER).collect::<String>(),
                        on_complete: move || {
                            is_skills_hard_cmd_typed.set(true);
                        },
                        class: "hard_skill_spaces"
                    }
                    span {" {JAVA.years}yrs"}
                },
                div {
                    span { color: SQL.color, font_weight: "bold", "{SQL.lang} "} 
                    TypedText { 
                        text: std::iter::repeat(SKILLS_HARD_RESPONSE_SPACE).take(SQL.years*SKILLS_HARD_RESPONSE_SPACE_MULTIPLIER).collect::<String>(),
                        on_complete: move || {
                            is_skills_hard_cmd_typed.set(true);
                        },
                        class: "hard_skill_spaces"
                    }
                    span {" {SQL.years}yrs"}
                },
                div {
                    span { color: AWS.color, font_weight: "bold", "{AWS.lang} "} 
                    TypedText { 
                        text: std::iter::repeat(SKILLS_HARD_RESPONSE_SPACE).take(AWS.years*SKILLS_HARD_RESPONSE_SPACE_MULTIPLIER).collect::<String>(),
                        on_complete: move || {
                            is_skills_hard_cmd_typed.set(true);
                        },
                        class: "hard_skill_spaces"
                    }
                    span {" {AWS.years}yrs"}
                },
                div {
                    span { color: Docker.color, font_weight: "bold", "{Docker.lang} "} 
                    TypedText { 
                        text: std::iter::repeat(SKILLS_HARD_RESPONSE_SPACE).take(Docker.years*SKILLS_HARD_RESPONSE_SPACE_MULTIPLIER).collect::<String>(),
                        on_complete: move || {
                            is_skills_hard_cmd_typed.set(true);
                        },
                        class: "hard_skill_spaces"
                    }
                    span {" {Docker.years}yrs"}
                },
                div {
                    class: "response", 
                    span { color: Python.color, font_weight: "bold", "{Python.lang} "} 
                    TypedText { 
                        text: std::iter::repeat(SKILLS_HARD_RESPONSE_SPACE).take(Python.years*SKILLS_HARD_RESPONSE_SPACE_MULTIPLIER).collect::<String>(),
                        on_complete: move || {
                            is_skills_hard_cmd_typed.set(true);
                        },
                        class: "hard_skill_spaces"
                    }
                    span {" {Python.years}yrs"}
                }
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
        div { 
            class: if props.class.is_some() { props.class },
            "{typed_text}" 
        }
    }
}

