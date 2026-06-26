use learning_core::{
    cards, exercise_by_id, exercises, exercises_for_lesson, lessons, next_exercise_after,
    previous_exercise_before, recommend_next_exercise, recommend_next_lesson, stage_summaries,
    Exercise, ExerciseKind, Lesson, UserAnswer,
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
    exercise_option, exercise_prompt, exercise_title, lesson_goal, lesson_summary, lesson_title,
    stage_label, t, Language,
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
    let steps = exercise_reasoning_steps(exercise, language, expected);
    let trap = exercise_common_trap(exercise, language);

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
) -> Vec<String> {
    match (exercise.kind, language, expected) {
        (ExerciseKind::SingleChoice, Language::Zh, Some(answer)) => vec![
            format!("先把题目归类：这题考的是「{}」，也就是要判断哪一个选项符合当前 Rust 规则。", exercise_title(exercise, language)),
            "再回到代码里找证据：重点看类型标注、分号、所有权移动、借用范围、match 分支是否完整这些会被编译器检查的位置。".to_owned(),
            format!("最后对照参考答案「{answer}」复盘：正确选项必须能解释代码为什么成立或为什么报错，不能只凭关键词眼熟。"),
        ],
        (ExerciseKind::SingleChoice, Language::Zh, None) => vec![
            format!("先别急着选答案，先判断这题属于哪个知识点：{}。", exercise_title(exercise, language)),
            "逐行读代码，只标出会影响编译或结果的地方：类型、分号、是否 move、是否借用、分支返回类型是否一致。".to_owned(),
            "逐个排除选项：只要选项和代码证据冲突，就先排除；剩下的选项要能完整解释题干。".to_owned(),
        ],
        (ExerciseKind::FillBlank, Language::Zh, Some(answer)) => vec![
            "先看空白处在语法结构里的位置：它是在类型位置、表达式位置、模式位置，还是方法/函数调用位置。".to_owned(),
            format!("再把参考答案「{answer}」放回代码里读一遍，确认整句代码能从左到右连贯解释。"),
            "最后检查大小写、括号、冒号、分号和引用符号：填空题很多错误不是概念错，而是语法角色没对上。".to_owned(),
        ],
        (ExerciseKind::FillBlank, Language::Zh, None) => vec![
            "先确定空白处需要的是哪类东西：类型、变量名、模式、表达式、方法名，还是标点符号的一部分。".to_owned(),
            "把空白前后的代码连起来读，尤其注意 `:` 后面通常是类型，`let` 左侧通常是模式，函数体最后通常是表达式。".to_owned(),
            "写答案前先在脑中补全整行代码，如果补完后读起来不像 Rust 语法，就回到前一步重新判断空白角色。".to_owned(),
        ],
        (ExerciseKind::CodeOutput, Language::Zh, Some(answer)) => vec![
            "先从入口语句开始执行，不要跳读；遇到函数调用就进入函数体，看它到底返回了什么。".to_owned(),
            "注意 Rust 的表达式规则：没有分号的尾表达式会成为返回值，加了分号则只是语句，值会被丢弃。".to_owned(),
            format!("最后把 `println!` 或表达式结果展开成实际文本，对照参考输出「{answer}」。"),
        ],
        (ExerciseKind::CodeOutput, Language::Zh, None) => vec![
            "先找到代码真正会执行的入口，通常是最后几行的 `println!` 或变量绑定。".to_owned(),
            "遇到函数、block、match、if 表达式时，先算出它们产出的值，再继续往下代入。".to_owned(),
            "输出题要严格区分值和格式：数字、空格、换行、字符串内容都要按代码实际输出填写。".to_owned(),
        ],
        (ExerciseKind::OrderSteps, Language::Zh, Some(answer)) => vec![
            "先找必须最先发生的动作：通常是创建值、拿到引用、构造迭代器、匹配分支或准备错误处理。".to_owned(),
            "再按数据流排序：谁依赖谁，谁必须在谁之后使用，就把它放到后面。".to_owned(),
            format!("最后对照参考顺序「{answer}」，检查每一步是否都能使用上一步产生的值。"),
        ],
        (ExerciseKind::OrderSteps, Language::Zh, None) => vec![
            "先找起点：哪一步创建了后续步骤要用的值或上下文。".to_owned(),
            "再找依赖：如果某一步要使用变量、引用或结果，那么它一定排在产生这些东西的步骤之后。".to_owned(),
            "最后找收尾：打印、返回、collect、sum、unwrap_or 这类消费结果的动作通常靠后。".to_owned(),
        ],
        (ExerciseKind::SingleChoice, Language::En, Some(answer)) => vec![
            format!("Classify the question first: it tests '{}', so the right option must match that Rust rule.", exercise_title(exercise, language)),
            "Go back to the code and look for compiler-checked evidence: types, semicolons, moves, borrow ranges, and complete match arms.".to_owned(),
            format!("Then compare with the expected answer '{answer}' and make sure it explains the code, not just a familiar keyword."),
        ],
        (ExerciseKind::SingleChoice, Language::En, None) => vec![
            format!("First identify the topic: {}.", exercise_title(exercise, language)),
            "Read the code line by line and mark only the parts that affect compilation or the result.".to_owned(),
            "Eliminate options that contradict the code evidence; the remaining one should explain the prompt fully.".to_owned(),
        ],
        (ExerciseKind::FillBlank, Language::En, Some(answer)) => vec![
            "Identify what syntactic role the blank has: type, expression, pattern, or call target.".to_owned(),
            format!("Put the expected answer '{answer}' back into the code and read the whole line from left to right."),
            "Check spelling, punctuation, references, colons, and semicolons; fill blanks often fail because the role is mismatched.".to_owned(),
        ],
        (ExerciseKind::FillBlank, Language::En, None) => vec![
            "First decide what kind of thing the blank needs: a type, variable, pattern, expression, method, or punctuation.".to_owned(),
            "Read the tokens before and after the blank; `:` usually asks for a type, while the left side of `let` is a pattern.".to_owned(),
            "Mentally complete the whole line before typing. If it does not read like Rust syntax, revisit the role.".to_owned(),
        ],
        (ExerciseKind::CodeOutput, Language::En, Some(answer)) => vec![
            "Start at the actual entry statement and step into function calls instead of guessing from the function name.".to_owned(),
            "Remember expression rules: a final expression without a semicolon returns a value; a semicolon turns it into a statement.".to_owned(),
            format!("Expand the final `println!` or expression into concrete text and compare it with '{answer}'."),
        ],
        (ExerciseKind::CodeOutput, Language::En, None) => vec![
            "Find the statement that actually produces output, usually `println!` near the end.".to_owned(),
            "Evaluate functions, blocks, match, and if expressions before substituting their values into the output.".to_owned(),
            "Be exact about formatting: numbers, spaces, newlines, and string contents all matter.".to_owned(),
        ],
        (ExerciseKind::OrderSteps, Language::En, Some(answer)) => vec![
            "Find the action that must happen first: creating a value, borrowing it, building an iterator, or preparing error handling.".to_owned(),
            "Sort by data dependency: if a step uses something, it must come after the step that creates it.".to_owned(),
            format!("Compare with the expected order '{answer}' and verify that every step can use the previous result."),
        ],
        (ExerciseKind::OrderSteps, Language::En, None) => vec![
            "Find the starting step that creates the value or context used later.".to_owned(),
            "Then track dependencies: any step using a variable, reference, or result must come after it is produced.".to_owned(),
            "Finish with consuming actions such as printing, returning, collect, sum, or unwrap_or.".to_owned(),
        ],
    }
}

