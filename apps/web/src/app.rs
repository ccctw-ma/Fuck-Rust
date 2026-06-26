use learning_core::{exercise_by_id, exercises, lesson_progress, ProgressSnapshot};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::PointerEvent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Header, SideRail};
use crate::i18n::{t, Language};
use crate::pages::{CardsPage, ExercisePage, HomePage, LearnPage, NotFoundPage, StatsPage};
use crate::storage::{
    load_language, load_progress, load_theme, save_language, save_progress, save_theme,
};

const RAIL_COLLAPSE_WIDTH: f64 = 1060.0;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/src/monaco_playground.js")]
extern "C" {
    #[wasm_bindgen(js_name = createMonacoRustEditor)]
    fn create_monaco_rust_editor(
        container: web_sys::Element,
        initial_value: &str,
        on_change: &js_sys::Function,
    ) -> wasm_bindgen::JsValue;

    #[wasm_bindgen(js_name = getMonacoEditorValue)]
    fn get_monaco_editor_value(editor: &wasm_bindgen::JsValue) -> String;

    #[wasm_bindgen(js_name = layoutMonacoEditor)]
    fn layout_monaco_editor(editor: &wasm_bindgen::JsValue);

    #[wasm_bindgen(js_name = disposeMonacoEditor)]
    fn dispose_monaco_editor(editor: &wasm_bindgen::JsValue);
}

#[cfg(not(target_arch = "wasm32"))]
fn create_monaco_rust_editor(
    _: web_sys::Element,
    _: &str,
    _: &js_sys::Function,
) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::NULL
}

#[cfg(not(target_arch = "wasm32"))]
fn get_monaco_editor_value(_: &wasm_bindgen::JsValue) -> String {
    String::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn layout_monaco_editor(_: &wasm_bindgen::JsValue) {}

#[cfg(not(target_arch = "wasm32"))]
fn dispose_monaco_editor(_: &wasm_bindgen::JsValue) {}

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
    let drawer_width = use_state(|| 640u32);
    let is_resizing = use_state(|| false);
    let editor_host = use_node_ref();
    let monaco_editor = use_mut_ref(|| None::<wasm_bindgen::JsValue>);
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
    {
        let code = code.clone();
        let editor_host = editor_host.clone();
        let monaco_editor = monaco_editor.clone();
        use_effect_with((), move |_| {
            let Some(container) = editor_host.cast::<web_sys::Element>() else {
                return Box::new(|| ()) as Box<dyn FnOnce()>;
            };
            let on_change = Closure::<dyn FnMut(String)>::wrap(Box::new(move |next: String| {
                code.set(next);
            }));
            let editor = create_monaco_rust_editor(
                container,
                &default_playground_code(),
                on_change.as_ref().unchecked_ref(),
            );
            *monaco_editor.borrow_mut() = Some(editor.clone());
            on_change.forget();

            Box::new(move || {
                dispose_monaco_editor(&editor);
            }) as Box<dyn FnOnce()>
        });
    }

    {
        let monaco_editor = monaco_editor.clone();
        use_effect_with(*drawer_width, move |_| {
            if let Some(editor) = monaco_editor.borrow().as_ref() {
                layout_monaco_editor(editor);
            }
            || ()
        });
    }

    let on_resize_start = {
        let is_resizing = is_resizing.clone();
        Callback::from(move |event: PointerEvent| {
            event.prevent_default();
            is_resizing.set(true);
        })
    };
    let on_resize_move = {
        let drawer_width = drawer_width.clone();
        let is_resizing = is_resizing.clone();
        let monaco_editor = monaco_editor.clone();
        Callback::from(move |event: PointerEvent| {
            if !*is_resizing {
                return;
            }
            let viewport_width = web_sys::window()
                .and_then(|window| window.inner_width().ok())
                .and_then(|width| width.as_f64())
                .unwrap_or(1024.0);
            let max_width = (viewport_width - 28.0).max(420.0);
            let next_width = (viewport_width - f64::from(event.client_x()))
                .clamp(420.0, max_width)
                .round() as u32;
            drawer_width.set(next_width);
            if let Some(editor) = monaco_editor.borrow().as_ref() {
                layout_monaco_editor(editor);
            }
        })
    };
    let on_resize_end = {
        let is_resizing = is_resizing.clone();
        Callback::from(move |_| is_resizing.set(false))
    };
    let run_action = {
        let output = output.clone();
        let status = status.clone();
        let language = props.language;
        let monaco_editor = monaco_editor.clone();
        let code = code.clone();
        Callback::from(move |()| {
            let current_code = monaco_editor
                .borrow()
                .as_ref()
                .map(get_monaco_editor_value)
                .unwrap_or_else(|| (*code).clone());
            code.set(current_code.clone());
            output.set(t(language, "playground_running").to_owned());
            status.set(PlaygroundStatus::Running);
            run_playground_code(current_code, output.clone(), status.clone(), language);
        })
    };
    let run_code = {
        let run_action = run_action.clone();
        Callback::from(move |_| run_action.emit(()))
    };
    html! {
        <>
            {
                if *is_resizing {
                    html! {
                        <div
                            class="playground-resize-capture"
                            onpointermove={on_resize_move.clone()}
                            onpointerup={on_resize_end.clone()}
                            onpointercancel={on_resize_end.clone()}
                        />
                    }
                } else {
                    html! {}
                }
            }
            <button
                class="playground-fab"
                type="button"
                aria-expanded={props.is_open.to_string()}
                onclick={props.on_toggle.clone()}
            >
                { if props.is_open { t(props.language, "playground_close") } else { t(props.language, "playground_open") } }
            </button>
            <aside class={drawer_class} style={drawer_style} aria-label={t(props.language, "playground_title")}>
                <button
                    class="playground-resize-rail"
                    type="button"
                    aria-label={t(props.language, "playground_resize")}
                    onpointerdown={on_resize_start}
                ></button>
                <div class="playground-header">
                    <div>
                        <p class="eyebrow">{ "RUN" }</p>
                        <h2>{ t(props.language, "playground_title") }</h2>
                        <p>{ t(props.language, "playground_copy") }</p>
                    </div>
                    <button class="tiny-button" type="button" onclick={props.on_close.clone()}>{ "×" }</button>
                </div>
                <div class="playground-editor-shell">
                    <div class="playground-tabs">
                        <span class="playground-dot red"></span>
                        <span class="playground-dot amber"></span>
                        <span class="playground-dot green"></span>
                        <span>{ "main.rs" }</span>
                        <span class="playground-tab-hint">{ "Monaco · Rust · IntelliSense" }</span>
                    </div>
                    <div class="playground-monaco-host" ref={editor_host}></div>
                </div>
                <div class="playground-actions">
                    <button
                        class="primary-button"
                        type="button"
                        data-playground-run="true"
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
