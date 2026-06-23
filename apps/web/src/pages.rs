use learning_core::{
    cards, exercise_by_id, exercises, exercises_for_lesson, lessons, recommend_next_exercise,
    recommend_next_lesson, stage_summaries, Exercise, ExerciseKind, UserAnswer,
};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::TargetCast;
use yew_router::prelude::*;

use crate::app::{ProgressHandle, Route};
use crate::components::{DemoBlock, MetricCard, StageCard};
use crate::i18n::{
    answer_summary, card_fix, card_summary, card_title, demo_takeaway, demo_title,
    exercise_explanation, exercise_hint, exercise_kind_label, exercise_level_label,
    exercise_option, exercise_prompt, exercise_title, lesson_goal, lesson_guide, lesson_summary,
    lesson_title, stage_label, t, Language,
};

#[derive(Properties, PartialEq)]
pub struct ProgressPageProps {
    pub progress: ProgressHandle,
    pub language: Language,
}

#[function_component(HomePage)]
pub fn home_page(props: &ProgressPageProps) -> Html {
    let snapshot = &*props.progress.snapshot;
    let language = props.language;
    let total = exercises().len();
    let completed = snapshot.completed_exercises.len();
    let correct_rate = percent(snapshot.correct_rate());
    let next_exercise = recommend_next_exercise(snapshot);
    let next_lesson = recommend_next_lesson(snapshot);
    let next_link = next_exercise
        .map(|exercise| Route::Exercise {
            id: exercise.id.to_owned(),
        })
        .unwrap_or(Route::Learn);

    html! {
        <>
            <section class="hero-card">
                <span class="eyebrow">{ t(language, "hero_eyebrow") }</span>
                <h1 class="hero-title">{ t(language, "hero_title_prefix") }<span>{ t(language, "hero_title_accent") }</span></h1>
                <p class="hero-copy">
                    { t(language, "hero_copy") }
                </p>
                <div class="hero-actions">
                    <Link<Route> to={next_link} classes="primary-button">
                        {
                            next_lesson
                                .map(|lesson| format!("{}{}", t(language, "continue"), lesson_title(lesson, language)))
                                .unwrap_or_else(|| t(language, "view_path").to_owned())
                        }
                    </Link<Route>>
                    <Link<Route> to={Route::Cards} classes="ghost-button">{ t(language, "review_cards") }</Link<Route>>
                </div>
                <div class="metric-grid">
                    <MetricCard value={format!("{completed}/{total}")} label={t(language, "done_exercises")} />
                    <MetricCard value={correct_rate} label={t(language, "accuracy")} />
                    <MetricCard value={format!("{} {}", snapshot.streak_days(), t(language, "days"))} label={t(language, "streak")} />
                </div>
            </section>
            <section class="panel full">
                <h2 class="section-title">{ t(language, "four_stages") }</h2>
                <p class="section-copy">{ t(language, "four_stages_copy") }</p>
                <div class="panel-grid">
                    { for stage_summaries(snapshot).into_iter().map(|summary| html! {
                        <StageCard
                            stage={summary.stage}
                            lessons={summary.lesson_count}
                            rate={summary.completion_rate()}
                            language={language}
                        />
                    })}
                </div>
            </section>
        </>
    }
}

