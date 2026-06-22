use learning_core::{LessonProgress, Stage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::{Route, Theme};
use crate::i18n::{stage_description, stage_label, t, Language};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub language: Language,
    pub theme: Theme,
    pub on_language_toggle: Callback<MouseEvent>,
    pub on_theme_toggle: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let theme_label = if props.theme == Theme::Dark {
        t(props.language, "theme_light")
    } else {
        t(props.language, "theme_dark")
    };

    html! {
        <header class="topbar">
            <Link<Route> to={Route::Home} classes="brand">
                <span class="brand-mark">
                    <img src="/public/logo.png" alt="Fuck Rust logo" />
                </span>
                <span>
                    <p class="brand-title">{ t(props.language, "brand_title") }</p>
                    <p class="brand-subtitle">{ t(props.language, "brand_subtitle") }</p>
                </span>
            </Link<Route>>
            <nav class="nav" aria-label="主导航">
                <NavLink route={Route::Home} label={t(props.language, "home")} />
                <NavLink route={Route::Learn} label={t(props.language, "learn")} />
                <NavLink route={Route::Cards} label={t(props.language, "cards")} />
                <NavLink route={Route::Stats} label={t(props.language, "stats")} />
            </nav>
            <div class="toolbar">
                <button class="tiny-button" type="button" onclick={props.on_language_toggle.clone()}>
                    { props.language.toggle_label() }
                </button>
                <button class="tiny-button" type="button" onclick={props.on_theme_toggle.clone()}>
                    { theme_label }
                </button>
            </div>
        </header>
    }
}

#[derive(Properties, PartialEq)]
struct NavLinkProps {
    route: Route,
    label: &'static str,
}

#[function_component(NavLink)]
fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <Link<Route> to={props.route.clone()} classes="nav-link">
            { props.label }
        </Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct SideRailProps {
    pub progress_rate: f32,
    pub lessons: Vec<LessonProgress>,
    pub language: Language,
    pub is_open: bool,
    pub on_toggle: Callback<MouseEvent>,
}

#[function_component(SideRail)]
pub fn side_rail(props: &SideRailProps) -> Html {
    let percent = (props.progress_rate * 100.0).round() as u8;
    let rail_class = if props.is_open {
        "side-rail is-open"
    } else {
        "side-rail is-collapsed"
    };
    let toggle_label = if props.is_open {
        t(props.language, "close_categories")
    } else {
        t(props.language, "open_categories")
    };

    html! {
        <aside class={rail_class} aria-label={t(props.language, "rail_title")}>
            <div class="rail-header">
                <p class="rail-title">{ t(props.language, "rail_title") }</p>
                <button
                    class="rail-toggle tiny-button"
                    type="button"
                    aria-controls="learning-path-rail"
                    aria-expanded={props.is_open.to_string()}
                    onclick={props.on_toggle.clone()}
                >
                    <span aria-hidden="true">{ if props.is_open { "←" } else { "☰" } }</span>
                    <span>{ toggle_label }</span>
                </button>
            </div>
            <span class="rail-collapsed-label">{ t(props.language, "path_short") }</span>
            <div id="learning-path-rail" class="rail-content" aria-hidden={(!props.is_open).to_string()}>
                <p class="rail-caption">
                    { t(props.language, "rail_caption") }
                </p>
                <div class="progress-track" aria-label={t(props.language, "total_progress")}>
                    <div class="progress-fill" style={format!("width: {percent}%")}></div>
                </div>
                <p class="rail-caption">{ format!("{} {percent}%", t(props.language, "total_progress")) }</p>
                <div class="rail-list">
                    { for props.lessons.iter().map(|lesson| render_lesson(lesson, props.language)) }
                </div>
            </div>
        </aside>
    }
}

fn render_lesson(item: &LessonProgress, language: Language) -> Html {
    let done = item.completed == item.total && item.total > 0;
    let dot_class = if done {
        "status-dot done"
    } else {
        "status-dot"
    };
    let first_exercise = item
        .lesson
        .exercise_ids
        .first()
        .copied()
        .unwrap_or_default()
        .to_owned();

    html! {
        <Link<Route> to={Route::Exercise { id: first_exercise }} classes="rail-item">
            <span class={dot_class}></span>
            <span>
                <span class="rail-item-title">{ crate::i18n::lesson_title(item.lesson, language) }</span>
                <span class="rail-item-meta">
                    { format!("{} · {}/{}", stage_label(item.lesson.stage, language), item.completed, item.total) }
                </span>
            </span>
            <span class="pill">{ format!("{} {}", item.lesson.minutes, t(language, "minute_suffix")) }</span>
        </Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct MetricCardProps {
    pub value: String,
    pub label: &'static str,
}

#[function_component(MetricCard)]
pub fn metric_card(props: &MetricCardProps) -> Html {
    html! {
        <article class="metric-card">
            <p class="metric-value">{ &props.value }</p>
            <p class="metric-label">{ props.label }</p>
        </article>
    }
}

#[derive(Properties, PartialEq)]
pub struct StageCardProps {
    pub stage: Stage,
    pub rate: f32,
    pub lessons: usize,
    pub language: Language,
}

#[function_component(StageCard)]
pub fn stage_card(props: &StageCardProps) -> Html {
    let percent = (props.rate * 100.0).round() as u8;

    html! {
        <article class="stage-card">
            <div class="tag-row">
                <span class="pill strong">{ stage_label(props.stage, props.language) }</span>
                <span class="pill">{ format!("{} {}", props.lessons, t(props.language, "lesson_count_suffix")) }</span>
            </div>
            <p class="lesson-summary">{ stage_description(props.stage, props.language) }</p>
            <div class="progress-track">
                <div class="progress-fill" style={format!("width: {percent}%")}></div>
            </div>
            <p class="metric-label">{ format!("{} {percent}%", t(props.language, "completed")) }</p>
        </article>
    }
}

#[derive(Properties, PartialEq)]
pub struct DemoBlockProps {
    pub title: &'static str,
    pub code: &'static str,
    pub output: &'static str,
    pub takeaway: &'static str,
    pub output_label: &'static str,
    pub book_url: &'static str,
    pub book_label: &'static str,
    pub guide_title: &'static str,
    pub goals_title: &'static str,
    pub guide: Vec<&'static str>,
    pub goals: Vec<&'static str>,
}

#[function_component(DemoBlock)]
pub fn demo_block(props: &DemoBlockProps) -> Html {
    html! {
        <aside class="demo-panel">
            <div class="demo-heading">
                <p class="demo-title">{ props.title }</p>
                <a class="pill strong" href={props.book_url} target="_blank" rel="noreferrer">
                    { props.book_label }
                </a>
            </div>
            <div class="guide-list">
                <p class="guide-title">{ props.guide_title }</p>
                { for props.guide.iter().map(|copy| html! { <p class="guide-copy">{ *copy }</p> }) }
            </div>
            <div class="guide-list">
                <p class="guide-title">{ props.goals_title }</p>
                { for props.goals.iter().enumerate().map(|(index, goal)| html! {
                    <p class="guide-step">
                        <span>{ format!("{:02}", index + 1) }</span>
                        { *goal }
                    </p>
                }) }
            </div>
            <pre class="code-block"><code>{ props.code }</code></pre>
            <span class="pill demo-output">{ format!("{}: {}", props.output_label, props.output) }</span>
            <p class="card-text">{ props.takeaway }</p>
        </aside>
    }
}
