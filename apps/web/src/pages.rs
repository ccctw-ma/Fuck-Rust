use learning_core::{
    cards, exercise_by_id, exercises, exercises_for_lesson, lessons, next_exercise_after,
    previous_exercise_before, recommend_next_exercise, recommend_next_lesson, stage_summaries,
    Exercise, ExerciseDifficulty, ExerciseKind, Lesson, UserAnswer,
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
            { render_pre_question_guide(exercise, lesson, language) }
        </>
    }
}

fn render_pre_question_guide(exercise: &Exercise, lesson: &Lesson, language: Language) -> Html {
    let mental_model = lesson_mental_model(lesson, language);
    let anchors = exercise_syntax_anchors(exercise, language);
    let checklist = exercise_pre_answer_checklist(exercise, language);

    html! {
        <aside class="primer-panel" aria-label={t(language, "primer_title")}>
            <div class="demo-heading">
                <p class="demo-title">{ t(language, "primer_title") }</p>
                <span class="pill strong">{ exercise_level_label(exercise, language) }</span>
            </div>
            <p class="primer-lead">{ exercise_focus_copy(exercise, lesson, language) }</p>
            <div class="primer-grid">
                <div class="guide-list primer-card">
                    <p class="guide-title">{ t(language, "concept_model") }</p>
                    { for mental_model.into_iter().map(|copy| html! { <p class="guide-copy">{ copy }</p> }) }
                </div>
                <div class="guide-list primer-card">
                    <p class="guide-title">{ t(language, "syntax_anchor") }</p>
                    { for anchors.into_iter().enumerate().map(|(index, copy)| html! {
                        <p class="guide-step"><span>{ format!("{:02}", index + 1) }</span>{ copy }</p>
                    }) }
                </div>
            </div>
            <div class="guide-list primer-card checklist-card">
                <p class="guide-title">{ t(language, "before_answer") }</p>
                { for checklist.into_iter().enumerate().map(|(index, copy)| html! {
                    <p class="guide-step"><span>{ format!("{:02}", index + 1) }</span>{ copy }</p>
                }) }
            </div>
        </aside>
    }
}

fn exercise_focus_copy(exercise: &Exercise, lesson: &Lesson, language: Language) -> String {
    match language {
        Language::Zh => format!(
            "这道题属于「{}」模块，当前聚焦「{}」。先把下面三块读完，再看题干，目标是知道题目在考哪条 Rust 规则，而不是凭感觉猜答案。",
            lesson_title(lesson, language),
            exercise_title(exercise, language)
        ),
        Language::En => format!(
            "This exercise belongs to '{}', focusing on '{}'. Read these three blocks first so you know which Rust rule is being tested before answering.",
            lesson_title(lesson, language),
            exercise_title(exercise, language)
        ),
    }
}