#[function_component(LearnPage)]
pub fn learn_page(props: &ProgressPageProps) -> Html {
    let snapshot = &*props.progress.snapshot;
    let language = props.language;

    html! {
        <section class="panel full">
            <h1 class="section-title">{ t(language, "learn_title") }</h1>
            <p class="section-copy">{ t(language, "learn_copy") }</p>
            <div class="lesson-grid">
                { for lessons().iter().map(|lesson| {
                    let lesson_exercises = learning_core::exercises_for_lesson(lesson.id);
                    let total = lesson_exercises.len();
                    let completed = lesson_exercises
                        .iter()
                        .filter(|exercise| snapshot.is_completed(exercise.id))
                        .count();
                    let first_id = lesson_exercises
                        .first()
                        .map(|exercise| exercise.id)
                        .unwrap_or_default()
                        .to_owned();

                    html! {
                        <article class="lesson-card">
                            <div class="tag-row">
                                <span class="pill strong">{ stage_label(lesson.stage, language) }</span>
                                <span class="pill">{ format!("{} {}", t(language, "difficulty"), lesson.difficulty) }</span>
                                <span class="pill">{ format!("{} {}", lesson.minutes, t(language, "minute_suffix")) }</span>
                                <a class="pill" href={lesson.book_url} target="_blank" rel="noreferrer">{ t(language, "book_ref") }</a>
                            </div>
                            <h2 class="lesson-title">{ lesson_title(lesson, language) }</h2>
                            <p class="lesson-summary">{ lesson_summary(lesson, language) }</p>
                            <ul class="goal-list">
                                { for lesson.goals.iter().enumerate().map(|(index, _)| html! { <li class="tag">{ lesson_goal(lesson, index, language) }</li> }) }
                            </ul>
                            <div class="progress-track">
                                <div
                                    class="progress-fill"
                                    style={format!("width: {}%", completed * 100 / total)}
                                ></div>
                            </div>
                            <p class="metric-label">{ format!("{} {}/{}", t(language, "completed"), completed, total) }</p>
                            <Link<Route> to={Route::Exercise { id: first_id }} classes="tiny-button">
                                { t(language, "enter_exercise") }
                            </Link<Route>>
                        </article>
                    }
                }) }
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct ExercisePageProps {
    pub id: String,
    pub progress: ProgressHandle,
    pub language: Language,
}

#[function_component(ExercisePage)]
pub fn exercise_page(props: &ExercisePageProps) -> Html {
    let Some(exercise) = exercise_by_id(&props.id) else {
        return html! { <NotFoundPage language={props.language} /> };
    };
    let language = props.language;
    let lesson_position = exercise_lesson_position(exercise);

    let selected_choice = use_state(|| None::<usize>);
    let text_value = use_state(String::new);
    let ordered_values = use_state(Vec::<String>::new);
    let feedback = use_state(|| None::<(bool, String, &'static str)>);

    {
        let selected_choice = selected_choice.clone();
        let text_value = text_value.clone();
        let ordered_values = ordered_values.clone();
        let feedback = feedback.clone();
        let id = props.id.clone();
        use_effect_with(id, move |_| {
            selected_choice.set(None);
            text_value.set(String::new());
            ordered_values.set(Vec::new());
            feedback.set(None);
            || ()
        });
    }

    let on_text = {
        let text_value = text_value.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            text_value.set(input.value());
        })
    };
    let on_output = {
        let text_value = text_value.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlTextAreaElement = event.target_unchecked_into();
            text_value.set(input.value());
        })
    };
    let on_reset_order = {
        let ordered_values = ordered_values.clone();
        Callback::from(move |_| ordered_values.set(Vec::new()))
    };
    let on_submit = {
        let selected_choice = selected_choice.clone();
        let text_value = text_value.clone();
        let ordered_values = ordered_values.clone();
        let feedback = feedback.clone();
        let progress = props.progress.clone();
        Callback::from(move |_| {
            if let Some(answer) =
                draft_answer(exercise, *selected_choice, &text_value, &ordered_values)
            {
                let outcome = exercise.check(&answer);
                progress.record_attempt(exercise.id, outcome.correct);
                feedback.set(Some((
                    outcome.correct,
                    answer_summary(exercise.answer, language),
                    exercise_explanation(exercise, language),
                )));
            }
        })
    };

    html! {
        <section class="exercise-panel">
            { render_demo_for_exercise(exercise, language) }
            <article>
                <div class="exercise-meta">
                    <span class="eyebrow">{ exercise_kind_label(exercise.kind, language) }</span>
                    <span class="eyebrow level-pill">{ exercise_level_label(exercise, language) }</span>
                    {
                        if let Some((lesson_title_text, current, total)) = lesson_position {
                            html! {
                                <span class="eyebrow exercise-index-pill">
                                    { exercise_index_label(language, lesson_title_text, current, total) }
                                </span>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <h1 class="exercise-title">{ exercise_title(exercise, language) }</h1>
                <p class="exercise-prompt">{ exercise_prompt(exercise, language) }</p>
                {
                    if exercise.code.is_empty() {
                        html! {}
                    } else {
                        html! { <pre class="code-block"><code>{ exercise.code }</code></pre> }
                    }
                }
                { render_answer_input(exercise, AnswerInputState {
                    selected_choice: selected_choice.clone(),
                    text_value: text_value.clone(),
                    ordered_values: ordered_values.clone(),
                    on_text,
                    on_output,
                    on_reset_order,
                    language,
                }) }
                <div class="button-row">
                    <button class="primary-button" type="button" onclick={on_submit}>{ t(language, "submit") }</button>
                    <Link<Route> to={next_route(exercise.id)} classes="ghost-button">{ t(language, "next_exercise") }</Link<Route>>
                </div>
                {
                    if let Some((correct, expected, explanation)) = &*feedback {
                        let class_name = if *correct { "feedback correct" } else { "feedback wrong" };
                        let title = if *correct { t(language, "correct_title") } else { t(language, "wrong_title") };
                        html! {
                            <div class={class_name}>
                                <p class="feedback-title">{ title }</p>
                                <p class="card-text">{ format!("{}{expected}", t(language, "expected")) }</p>
                                <p class="card-text">{ *explanation }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="feedback">
                                <p class="feedback-title">{ t(language, "hint") }</p>
                                <p class="card-text">{ exercise_hint(exercise, language) }</p>
                            </div>
                        }
                    }
                }
            </article>
        </section>
    }
}

struct AnswerInputState {
    selected_choice: UseStateHandle<Option<usize>>,
    text_value: UseStateHandle<String>,
    ordered_values: UseStateHandle<Vec<String>>,
    on_text: Callback<InputEvent>,
    on_output: Callback<InputEvent>,
    on_reset_order: Callback<MouseEvent>,
    language: Language,
}

fn render_answer_input(exercise: &'static Exercise, state: AnswerInputState) -> Html {
    match exercise.kind {
        ExerciseKind::SingleChoice => html! {
            <div class="answer-stack">
                { for exercise.options.iter().enumerate().map(|(index, _option)| {
                    let selected_choice = state.selected_choice.clone();
                    let selected = *selected_choice == Some(index);
                    let class_name = if selected { "option-button selected" } else { "option-button" };
                    html! {
                        <button
                            class={class_name}
                            type="button"
                            onclick={Callback::from(move |_| selected_choice.set(Some(index)))}
                        >
                            { exercise_option(exercise, index, state.language) }
                        </button>
                    }
                }) }
            </div>
        },
        ExerciseKind::FillBlank => html! {
            <div class="answer-stack">
                <input
                    class="text-input"
                    value={(*state.text_value).clone()}
                    placeholder={t(state.language, "input_blank")}
                    oninput={state.on_text}
                />
            </div>
        },
        ExerciseKind::CodeOutput => html! {
            <div class="answer-stack">
                <textarea
                    class="text-input"
                    value={(*state.text_value).clone()}
                    placeholder={t(state.language, "input_output")}
                    rows="4"
                    oninput={state.on_output}
                />
            </div>
        },
        ExerciseKind::OrderSteps => html! {
            <div class="answer-stack">
                { for exercise.options.iter().enumerate().map(|(index, option)| {
                    let ordered_values = state.ordered_values.clone();
                    let option_text = (*option).to_owned();
                    let selected = ordered_values.iter().any(|item| item == &option_text);
                    let class_name = if selected { "option-button selected" } else { "option-button" };
                    html! {
                        <button
                            class={class_name}
                            type="button"
                            disabled={selected}
                            onclick={Callback::from(move |_| {
                                let mut next = (*ordered_values).clone();
                                next.push(option_text.clone());
                                ordered_values.set(next);
                            })}
                        >
                            { exercise_option(exercise, index, state.language) }
                        </button>
                    }
                }) }
                <div class="feedback">
                    <p class="feedback-title">{ t(state.language, "current_order") }</p>
                    <p class="card-text">
                        {
                            if state.ordered_values.is_empty() {
                                t(state.language, "empty_order").to_owned()
                            } else {
                                state.ordered_values.join(" → ")
                            }
                        }
                    </p>
                    <button class="tiny-button" type="button" onclick={state.on_reset_order}>{ t(state.language, "reset_order") }</button>
                </div>
            </div>
        },
    }
}

fn draft_answer(
    exercise: &Exercise,
    selected_choice: Option<usize>,
    text_value: &str,
    ordered_values: &[String],
) -> Option<UserAnswer> {
    match exercise.kind {
        ExerciseKind::SingleChoice => selected_choice.map(UserAnswer::Choice),
        ExerciseKind::FillBlank => Some(UserAnswer::Text(text_value.to_owned())),
        ExerciseKind::CodeOutput => Some(UserAnswer::Output(text_value.to_owned())),
        ExerciseKind::OrderSteps => Some(UserAnswer::Ordered(ordered_values.to_vec())),
    }
}

fn exercise_lesson_position(exercise: &Exercise) -> Option<(&'static str, usize, usize)> {
    let lesson = lessons()
        .iter()
        .find(|lesson| lesson.id == exercise.lesson_id)?;
    let lesson_exercises = exercises_for_lesson(lesson.id);
    let index = lesson_exercises
        .iter()
        .position(|item| item.id == exercise.id)?;

    Some((lesson.title, index + 1, lesson_exercises.len()))
}

fn exercise_index_label(
    language: Language,
    lesson_title_text: &'static str,
    current: usize,
    total: usize,
) -> String {
    match language {
        Language::Zh => format!("{lesson_title_text} · 第 {current}/{total} 题"),
        Language::En => format!("{lesson_title_text} · {current}/{total}"),
    }
}

fn next_route(current_id: &str) -> Route {
    let all = exercises();
    let next = all
        .iter()
        .position(|exercise| exercise.id == current_id)
        .and_then(|index| all.get(index + 1))
        .or_else(|| all.first());

    next.map(|exercise| Route::Exercise {
        id: exercise.id.to_owned(),
    })
    .unwrap_or(Route::Learn)
}

fn render_demo_for_exercise(exercise: &Exercise, language: Language) -> Html {
    let lesson = lessons()
        .iter()
        .find(|lesson| lesson.id == exercise.lesson_id)
        .expect("exercise references a lesson");

    html! {
        <DemoBlock
            title={demo_title(lesson, language)}
            code={lesson.demo.code}
            output={lesson.demo.output}
            takeaway={demo_takeaway(lesson, language)}
            output_label={t(language, "demo_output")}
            book_url={lesson.book_url}
            book_label={t(language, "book_ref")}
            guide_title={t(language, "quick_guide")}
            goals_title={t(language, "learning_goals")}
            guide={lesson_guide(lesson, language).to_vec()}
            goals={lesson.goals.iter().enumerate().map(|(index, _)| lesson_goal(lesson, index, language)).collect::<Vec<_>>()}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct LanguageOnlyProps {
    pub language: Language,
}

#[function_component(CardsPage)]
pub fn cards_page(props: &LanguageOnlyProps) -> Html {
    let language = props.language;

    html! {
        <section class="panel full">
            <h1 class="section-title">{ t(language, "cards_title") }</h1>
            <p class="section-copy">{ t(language, "cards_copy") }</p>
            <div class="card-grid">
                { for cards().iter().map(|card| html! {
                    <article class="knowledge-card">
                        <div class="tag-row">
                            <span class="pill strong">{ card.tag }</span>
                        </div>
                        <h2 class="card-title">{ card_title(card, language) }</h2>
                        <p class="card-text">{ card_summary(card, language) }</p>
                        <pre class="code-block"><code>{ card.wrong }</code></pre>
                        <p class="card-text">{ card_fix(card, language) }</p>
                    </article>
                }) }
            </div>
        </section>
    }
}

#[function_component(StatsPage)]
pub fn stats_page(props: &ProgressPageProps) -> Html {
    let snapshot = &*props.progress.snapshot;
    let language = props.language;
    let total = exercises().len();
    let weak = snapshot.weak_lessons(lessons());

    html! {
        <>
            <section class="stats-grid">
                <StatCard label={t(language, "completion_rate")} value={percent(snapshot.completion_rate(total))} />
                <StatCard label={t(language, "accuracy")} value={percent(snapshot.correct_rate())} />
                <StatCard label={t(language, "done")} value={format!("{}/{}", snapshot.completed_exercises.len(), total)} />
                <StatCard label={t(language, "streak")} value={format!("{} {}", snapshot.streak_days(), t(language, "days"))} />
            </section>
            <section class="panel full">
                <h1 class="section-title">{ t(language, "weak_lessons") }</h1>
                <p class="section-copy">{ t(language, "weak_copy") }</p>
                <div class="weak-list">
                    {
                        if weak.is_empty() {
                            html! { <div class="empty-state">{ t(language, "weak_empty") }</div> }
                        } else {
                            html! {
                                <>
                                    { for weak.iter().map(|item| html! {
                                        <article class="lesson-card">
                                            <h2 class="lesson-title">{ item.title }</h2>
                                            <p class="lesson-summary">
                                                { format!("{} {} · {} {} {}", t(language, "accuracy"), percent(item.correct_rate), t(language, "samples"), item.attempts, t(language, "times")) }
                                            </p>
                                        </article>
                                    }) }
                                </>
                            }
                        }
                    }
                </div>
            </section>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct StatCardProps {
    label: &'static str,
    value: String,
}

#[function_component(StatCard)]
fn stat_card(props: &StatCardProps) -> Html {
    html! {
        <article class="stat-card">
            <p class="stat-label">{ props.label }</p>
            <p class="stat-value">{ &props.value }</p>
        </article>
    }
}

#[function_component(NotFoundPage)]
pub fn not_found_page(props: &LanguageOnlyProps) -> Html {
    let language = props.language;

    html! {
        <section class="panel full">
            <div class="empty-state">
                <h1>{ t(language, "not_found_title") }</h1>
                <p>{ t(language, "not_found_copy") }</p>
                <Link<Route> to={Route::Learn} classes="primary-button">{ t(language, "back_to_path") }</Link<Route>>
            </div>
        </section>
    }
}

fn percent(value: f32) -> String {
    format!("{}%", (value * 100.0).round() as u8)
}