fn exercise_common_trap(exercise: &Exercise, language: Language) -> String {
    let difficulty_note = match (exercise.level(), language) {
        (1, Language::Zh) => "基础题先看语法位置，不要把其他章节的复杂规则带进来。",
        (2, Language::Zh) => "进阶题通常会同时考两个小规则，必须把代码证据逐条对上。",
        (_, Language::Zh) => "挑战题常把正确写法和相似但错误的写法放在一起，尤其要检查所有权、生命周期或返回类型。",
        (1, Language::En) => "For basic questions, focus on syntax position before importing rules from later chapters.",
        (2, Language::En) => "Practice questions often combine two small rules, so match each claim to code evidence.",
        (_, Language::En) => "Challenge questions often place a correct pattern next to a similar wrong one; inspect ownership, lifetimes, or return types carefully.",
    };

    let kind_note = match (exercise.kind, language) {
        (ExerciseKind::SingleChoice, Language::Zh) => "不要只看选项里的熟悉词；正确选项必须能解释这段代码的具体行为。",
        (ExerciseKind::FillBlank, Language::Zh) => "不要只填“意思对”的词；Rust 还要求它在当前位置是合法语法。",
        (ExerciseKind::CodeOutput, Language::Zh) => "不要凭直觉写结果；要按执行顺序算到最后，尤其注意分号和格式化输出。",
        (ExerciseKind::OrderSteps, Language::Zh) => "不要按文字顺眼排序；要按变量和引用的依赖关系排序。",
        (ExerciseKind::SingleChoice, Language::En) => "Do not pick by familiar wording; the right option must explain this exact code.",
        (ExerciseKind::FillBlank, Language::En) => "Do not type a word that only 'means' the right thing; it must be valid Rust at that position.",
        (ExerciseKind::CodeOutput, Language::En) => "Do not guess the result; evaluate in execution order and watch semicolons and formatting.",
        (ExerciseKind::OrderSteps, Language::En) => "Do not sort by how the text reads; sort by variable and reference dependencies.",
    };

    format!("{difficulty_note}{kind_note}")
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

    html! {
        <>
            <DemoBlock
                title={demo_title(lesson, language)}
                source_path={lesson.demo.source_path}
                source_lines={lesson.demo.source_lines}
                source_role={demo_source_role(lesson, language)}
                book_rule={demo_book_rule(lesson, language)}
                source_url={lesson.demo.source_url}
                source_label={t(language, "source_ref")}
                code={lesson.demo.code}
                output={lesson.demo.output}
                takeaway={demo_takeaway(lesson, language)}
                output_label={t(language, "demo_output")}
                book_url={lesson.book_url}
                book_label={t(language, "book_ref")}
                guide_title={t(language, "quick_guide")}
                goals_title={t(language, "learning_goals")}
                source_anchor_label={t(language, "source_anchor")}
                source_role_label={t(language, "source_role")}
                source_rule_label={t(language, "source_rule")}
                guide={enriched_lesson_guide(exercise, lesson, language)}
                goals={enriched_lesson_goals(exercise, lesson, language)}
            />
        </>
    }
}