fn lesson_mental_model(lesson: &Lesson, language: Language) -> Vec<String> {
    match (lesson.id, language) {
        ("syntax-basics", Language::Zh) => vec![
            "Rust 会先判断代码片段处在声明、表达式、语句还是模式位置；位置错了，词义再接近也不能编译。".to_owned(),
            "`let` 默认不可变；`mut` 只允许同一个绑定改值，不会改变静态类型规则。".to_owned(),
            "表达式会产生值，语句只是执行动作；分号通常会把表达式的值丢掉。".to_owned(),
        ],
        ("control-flow", Language::Zh) => vec![
            "`if` 和 `match` 不只是流程控制，也可以直接产出一个值。".to_owned(),
            "`match` 的核心是穷尽：每一种可能输入都要有分支处理。".to_owned(),
            "`if let` 是只关心一个模式的简写；它不会帮你检查其他情况是否被处理。".to_owned(),
        ],
        ("data-functions", Language::Zh) => vec![
            "元组按位置组织固定数量的值，可以混合类型；数组 `[T; N]` 要求元素类型相同，长度 N 也是类型的一部分。".to_owned(),
            "函数签名是契约：参数类型必须写明，返回类型写在 `->` 后面。".to_owned(),
            "函数体最后一个无分号表达式就是返回值；`return` 可以提前返回，但 Rust 更常用尾表达式。".to_owned(),
        ],
        ("ownership", Language::Zh) => vec![
            "所有权回答的是“谁负责释放这块数据”；同一时刻一个值只有一个 owner。".to_owned(),
            "`String`、`Vec` 这类拥有堆数据的类型赋值时通常 move；整数、bool 等 Copy 类型赋值后原变量仍可用。".to_owned(),
            "如果只是读取，用引用借用；如果确实需要两份拥有的数据，再显式 `clone`。".to_owned(),
        ],
        ("slices", Language::Zh) => vec![
            "切片是借来的窗口，不是新集合；它指向原数据的一段连续区域。".to_owned(),
            "字符串切片范围按字节算，并且必须落在 UTF-8 字符边界上。".to_owned(),
            "返回 `&str` 能把结果和原字符串的借用关系绑定起来，避免旧索引失效。".to_owned(),
        ],
        ("borrowing", Language::Zh) => vec![
            "借用让代码临时访问值而不取得所有权：`&T` 读，`&mut T` 写。".to_owned(),
            "同一时间要么多个共享读，要么一个独占写；不能读写借用同时活跃。".to_owned(),
            "借用通常持续到最后一次使用；缩短使用范围就能释放借用。".to_owned(),
        ],
        ("structs-enums", Language::Zh) => vec![
            "struct 用字段名组织总是一起出现的数据，字段名让代码比元组更自解释。".to_owned(),
            "方法接收者决定权限：`&self` 只能读，`&mut self` 能改，`self` 会消费值。".to_owned(),
            "enum 表示有限状态集合；写变体要带枚举名，例如 `Message::Quit`，因为 `Quit` 属于 `Message`。".to_owned(),
        ],
        ("result-option", Language::Zh) => vec![
            "`Option<T>` 表示可能没有值，用 `Some(value)` 和 `None` 代替 null。".to_owned(),
            "`Result<T, E>` 表示可能失败，成功是 `Ok(T)`，失败是 `Err(E)`。".to_owned(),
            "`?` 不是捕获错误，而是在失败时提前返回；成功时拆出内部值继续执行。".to_owned(),
        ],
        ("collections", Language::Zh) => vec![
            "集合管理一组值：`Vec` 保持顺序，`String` 管 UTF-8 文本，`HashMap` 按 key 查 value。".to_owned(),
            "修改集合本身通常需要 `mut`，把拥有所有权的 key/value 放进集合会发生 move。".to_owned(),
            "`entry` API 把“存在就更新，不存在就插入”合并成一条安全路径。".to_owned(),
        ],
        ("iterators-traits", Language::Zh) => vec![
            "迭代器链像流水线：`map/filter` 只是描述步骤，`collect/sum/for` 才真正执行。".to_owned(),
            "`iter()` 借用元素，`into_iter()` 倾向于交出元素所有权；先判断后续还需不需要原集合。".to_owned(),
            "trait 是能力约束：泛型代码想打印、比较或相加，就必须声明对应 trait bound。".to_owned(),
        ],
        ("generics-traits", Language::Zh) => vec![
            "泛型把具体类型换成参数，但函数体里用到的能力必须通过 trait bound 说明。".to_owned(),
            "`where` 子句只是把复杂约束换个位置写，不改变约束含义。".to_owned(),
            "生命周期标注描述引用之间的关系，不会让局部变量活得更久。".to_owned(),
        ],
        ("concurrency", Language::Zh) => vec![
            "线程可能比当前函数活得久，所以跨线程捕获外部值时经常要 `move` 取得所有权。".to_owned(),
            "channel 发送的是值的所有权；发送后原线程通常不能继续使用这个值。".to_owned(),
            "共享可变状态要同时解决多个 owner 和互斥修改：常见组合是 `Arc<Mutex<T>>`。".to_owned(),
        ],
        _ => lesson_guide(lesson, language)
            .iter()
            .map(|item| (*item).to_owned())
            .collect(),
    }
}

