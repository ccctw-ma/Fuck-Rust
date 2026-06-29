use learning_core::{
    cards, exercise_by_id, exercises, exercises_for_lesson, lessons, next_exercise_after,
    previous_exercise_before, recommend_next_exercise, recommend_next_lesson, stage_summaries,
    Exercise, ExerciseKind, Lesson, SourceContext, UserAnswer,
};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::TargetCast;
use yew_router::prelude::*;

use crate::app::{ProgressHandle, Route};
use crate::components::{DemoBlock, MetricCard, StageCard};
use crate::i18n::{
    answer_summary, card_fix, card_summary, card_title, exercise_explanation, exercise_hint,
    exercise_kind_label, exercise_level_label, exercise_option, exercise_prompt, exercise_title,
    lesson_goal, lesson_summary, lesson_title, stage_label, t, Language,
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
                                <a class="pill source-pill" href={lesson.demo.source_url} target="_blank" rel="noreferrer">{ t(language, "source_ref") }</a>
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
                    <Link<Route> to={previous_route(exercise.id)} classes="ghost-button">{ t(language, "previous_exercise") }</Link<Route>>
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
                                { render_exercise_support(exercise, language, Some(expected.as_str())) }
                            </div>
                        }
                    } else {
                        html! {
                            <div class="feedback">
                                <p class="feedback-title">{ t(language, "hint") }</p>
                                <p class="card-text">{ exercise_hint(exercise, language) }</p>
                                { render_exercise_support(exercise, language, None) }
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

fn render_exercise_support(
    exercise: &Exercise,
    language: Language,
    expected: Option<&str>,
) -> Html {
    let source = exercise.source_context();
    let steps = exercise_reasoning_steps(exercise, language, expected, source);
    let trap = exercise_common_trap(exercise, language, source);

    html! {
        <div class="solution-guide">
            <p class="solution-title">{ t(language, "how_to_solve") }</p>
            <ol class="solution-steps">
                { for steps.into_iter().map(|step| html! { <li>{ step }</li> }) }
            </ol>
            <div class="trap-card">
                <p class="solution-title">{ t(language, "common_trap") }</p>
                <p class="card-text">{ trap }</p>
            </div>
        </div>
    }
}

fn exercise_reasoning_steps(
    exercise: &Exercise,
    language: Language,
    expected: Option<&str>,
    source: SourceContext,
) -> Vec<String> {
    let expected_copy = expected
        .map(|answer| match language {
            Language::Zh => format!("参考答案是「{answer}」。"),
            Language::En => format!("Expected answer: {answer}."),
        })
        .unwrap_or_else(|| match language {
            Language::Zh => "提交前先用下面的证据链自查答案。".to_owned(),
            Language::En => {
                "Before submitting, check your answer against this evidence chain.".to_owned()
            }
        });
    let code_focus = exercise_code_focus(exercise, language);

    match language {
        Language::Zh => vec![
            format!(
                "源码证据：先看 `{}` {}。这里的关键规则是：{}",
                source.source_path, source.source_lines, source.book_rule
            ),
            format!(
                "题目代码证据：{}；这对应到题目「{}」里的空白、选项或输出。",
                code_focus,
                exercise_title(exercise, language)
            ),
            format!(
                "{}再用题目解释复核：{}",
                expected_copy,
                exercise_explanation(exercise, language)
            ),
        ],
        Language::En => vec![
            format!(
                "Source evidence: inspect `{}` {}. The relevant rule is: {}",
                source.source_path, source.source_lines, source.book_rule
            ),
            format!(
                "Prompt evidence: {}; this maps directly to the blank, option, or output in '{}'.",
                code_focus,
                exercise_title(exercise, language)
            ),
            format!(
                "{}Then verify with the exercise explanation: {}",
                expected_copy,
                exercise_explanation(exercise, language)
            ),
        ],
    }
}

fn exercise_common_trap(exercise: &Exercise, language: Language, source: SourceContext) -> String {
    match language {
        Language::Zh => format!(
            "不要把这题当成脱离源码的概念题。必须同时满足三件事：题目代码 `{}` 读得通、参考答案能解释 `{} {}` 的源码规则、解释能对应到“{}”。提示只作为辅助，最终以源码和题干证据为准。",
            compact_code(exercise.code),
            source.source_path,
            source.source_lines,
            exercise_explanation(exercise, language)
        ),
        Language::En => format!(
            "Do not treat this as a detached concept quiz. The answer must make the prompt code `{}` read correctly, explain the rule in `{}` {}, and match this explanation: {}",
            compact_code(exercise.code),
            source.source_path,
            source.source_lines,
            exercise_explanation(exercise, language)
        ),
    }
}

fn exercise_code_focus(exercise: &Exercise, language: Language) -> String {
    let compact = compact_code(exercise.code);
    if compact.is_empty() {
        return match language {
            Language::Zh => format!(
                "这题没有代码块，证据主要在排序/选项文字中：{}",
                exercise.prompt
            ),
            Language::En => format!(
                "This exercise has no code block; the evidence is in the ordering/options: {}",
                exercise.prompt
            ),
        };
    }

    match language {
        Language::Zh => format!("题干代码是 `{compact}`"),
        Language::En => format!("the prompt code is `{compact}`"),
    }
}

fn compact_code(code: &str) -> String {
    code.split_whitespace().collect::<Vec<_>>().join(" ")
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
    next_exercise_after(current_id)
        .map(|exercise| Route::Exercise {
            id: exercise.id.to_owned(),
        })
        .unwrap_or(Route::Learn)
}

fn previous_route(current_id: &str) -> Route {
    previous_exercise_before(current_id)
        .map(|exercise| Route::Exercise {
            id: exercise.id.to_owned(),
        })
        .unwrap_or(Route::Learn)
}

fn render_demo_for_exercise(exercise: &Exercise, language: Language) -> Html {
    let lesson = lessons()
        .iter()
        .find(|lesson| lesson.id == exercise.lesson_id)
        .expect("exercise references a lesson");
    let source = exercise.source_context();

    html! {
        <>
            <DemoBlock
                title={source.title}
                source_path={source.source_path}
                source_lines={source.source_lines}
                source_role={source.source_role}
                book_rule={source.book_rule}
                source_url={source.source_url}
                source_label={t(language, "source_ref")}
                code={source.code}
                output={source.output}
                takeaway={source.takeaway}
                output_label={t(language, "demo_output")}
                book_url={lesson.book_url}
                book_label={t(language, "book_ref")}
                guide_title={t(language, "quick_guide")}
                goals_title={t(language, "learning_goals")}
                source_anchor_label={t(language, "source_anchor")}
                source_role_label={t(language, "source_role")}
                source_rule_label={t(language, "source_rule")}
                guide={enriched_lesson_guide(exercise, lesson, language, source)}
                goals={enriched_lesson_goals(exercise, language, source)}
            />
        </>
    }
}

fn enriched_lesson_guide(
    exercise: &Exercise,
    lesson: &Lesson,
    language: Language,
    source: SourceContext,
) -> Vec<String> {
    vec![
        exercise_knowledge_point(exercise, language),
        source_reading_focus(source, language),
        exercise_book_link(exercise, lesson, source, language),
        exercise_answer_hint(exercise, language).to_owned(),
    ]
}

fn enriched_lesson_goals(
    exercise: &Exercise,
    language: Language,
    source: SourceContext,
) -> Vec<String> {
    vec![
        exercise_source_mapping(exercise, source, language),
        exercise_source_evidence(source, language),
        format!(
            "{}：{}",
            t(language, "answer_hint"),
            exercise_hint(exercise, language)
        ),
    ]
}

fn source_reading_focus(source: SourceContext, language: Language) -> String {
    match language {
        Language::Zh => format!(
            "先把源码定位到 `{}` {}，理解它如何使用本题知识点，再回到题目判断空白或选项。",
            source.source_path, source.source_lines
        ),
        Language::En => format!(
            "Start at `{}` {}, see how this source uses the concept, then answer the blank or option.",
            source.source_path, source.source_lines
        ),
    }
}

fn exercise_source_mapping(
    exercise: &Exercise,
    source: SourceContext,
    language: Language,
) -> String {
    match language {
        Language::Zh => format!(
            "{}：这道题把「{}」落到 `{}` 的源码片段里，不是孤立背概念。",
            t(language, "exercise_link"),
            exercise_title(exercise, language),
            source.source_path
        ),
        Language::En => format!(
            "{}: '{}' is mapped back to `{}` instead of being a detached concept drill.",
            t(language, "exercise_link"),
            exercise_title(exercise, language),
            source.source_path
        ),
    }
}

fn exercise_source_evidence(source: SourceContext, language: Language) -> String {
    match language {
        Language::Zh => format!(
            "{}：`{}` {} 里的关键代码就是本题判断依据，不再混用整课背景。",
            t(language, "source_question"),
            source.source_path,
            source.source_lines
        ),
        Language::En => format!(
            "{}: `{}` {} is the evidence for this question, not a generic lesson-level backdrop.",
            t(language, "source_question"),
            source.source_path,
            source.source_lines
        ),
    }
}

fn exercise_knowledge_point(exercise: &Exercise, language: Language) -> String {
    match (exercise.id, language) {
        ("impl-trait-param", Language::Zh) => {
            "知识点：`impl Trait` 是函数参数里的简写，意思是“这个参数可以是任意实现某个 trait 的具体类型”。本题问的不是 Candidate 生命周期，而是 `Display` 这种格式化能力。".to_owned()
        }
        ("impl-trait-param", Language::En) => {
            "Concept: `impl Trait` in parameter position means this argument can be any concrete type implementing that trait. This exercise is about the Display formatting capability, not Candidate lifetimes.".to_owned()
        }
        ("trait-bound-display" | "where-clause", Language::Zh) => {
            "知识点：泛型参数不是“什么都能做”。函数体里用了 `{}`、`to_string()` 或用户可读格式化，就要声明 `Display` trait bound。".to_owned()
        }
        ("trait-bound-display" | "where-clause", Language::En) => {
            "Concept: generic parameters are not magic. If the body uses `{}`, `to_string()`, or user-facing formatting, it needs a `Display` bound.".to_owned()
        }
        ("derive-debug-bound", Language::Zh) => {
            "知识点：`Display` 面向用户可读输出，`Debug` 面向开发调试输出。看到 `{:?}` 时，优先想到 `Debug`。".to_owned()
        }
        ("derive-debug-bound", Language::En) => {
            "Concept: `Display` is for user-facing output, while `Debug` is for developer diagnostics. `{:?}` points to `Debug`.".to_owned()
        }
        ("generic-largest", Language::Zh) => {
            "知识点：泛型约束要来自函数体实际使用的能力。这里源码调用 `path.as_ref()`，所以核心约束是 `AsRef<Path>`。".to_owned()
        }
        ("generic-largest", Language::En) => {
            "Concept: a generic bound should come from the capability used in the body. Here the source calls `path.as_ref()`, so the key bound is `AsRef<Path>`.".to_owned()
        }
        (_, Language::Zh) => format!(
            "知识点：{}。先看题目代码真正用了什么语法或能力，再回源码确认同一规则如何出现在 ripgrep 中。",
            exercise_title(exercise, language)
        ),
        (_, Language::En) => format!(
            "Concept: {}. First identify the syntax or capability used in the prompt, then connect it to the matching ripgrep source.",
            exercise_title(exercise, language)
        ),
    }
}

fn exercise_book_link(
    exercise: &Exercise,
    lesson: &Lesson,
    source: SourceContext,
    language: Language,
) -> String {
    match (exercise.id, language) {
        ("syntax-let-mut", Language::Zh) => "本题空白在 `let ____ count` 的绑定位置。Rust Book 说绑定默认不可变，所以后面的 `count += 1` 要合法，绑定处必须声明可变。".to_owned(),
        ("syntax-let-mut", Language::En) => "The blank sits in `let ____ count`. The Book says bindings are immutable by default, so `count += 1` only works if the binding is marked mutable.".to_owned(),
        ("enum-if-let-method", Language::Zh) => "题干给出 `msg` 的类型是 `Message`，`Quit` 是这个 enum 的变体。匹配时要写完整路径 `Message::Quit`，不是单独写 `Quit`。".to_owned(),
        ("enum-if-let-method", Language::En) => "The prompt says `msg` has type `Message`, and `Quit` is a variant of that enum. Match it with the full path `Message::Quit`, not bare `Quit`.".to_owned(),
        ("impl-trait-param", Language::Zh) => "本题空白在 `impl ____` 的 trait 位置。对照 ripgrep 的 `error_message<T: std::fmt::Display>`：参数要能转成用户可读字符串，所以答案是 `Display`。".to_owned(),
        ("impl-trait-param", Language::En) => "The blank is the trait in `impl ____`. Compare ripgrep's `error_message<T: std::fmt::Display>`: the value must become user-facing text, so the answer is `Display`.".to_owned(),
        ("trait-bound-display" | "where-clause", Language::Zh) => "本题要看函数体：只要代码使用 `{}` 或 `to_string()` 输出泛型值，T 就必须提供 `Display` 能力。".to_owned(),
        ("trait-bound-display" | "where-clause", Language::En) => "Look at the body: if generic T is printed with `{}` or converted with `to_string()`, T must provide `Display`.".to_owned(),
        _ => match (lesson.id, exercise.kind, language) {
            ("syntax-basics", _, Language::Zh) => "对应到本题，先判断空白或输出来自绑定、表达式、语句还是模式位置；Rust 的位置规则比“词看起来像什么”更重要。".to_owned(),
            ("control-flow", _, Language::Zh) => "对应到本题，盯住 `if`、`match` 或 `if let` 的条件和分支：Rust Book 要求分支覆盖明确，并且作为表达式时产出同一种类型。".to_owned(),
            ("data-functions", _, Language::Zh) => "对应到本题，把类型标注、函数参数、`->` 返回类型和最后一行尾表达式圈出来；答案必须和这些签名证据一致。".to_owned(),
            ("ownership", _, Language::Zh) => "对应到本题，先标出哪个变量拥有值，再看题目有没有赋值、传参或函数返回导致所有权转移。".to_owned(),
            ("slices", _, Language::Zh) => "对应到本题，切片答案要能说明它借用了原数据的哪一段；不要把切片当成新分配出来的集合。".to_owned(),
            ("borrowing", _, Language::Zh) => "对应到本题，先分清这里需要读借用还是写借用，再检查同一时间有没有多个读或一个独占写的冲突。".to_owned(),
            ("structs-enums", _, Language::Zh) => "对应到本题，先看题干是在字段、方法接收者还是 enum 变体路径上设问；答案要贴合这个具体语法位置。".to_owned(),
            ("result-option", _, Language::Zh) => "对应到本题，先确认代码处理的是 `Some/None` 还是 `Ok/Err`；`?` 题还要看失败时会不会提前返回。".to_owned(),
            ("collections", _, Language::Zh) => "对应到本题，看集合操作是在读、插入、更新还是转移 key/value 所有权；不同 API 对 `mut` 和 move 的要求不同。".to_owned(),
            ("iterators-traits", _, Language::Zh) => "对应到本题，先判断迭代器链只是描述步骤，还是已经被 `collect`、`sum` 或 `for` 消费执行。".to_owned(),
            ("generics-traits", _, Language::Zh) => "对应到本题，找出函数体实际用到了什么能力；泛型参数只有写出对应 trait bound 才能使用这些能力。".to_owned(),
            ("concurrency", _, Language::Zh) => "对应到本题，先看值是否要跨线程或跨 channel 移动；线程闭包常用 `move` 是为了拿到可安全使用的所有权。".to_owned(),
            ("syntax-basics", _, Language::En) => "For this exercise, first decide whether the blank or output comes from a binding, expression, statement, or pattern position. Rust's position rules matter more than familiar words.".to_owned(),
            ("control-flow", _, Language::En) => "For this exercise, inspect the `if`, `match`, or `if let` condition and arms. The Book expects explicit coverage and one output type when used as an expression.".to_owned(),
            ("data-functions", _, Language::En) => "For this exercise, circle the type annotation, parameters, `->` return type, and final tail expression. The answer must match those signature clues.".to_owned(),
            ("ownership", _, Language::En) => "For this exercise, mark which variable owns the value, then check whether assignment, parameters, or returns move that ownership.".to_owned(),
            ("slices", _, Language::En) => "For this exercise, explain which part of the original data the slice borrows. Do not treat a slice as a newly allocated collection.".to_owned(),
            ("borrowing", _, Language::En) => "For this exercise, decide whether the code needs read borrowing or write borrowing, then check for conflicts between many readers and one exclusive writer.".to_owned(),
            ("structs-enums", _, Language::En) => "For this exercise, identify whether the prompt is about fields, method receivers, or enum variant paths. The answer must fit that exact syntax position.".to_owned(),
            ("result-option", _, Language::En) => "For this exercise, confirm whether the code handles `Some/None` or `Ok/Err`; with `?`, also check whether failure returns early.".to_owned(),
            ("collections", _, Language::En) => "For this exercise, check whether the collection operation reads, inserts, updates, or moves key/value ownership. Each API has different `mut` and move rules.".to_owned(),
            ("iterators-traits", _, Language::En) => "For this exercise, decide whether the iterator chain only describes work or is consumed by `collect`, `sum`, or `for`.".to_owned(),
            ("generics-traits", _, Language::En) => "For this exercise, find the capability used inside the function body. A generic parameter can only use it when the matching trait bound is declared.".to_owned(),
            ("concurrency", _, Language::En) => "For this exercise, check whether the value crosses a thread or channel boundary. Thread closures often use `move` to own data safely.".to_owned(),
            _ => match language {
                Language::Zh => format!(
                    "对应到本题，只保留能被题干和 `{}` 直接证明的规则；如果一句话解释不出“因为哪里，所以答案是什么”，先回题干找证据。",
                    source.source_path
                ),
                Language::En => format!(
                    "For this exercise, keep only rules proven by the prompt and `{}`. If you cannot say 'because this code shows X, the answer is Y', inspect the prompt again.",
                    source.source_path
                ),
            },
        },
    }
}

fn exercise_answer_hint(exercise: &Exercise, language: Language) -> &'static str {
    match (exercise.kind, language) {
        (ExerciseKind::FillBlank, Language::Zh) => "填完后整行必须像真实 Rust 代码一样能读通；大小写、`::`、`&`、`,` 都算答案的一部分。",
        (ExerciseKind::SingleChoice, Language::Zh) => "只选能被题干代码直接证明的那一项；熟悉但没证据的说法先排除。",
        (ExerciseKind::CodeOutput, Language::Zh) => "只写实际输出，按执行顺序推到最后；空格、换行和调试格式也要一致。",
        (ExerciseKind::OrderSteps, Language::Zh) => "按变量创建、借用、修改、消费的依赖排序；不能让后一步使用还不存在或已 move 的值。",
        (ExerciseKind::FillBlank, Language::En) => "After filling it in, the whole line must read like real Rust. Case, `::`, `&`, and punctuation are part of the answer.",
        (ExerciseKind::SingleChoice, Language::En) => "Choose only the option directly proven by the prompt code. Discard familiar claims that lack evidence.",
        (ExerciseKind::CodeOutput, Language::En) => "Type only the actual output after evaluating execution order. Spaces, newlines, and debug formatting matter.",
        (ExerciseKind::OrderSteps, Language::En) => "Order by creation, borrowing, mutation, and consumption dependencies. Later steps cannot use values that do not exist or were moved.",
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
