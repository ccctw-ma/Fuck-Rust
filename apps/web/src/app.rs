use learning_core::{exercise_by_id, exercises, lesson_progress, ProgressSnapshot};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlInputElement, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Header, SideRail};
use crate::i18n::{t, Language};
use crate::pages::{CardsPage, ExercisePage, HomePage, LearnPage, NotFoundPage, StatsPage};
use crate::storage::{
    load_language, load_progress, load_theme, save_language, save_progress, save_theme,
};

const RAIL_COLLAPSE_WIDTH: f64 = 1060.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub const fn code(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => Theme::Light,
        }
    }

    pub const fn toggled(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/learn")]
    Learn,
    #[at("/exercise/:id")]
    Exercise { id: String },
    #[at("/cards")]
    Cards,
    #[at("/stats")]
    Stats,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq)]
pub struct ProgressHandle {
    pub snapshot: UseStateHandle<ProgressSnapshot>,
}

impl ProgressHandle {
    pub fn record_attempt(&self, exercise_id: &str, correct: bool) {
        let mut next = (*self.snapshot).clone();
        next.record_attempt(exercise_id, correct, current_timestamp());
        save_progress(&next);
        self.snapshot.set(next);
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppShell />
        </BrowserRouter>
    }
}

#[function_component(AppShell)]
fn app_shell() -> Html {
    let progress = use_state(load_progress);
    let language = use_state(load_language);
    let theme = use_state(load_theme);
    let rail_open = use_state(|| !viewport_is_compact());
    let playground_open = use_state(|| false);
    let handle = ProgressHandle {
        snapshot: progress.clone(),
    };
    let total = exercises().len();
    let progress_rate = progress.completion_rate(total);
    let lessons = lesson_progress(&progress);
    let route = use_route::<Route>();
    let active_lesson_id = route.as_ref().and_then(active_lesson_id_for_route);
    let active_language = *language;
    let active_theme = *theme;
    let on_language_toggle = {
        let language = language.clone();
        Callback::from(move |_| {
            let next = if *language == Language::Zh {
                Language::En
            } else {
                Language::Zh
            };
            save_language(next);
            language.set(next);
        })
    };
    let on_theme_toggle = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let next = (*theme).toggled();
            save_theme(next);
            theme.set(next);
        })
    };
    let on_playground_toggle = {
        let playground_open = playground_open.clone();
        Callback::from(move |_| playground_open.set(!*playground_open))
    };
    let on_playground_close = {
        let playground_open = playground_open.clone();
        Callback::from(move |_| playground_open.set(false))
    };
    let on_rail_toggle = {
        let rail_open = rail_open.clone();
        Callback::from(move |_| {
            if viewport_is_compact() {
                rail_open.set(!*rail_open);
            } else {
                rail_open.set(true);
            }
        })
    };

    {
        use_effect_with((active_theme, active_language), move |(theme, language)| {
            apply_document_preferences(*theme, *language);
            || ()
        });
    }

    {
        let rail_open = rail_open.clone();
        use_effect_with((), move |_| install_resize_sync(rail_open))
    }

    html! {
        <div class="theme-root">
            <div class="app-shell">
                <Header
                    language={active_language}
                    theme={active_theme}
                    on_language_toggle={on_language_toggle}
                    on_theme_toggle={on_theme_toggle}
                    on_playground_toggle={on_playground_toggle.clone()}
                />
                <div class="shell-grid">
                    <SideRail
                        progress_rate={progress_rate}
                        lessons={lessons}
                        language={active_language}
                        is_open={*rail_open}
                        active_lesson_id={active_lesson_id}
                        on_toggle={on_rail_toggle}
                    />
                    <main class="main-stack">
                        <Switch<Route> render={move |route| switch(route, handle.clone(), active_language)} />
                    </main>
                </div>
                <PlaygroundDrawer
                    language={active_language}
                    is_open={*playground_open}
                    on_toggle={on_playground_toggle}
                    on_close={on_playground_close}
                />
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PlaygroundStatus {
    Ready,
    Running,
    Success,
    Failed,
}

#[derive(Properties, PartialEq)]
struct PlaygroundDrawerProps {
    language: Language,
    is_open: bool,
    on_toggle: Callback<MouseEvent>,
    on_close: Callback<MouseEvent>,
}

#[function_component(PlaygroundDrawer)]
fn playground_drawer(props: &PlaygroundDrawerProps) -> Html {
    let code = use_state(default_playground_code);
    let output = use_state(|| t(props.language, "playground_ready").to_owned());
    let status = use_state(|| PlaygroundStatus::Ready);
    let drawer_width = use_state(|| 560u32);
    let suggestions_open = use_state(|| false);
    let completions = filtered_rust_completions(&code);
    let drawer_class = if props.is_open {
        "playground-drawer is-open"
    } else {
        "playground-drawer"
    };
    let drawer_style = format!("--playground-width: {}px;", *drawer_width);
    let status_class = match *status {
        PlaygroundStatus::Ready => "playground-status",
        PlaygroundStatus::Running => "playground-status running",
        PlaygroundStatus::Success => "playground-status success",
        PlaygroundStatus::Failed => "playground-status failed",
    };
    let status_label = match *status {
        PlaygroundStatus::Ready => t(props.language, "playground_ready"),
        PlaygroundStatus::Running => t(props.language, "playground_running"),
        PlaygroundStatus::Success => t(props.language, "playground_success"),
        PlaygroundStatus::Failed => t(props.language, "playground_failed"),
    };
    let on_input = {
        let code = code.clone();
        let suggestions_open = suggestions_open.clone();
        Callback::from(move |event: InputEvent| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            let next = textarea.value();
            suggestions_open.set(!filtered_rust_completions(&next).is_empty());
            code.set(next);
        })
    };
    let on_width_input = {
        let drawer_width = drawer_width.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            if let Ok(width) = input.value().parse::<u32>() {
                drawer_width.set(width);
            }
        })
    };
    let run_action = {
        let code = code.clone();
        let output = output.clone();
        let status = status.clone();
        let language = props.language;
        Callback::from(move |()| {
            let current_code = (*code).clone();
            output.set(t(language, "playground_running").to_owned());
            status.set(PlaygroundStatus::Running);
            run_playground_code(current_code, output.clone(), status.clone(), language);
        })
    };
    let run_code = {
        let run_action = run_action.clone();
        Callback::from(move |_| run_action.emit(()))
    };
    let on_keydown = {
        let code = code.clone();
        let run_action = run_action.clone();
        let suggestions_open = suggestions_open.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Tab" && *suggestions_open {
                event.prevent_default();
                if let Some(completion) = filtered_rust_completions(&code).first() {
                    code.set(apply_completion(&code, completion.insert));
                    suggestions_open.set(false);
                }
            } else if event.key() == "Tab" {
                event.prevent_default();
                let textarea: HtmlTextAreaElement = event.target_unchecked_into();
                insert_text_at_cursor(&textarea, "    ");
                code.set(textarea.value());
            } else if event.key() == " " && event.ctrl_key() {
                event.prevent_default();
                suggestions_open.set(true);
            } else if event.key() == "Escape" {
                suggestions_open.set(false);
            } else if event.key() == "Enter" && (event.ctrl_key() || event.meta_key()) {
                event.prevent_default();
                run_action.emit(());
            }
        })
    };

    html! {
        <>
            <button
                class="playground-fab"
                type="button"
                aria-expanded={props.is_open.to_string()}
                onclick={props.on_toggle.clone()}
            >
                { if props.is_open { t(props.language, "playground_close") } else { t(props.language, "playground_open") } }
            </button>
            <aside class={drawer_class} style={drawer_style} aria-label={t(props.language, "playground_title")}>
                <div class="playground-resize-rail" aria-hidden="true"></div>
                <div class="playground-header">
                    <div>
                        <p class="eyebrow">{ "RUN" }</p>
                        <h2>{ t(props.language, "playground_title") }</h2>
                        <p>{ t(props.language, "playground_copy") }</p>
                    </div>
                    <button class="tiny-button" type="button" onclick={props.on_close.clone()}>{ "×" }</button>
                </div>
                <label class="playground-width-control">
                    <span>{ t(props.language, "playground_width") }</span>
                    <input
                        type="range"
                        min="420"
                        max="980"
                        step="10"
                        value={drawer_width.to_string()}
                        oninput={on_width_input}
                    />
                    <span>{ format!("{}px", *drawer_width) }</span>
                </label>
                <div class="playground-editor-shell">
                    <div class="playground-tabs">
                        <span class="playground-dot red"></span>
                        <span class="playground-dot amber"></span>
                        <span class="playground-dot green"></span>
                        <span>{ "main.rs" }</span>
                        <span class="playground-tab-hint">{ "Rust · IntelliSense · Ctrl Space" }</span>
                    </div>
                    <div class="playground-code-surface">
                        <pre class="playground-highlight" aria-hidden="true"><code>{ render_rust_highlight(&code) }</code></pre>
                        <textarea
                            class="playground-editor"
                            spellcheck="false"
                            autocomplete="off"
                            autocapitalize="off"
                            value={(*code).clone()}
                            oninput={on_input}
                            onkeydown={on_keydown}
                        />
                        {
                            if *suggestions_open && !completions.is_empty() {
                                html! {
                                    <div class="completion-panel" role="listbox" aria-label="Rust completions">
                                        { for completions.iter().take(8).map(|completion| {
                                            let code = code.clone();
                                            let suggestions_open = suggestions_open.clone();
                                            let insert = completion.insert;
                                            html! {
                                                <button
                                                    type="button"
                                                    class="completion-item"
                                                    onclick={Callback::from(move |_| {
                                                        code.set(apply_completion(&code, insert));
                                                        suggestions_open.set(false);
                                                    })}
                                                >
                                                    <span class="completion-label">{ completion.label }</span>
                                                    <span class="completion-kind">{ completion.kind }</span>
                                                </button>
                                            }
                                        }) }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
                <div class="playground-actions">
                    <button
                        class="primary-button"
                        type="button"
                        disabled={*status == PlaygroundStatus::Running}
                        onclick={run_code}
                    >
                        { if *status == PlaygroundStatus::Running { t(props.language, "playground_running") } else { t(props.language, "playground_run") } }
                    </button>
                    <span class={status_class}>{ status_label }</span>
                </div>
                <div class="playground-output">
                    <p class="guide-title">{ t(props.language, "playground_output") }</p>
                    <pre><code>{ (*output).clone() }</code></pre>
                </div>
            </aside>
        </>
    }
}

fn default_playground_code() -> String {
    "fn main() {\n    let mut count = 41;\n    count += 1;\n    println!(\"count = {count}\");\n}\n"
        .to_owned()
}

fn insert_text_at_cursor(textarea: &HtmlTextAreaElement, insert: &str) {
    let value = textarea.value();
    let start = textarea
        .selection_start()
        .ok()
        .flatten()
        .unwrap_or(value.len() as u32) as usize;
    let end = textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start as u32) as usize;
    let mut next = String::with_capacity(value.len() + insert.len());
    next.push_str(&value[..start]);
    next.push_str(insert);
    next.push_str(&value[end..]);
    textarea.set_value(&next);
    let cursor = (start + insert.len()) as u32;
    let _ = textarea.set_selection_range(cursor, cursor);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RustCompletion {
    label: &'static str,
    insert: &'static str,
    kind: &'static str,
}

const RUST_COMPLETIONS: &[RustCompletion] = &[
    RustCompletion {
        label: "println!",
        insert: "println!(\"{}\");",
        kind: "macro",
    },
    RustCompletion {
        label: "eprintln!",
        insert: "eprintln!(\"{}\");",
        kind: "macro",
    },
    RustCompletion {
        label: "dbg!",
        insert: "dbg!()",
        kind: "macro",
    },
    RustCompletion {
        label: "fn main",
        insert: "fn main() {\n    \n}",
        kind: "snippet",
    },
    RustCompletion {
        label: "let mut",
        insert: "let mut ",
        kind: "keyword",
    },
    RustCompletion {
        label: "match",
        insert: "match value {\n    _ => (),\n}",
        kind: "keyword",
    },
    RustCompletion {
        label: "if let",
        insert: "if let Some(value) = option {\n    \n}",
        kind: "snippet",
    },
    RustCompletion {
        label: "Result",
        insert: "Result<(), Box<dyn std::error::Error>>",
        kind: "type",
    },
    RustCompletion {
        label: "Option",
        insert: "Option",
        kind: "type",
    },
    RustCompletion {
        label: "String",
        insert: "String::from(\"\")",
        kind: "type",
    },
    RustCompletion {
        label: "Vec",
        insert: "Vec::new()",
        kind: "type",
    },
    RustCompletion {
        label: "impl",
        insert: "impl ",
        kind: "keyword",
    },
    RustCompletion {
        label: "struct",
        insert: "struct ",
        kind: "keyword",
    },
    RustCompletion {
        label: "enum",
        insert: "enum ",
        kind: "keyword",
    },
    RustCompletion {
        label: "use std::",
        insert: "use std::",
        kind: "module",
    },
];

fn filtered_rust_completions(code: &str) -> Vec<RustCompletion> {
    let prefix = current_completion_prefix(code);
    if prefix.is_empty() {
        return RUST_COMPLETIONS.iter().take(6).copied().collect();
    }
    RUST_COMPLETIONS
        .iter()
        .filter(|completion| {
            completion.label.starts_with(&prefix) || completion.insert.starts_with(&prefix)
        })
        .copied()
        .collect()
}

fn current_completion_prefix(code: &str) -> String {
    code.chars()
        .rev()
        .take_while(|character| {
            character.is_ascii_alphanumeric() || *character == '_' || *character == '!'
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

fn apply_completion(code: &str, insert: &str) -> String {
    let prefix = current_completion_prefix(code);
    if prefix.is_empty() {
        return format!("{code}{insert}");
    }
    let keep = code.len().saturating_sub(prefix.len());
    format!("{}{}", &code[..keep], insert)
}

fn render_rust_highlight(code: &str) -> Html {
    html! {
        <>
            { for code.split('\n').enumerate().map(|(index, line)| html! {
                <span class="code-line">
                    <span class="line-number">{ index + 1 }</span>
                    <span class="line-code">{ render_rust_line(line) }</span>
                </span>
            }) }
        </>
    }
}

fn render_rust_line(line: &str) -> Html {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '"' {
            if !current.is_empty() {
                tokens.push(render_rust_token(&current));
                current.clear();
            }
            let mut literal = String::from("\"");
            for next in chars.by_ref() {
                literal.push(next);
                if next == '"' {
                    break;
                }
            }
            tokens.push(html! { <span class="token string">{ literal }</span> });
        } else if character.is_ascii_alphanumeric() || character == '_' || character == '!' {
            current.push(character);
        } else {
            if !current.is_empty() {
                tokens.push(render_rust_token(&current));
                current.clear();
            }
            tokens.push(html! { <span>{ character }</span> });
        }
    }
    if !current.is_empty() {
        tokens.push(render_rust_token(&current));
    }
    html! { <>{ for tokens }</> }
}

fn render_rust_token(token: &str) -> Html {
    let class_name = if is_rust_keyword(token) {
        "token keyword"
    } else if token.ends_with('!') {
        "token macro"
    } else if token.chars().next().is_some_and(char::is_uppercase) {
        "token type"
    } else if token.chars().all(|character| character.is_ascii_digit()) {
        "token number"
    } else {
        "token ident"
    };
    html! { <span class={class_name}>{ token.to_owned() }</span> }
}

fn is_rust_keyword(token: &str) -> bool {
    matches!(
        token,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}

#[cfg(target_arch = "wasm32")]
fn run_playground_code(
    code: String,
    output: UseStateHandle<String>,
    status: UseStateHandle<PlaygroundStatus>,
    language: Language,
) {
    use gloo_net::http::Request;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_futures::spawn_local;

    #[derive(Serialize)]
    struct ExecuteRequest {
        channel: &'static str,
        mode: &'static str,
        edition: &'static str,
        #[serde(rename = "crateType")]
        crate_type: &'static str,
        tests: bool,
        code: String,
        backtrace: bool,
    }

    #[derive(Deserialize)]
    struct ExecuteResponse {
        success: bool,
        stdout: String,
        stderr: String,
    }

    spawn_local(async move {
        let request = ExecuteRequest {
            channel: "stable",
            mode: "debug",
            edition: "2021",
            crate_type: "bin",
            tests: false,
            code,
            backtrace: false,
        };
        let result = async {
            let response = Request::post("https://play.rust-lang.org/execute")
                .json(&request)
                .map_err(|err| err.to_string())?
                .send()
                .await
                .map_err(|err| err.to_string())?;
            response
                .json::<ExecuteResponse>()
                .await
                .map_err(|err| err.to_string())
        }
        .await;

        match result {
            Ok(response) => {
                let mut combined = String::new();
                if !response.stdout.trim().is_empty() {
                    combined.push_str(&response.stdout);
                }
                if !response.stderr.trim().is_empty() {
                    if !combined.is_empty() {
                        combined.push('\n');
                    }
                    combined.push_str(&response.stderr);
                }
                if combined.trim().is_empty() {
                    combined.push_str(if response.success {
                        "程序执行完成，无输出。"
                    } else {
                        "执行失败，但没有返回输出。"
                    });
                }
                status.set(if response.success {
                    PlaygroundStatus::Success
                } else {
                    PlaygroundStatus::Failed
                });
                output.set(combined);
            }
            Err(err) => {
                status.set(PlaygroundStatus::Failed);
                output.set(format!("{}\n{err}", t(language, "playground_error")));
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn run_playground_code(
    _: String,
    output: UseStateHandle<String>,
    status: UseStateHandle<PlaygroundStatus>,
    language: Language,
) {
    status.set(PlaygroundStatus::Failed);
    output.set(t(language, "playground_error").to_owned());
}

fn active_lesson_id_for_route(route: &Route) -> Option<&'static str> {
    match route {
        Route::Exercise { id } => exercise_by_id(id).map(|exercise| exercise.lesson_id),
        _ => None,
    }
}

fn switch(route: Route, progress: ProgressHandle, language: Language) -> Html {
    match route {
        Route::Home => html! { <HomePage progress={progress} language={language} /> },
        Route::Learn => html! { <LearnPage progress={progress} language={language} /> },
        Route::Exercise { id } => {
            html! { <ExercisePage id={id} progress={progress} language={language} /> }
        }
        Route::Cards => html! { <CardsPage language={language} /> },
        Route::Stats => html! { <StatsPage progress={progress} language={language} /> },
        Route::NotFound => html! { <NotFoundPage language={language} /> },
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_document_preferences(theme: Theme, language: Language) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.document_element() else {
        return;
    };
    let _ = root.set_attribute("data-theme", theme.code());
    let _ = root.set_attribute("lang", language.code());
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_document_preferences(_: Theme, _: Language) {}

#[cfg(target_arch = "wasm32")]
fn viewport_is_compact() -> bool {
    web_sys::window()
        .and_then(|window| window.inner_width().ok())
        .and_then(|width| width.as_f64())
        .map(|width| width < RAIL_COLLAPSE_WIDTH)
        .unwrap_or(true)
}

#[cfg(not(target_arch = "wasm32"))]
fn viewport_is_compact() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn install_resize_sync(rail_open: UseStateHandle<bool>) -> Box<dyn FnOnce()> {
    let Some(window) = web_sys::window() else {
        return Box::new(|| ());
    };
    rail_open.set(!viewport_is_compact());

    let handler = Closure::<dyn FnMut()>::wrap(Box::new({
        let rail_open = rail_open.clone();
        move || rail_open.set(!viewport_is_compact())
    }));
    let _ = window.add_event_listener_with_callback("resize", handler.as_ref().unchecked_ref());

    Box::new(move || {
        if let Some(window) = web_sys::window() {
            let _ = window
                .remove_event_listener_with_callback("resize", handler.as_ref().unchecked_ref());
        }
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn install_resize_sync(_: UseStateHandle<bool>) -> Box<dyn FnOnce()> {
    Box::new(|| ())
}

fn current_timestamp() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Date::now() / 1000.0) as u64
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}