fn enriched_lesson_guide(exercise: &Exercise, lesson: &Lesson, language: Language) -> Vec<String> {
    vec![
        source_reading_focus(lesson, language),
        exercise_book_link(exercise, lesson, language),
        exercise_answer_hint(exercise, language).to_owned(),
    ]
}

fn enriched_lesson_goals(exercise: &Exercise, lesson: &Lesson, language: Language) -> Vec<String> {
    let mut goals = lesson
        .goals
        .iter()
        .enumerate()
        .map(|(index, _)| lesson_goal(lesson, index, language).to_owned())
        .collect::<Vec<_>>();
    goals.push(exercise_source_mapping(exercise, lesson, language));
    goals.push(format!(
        "{}：{}",
        t(language, "answer_hint"),
        exercise_hint(exercise, language)
    ));
    goals
}

fn source_reading_focus(lesson: &Lesson, language: Language) -> String {
    match language {
        Language::Zh => format!(
            "先把源码定位到 `{}` {}，理解它在 ripgrep 里的职责，再回到题目判断具体 Rust 规则。",
            lesson.demo.source_path, lesson.demo.source_lines
        ),
        Language::En => format!(
            "Start at `{}` {}, understand its role in ripgrep, then answer by applying the mapped Rust rule.",
            lesson.demo.source_path, lesson.demo.source_lines
        ),
    }
}

