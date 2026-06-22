use learning_core::{exercises, lesson_progress, ProgressSnapshot};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Header, SideRail};
use crate::i18n::Language;
use crate::pages::{CardsPage, ExercisePage, HomePage, LearnPage, NotFoundPage, StatsPage};
use crate::storage::{
    load_language, load_progress, load_theme, save_language, save_progress, save_theme,
};

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
    let rail_open = use_state(|| false);
    let handle = ProgressHandle {
        snapshot: progress.clone(),
    };
    let total = exercises().len();
    let progress_rate = progress.completion_rate(total);
    let lessons = lesson_progress(&progress);
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
    let on_rail_toggle = {
        let rail_open = rail_open.clone();
        Callback::from(move |_| rail_open.set(!*rail_open))
    };

    {
        use_effect_with((active_theme, active_language), move |(theme, language)| {
            apply_document_preferences(*theme, *language);
            || ()
        });
    }

    html! {
        <div class="theme-root">
            <div class="app-shell">
                <Header
                    language={active_language}
                    theme={active_theme}
                    on_language_toggle={on_language_toggle}
                    on_theme_toggle={on_theme_toggle}
                />
                <div class="shell-grid">
                    <SideRail
                        progress_rate={progress_rate}
                        lessons={lessons}
                        language={active_language}
                        is_open={*rail_open}
                        on_toggle={on_rail_toggle}
                    />
                    <main class="main-stack">
                        <Switch<Route> render={move |route| switch(route, handle.clone(), active_language)} />
                    </main>
                </div>
            </div>
        </div>
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