fn exercise_syntax_anchors(exercise: &Exercise, language: Language) -> Vec<String> {
    match (exercise.kind, language) {
        (ExerciseKind::FillBlank, Language::Zh) => vec![
            "先看空白左边：`:` 后通常填类型；`let` 左侧通常填模式；`.` 后通常填字段或方法名。".to_owned(),
            "再看空白右边：`=>` 前是模式或 guard；函数参数列表里是签名位置；表达式内部则要能产出值。".to_owned(),
            "把答案代回整行代码，从左到右读一遍，确认它是合法 Rust 语法。".to_owned(),
        ],
        (ExerciseKind::SingleChoice, Language::Zh) => vec![
            "每个选项都必须能被代码中的具体证据支持：类型、所有权、借用、分支覆盖、返回值或 API 行为。".to_owned(),
            "先排除“总是自动”“完全由运行时决定”“一定复制”这类绝对化说法。".to_owned(),
            "正确选项不仅词面熟悉，还要能解释题干中的这段代码为什么成立或为什么失败。".to_owned(),
        ],
        (ExerciseKind::CodeOutput, Language::Zh) => vec![
            "从真正执行的语句开始推导，遇到函数调用、block、if、match 时先算出它们产出的值。".to_owned(),
            "特别检查最后一行有没有分号：无分号表达式产生值，有分号语句通常产生 `()`。".to_owned(),
            "输出答案要精确到空格、换行、引号内文本和调试格式。".to_owned(),
        ],
        (ExerciseKind::OrderSteps, Language::Zh) => vec![
            "先找到创建 owner、集合、引用或迭代器的步骤；后面的步骤不能使用还没产生的变量。".to_owned(),
            "再按依赖关系排序：借用必须发生在 owner 创建后，消费动作通常发生在转换链或修改动作之后。".to_owned(),
            "最后检查是否有使用后再 move、可变借用和不可变借用重叠这类顺序问题。".to_owned(),
        ],
        (ExerciseKind::FillBlank, Language::En) => vec![
            "Look left of the blank: after `:` means a type; left side of `let` means a pattern; after `.` means a field or method.".to_owned(),
            "Look right of the blank: before `=>` is a pattern or guard; in parameters it is signature syntax; inside expressions it must produce a value.".to_owned(),
            "Paste the answer back into the whole line and read it as valid Rust syntax.".to_owned(),
        ],
        (ExerciseKind::SingleChoice, Language::En) => vec![
            "Every option must be supported by concrete evidence: types, ownership, borrowing, branch coverage, return values, or API behavior.".to_owned(),
            "Eliminate absolute claims such as 'always automatic' or 'purely runtime'.".to_owned(),
            "The correct option must explain this exact snippet, not just contain a familiar keyword.".to_owned(),
        ],
        (ExerciseKind::CodeOutput, Language::En) => vec![
            "Start from the executed statement and evaluate calls, blocks, if, and match expressions before substituting values.".to_owned(),
            "Check semicolons carefully: final expressions produce values; statements usually produce `()`.".to_owned(),
            "Output answers must match spaces, newlines, literal text, and debug formatting exactly.".to_owned(),
        ],
        (ExerciseKind::OrderSteps, Language::En) => vec![
            "Find the step that creates the owner, collection, reference, or iterator first.".to_owned(),
            "Order by dependencies: borrowing comes after owner creation, and consuming actions come after transformations or mutations.".to_owned(),
            "Finally check for use-after-move or overlapping mutable and immutable borrows.".to_owned(),
        ],
    }
}

fn exercise_pre_answer_checklist(exercise: &Exercise, language: Language) -> Vec<String> {
    let difficulty = match (exercise.difficulty(), language) {
        (ExerciseDifficulty::Basic, Language::Zh) => "基础题：优先确认一个核心语法点，不要引入过多后续规则。",
        (ExerciseDifficulty::Practice, Language::Zh) => "进阶题：通常同时考两个相关规则，需要把两条证据都对上。",
        (ExerciseDifficulty::Challenge, Language::Zh) => "挑战题：常把相似概念放在一起迷惑你，要逐项核对边界条件。",
        (ExerciseDifficulty::Basic, Language::En) => "Basic: focus on one core syntax rule before importing later concepts.",
        (ExerciseDifficulty::Practice, Language::En) => "Practice: usually combines two related rules, so match both pieces of evidence.",
        (ExerciseDifficulty::Challenge, Language::En) => "Challenge: similar concepts are often placed together, so inspect edge cases carefully.",
    };
    let answer_shape = match (exercise.kind, language) {
        (ExerciseKind::FillBlank, Language::Zh) => "答案形状：填入后应当是一段能直接嵌回代码的 Rust 片段，大小写和符号都算答案的一部分。",
        (ExerciseKind::SingleChoice, Language::Zh) => "答案形状：只选一个最能解释代码证据的选项。",
        (ExerciseKind::CodeOutput, Language::Zh) => "答案形状：写实际输出文本，不写类型名、不写推导过程，必要时包含换行。",
        (ExerciseKind::OrderSteps, Language::Zh) => "答案形状：每一步必须能在前面步骤完成后合法执行，不能跳过变量创建或借用结束。",
        (ExerciseKind::FillBlank, Language::En) => "Answer shape: the filled text must be a Rust fragment that can be pasted back; case and punctuation matter.",
        (ExerciseKind::SingleChoice, Language::En) => "Answer shape: choose the one option that explains the code evidence.",
        (ExerciseKind::CodeOutput, Language::En) => "Answer shape: type concrete output, not a type name or reasoning steps.",
        (ExerciseKind::OrderSteps, Language::En) => "Answer shape: every step must be legal after previous steps.",
    };

    match language {
        Language::Zh => vec![
            difficulty.to_owned(),
            answer_shape.to_owned(),
            "答题前问自己：我能不能用一句“因为代码里的 ___，所以答案是 ___”解释出来？如果不能，先回到题干找证据。".to_owned(),
        ],
        Language::En => vec![
            difficulty.to_owned(),
            answer_shape.to_owned(),
            "Before answering, ask: can I say 'because the code has ___, the answer is ___'? If not, go back and find evidence.".to_owned(),
        ],
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