fn demo_source_role(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.demo.source_role;
    }

    match lesson.id {
        "syntax-basics" => {
            "ripgrep's executable entry point wires flag parsing, search execution, and error exit codes together."
        }
        "control-flow" => {
            "Selects the execution path for search, file listing, type listing, or shell completion generation."
        }
        "data-functions" => {
            "Reads pattern lines from an input reader and collects searchable patterns into `Vec<String>`."
        }
        "ownership" => {
            "Closes a child process stdout pipe and waits for the external decompressor process."
        }
        "slices" => {
            "Validates raw pattern bytes as UTF-8 before treating them as a searchable text pattern."
        }
        "borrowing" => {
            "Unifies output backends under `io::Write` so search results can be written safely."
        }
        "structs-enums" => {
            "Stores decompression command configuration and exposes builder methods for matcher construction."
        }
        "result-option" => {
            "Looks up a decompressor command for a path and returns `None` when no match is found."
        }
        "collections" => {
            "Parses config lines and collects arguments and parse errors into separate `Vec` values."
        }
        "iterators-traits" => {
            "Turns directory walk results into searchable haystacks before running the search loop."
        }
        "generics-traits" => {
            "Wraps a candidate path so globset can test it while preserving borrowed path data."
        }
        "concurrency" => {
            "Lets parallel workers send paths to one printer thread through a channel."
        }
        _ => lesson.demo.source_role,
    }
}

fn demo_book_rule(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.demo.book_rule;
    }

    match lesson.id {
        "syntax-basics" => {
            "`match` is an expression, arm tails can become return values, and macros perform side effects."
        }
        "control-flow" => "`match` must cover all modes, while guards refine a matched pattern.",
        "data-functions" => {
            "Function signatures state input, output, and error boundaries; a tail expression returns the final `Result`."
        }
        "ownership" => {
            "Moves transfer resource ownership; `Option::take` moves a value out and leaves `None` behind."
        }
        "slices" => {
            "Slices are borrowed views; a returned `&str` must stay tied to the input byte slice."
        }
        "borrowing" => {
            "`&mut self` gives exclusive mutation of writer state, while `&[u8]` only borrows the buffer for reading."
        }
        "structs-enums" => {
            "Structs hold related fields, impl blocks define behavior, and `&mut self` methods can support chaining."
        }
        "result-option" => {
            "`Option` models absence and `Result` models recoverable failure; keep those meanings separate."
        }
        "collections" => {
            "`Vec` is growable; `push` stores owned values and mutating the collection requires `mut`."
        }
        "iterators-traits" => {
            "Iterator adapters are lazy; `filter_map` describes a transformation and `for` consumes it."
        }
        "generics-traits" => {
            "Trait bounds describe accepted capabilities, and lifetimes relate borrowed output to borrowed input."
        }
        "concurrency" => {
            "Channels transfer ownership between threads, and `thread::spawn` closures often use `move`."
        }
        _ => lesson.demo.book_rule,
    }
}

fn exercise_source_mapping(exercise: &Exercise, lesson: &Lesson, language: Language) -> String {
    match language {
        Language::Zh => format!(
            "{}：这道题把「{}」落到 `{}` 的源码片段里，不是孤立背概念。",
            t(language, "exercise_link"),
            exercise_title(exercise, language),
            lesson.demo.source_path
        ),
        Language::En => format!(
            "{}: '{}' is mapped back to `{}` instead of being a detached concept drill.",
            t(language, "exercise_link"),
            exercise_title(exercise, language),
            lesson.demo.source_path
        ),
    }
}

fn exercise_book_link(exercise: &Exercise, lesson: &Lesson, language: Language) -> String {
    match (exercise.id, language) {
        ("syntax-let-mut", Language::Zh) => "本题空白在 `let ____ count` 的绑定位置。Rust Book 说绑定默认不可变，所以后面的 `count += 1` 要合法，绑定处必须声明可变。".to_owned(),
        ("syntax-let-mut", Language::En) => "The blank sits in `let ____ count`. The Book says bindings are immutable by default, so `count += 1` only works if the binding is marked mutable.".to_owned(),
        ("enum-if-let-method", Language::Zh) => "题干给出 `msg` 的类型是 `Message`，`Quit` 是这个 enum 的变体。匹配时要写完整路径 `Message::Quit`，不是单独写 `Quit`。".to_owned(),
        ("enum-if-let-method", Language::En) => "The prompt says `msg` has type `Message`, and `Quit` is a variant of that enum. Match it with the full path `Message::Quit`, not bare `Quit`.".to_owned(),
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
                Language::Zh => "对应到本题，只保留能被题干代码直接证明的规则；如果一句话解释不出“因为哪里，所以答案是什么”，先回题干找证据。".to_owned(),
                Language::En => "For this exercise, keep only rules proven by the prompt code. If you cannot say 'because this code shows X, the answer is Y', inspect the prompt again.".to_owned(),
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
