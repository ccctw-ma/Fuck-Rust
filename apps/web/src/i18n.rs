use learning_core::{Answer, Exercise, KnowledgeCard, Lesson, Stage};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Zh,
    En,
}

impl Language {
    pub const fn code(self) -> &'static str {
        match self {
            Language::Zh => "zh-CN",
            Language::En => "en",
        }
    }

    pub const fn toggle_label(self) -> &'static str {
        match self {
            Language::Zh => "EN",
            Language::En => "中",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "en" => Language::En,
            _ => Language::Zh,
        }
    }
}

pub fn t(language: Language, key: &str) -> &'static str {
    match language {
        Language::Zh => match key {
            "brand_title" => "Rust 阶梯学习站",
            "brand_subtitle" => "100% Rust 前端 · WASM",
            "home" => "首页",
            "learn" => "课程路径",
            "cards" => "知识卡片",
            "stats" => "学习统计",
            "theme_light" => "亮色",
            "theme_dark" => "暗色",
            "rail_title" => "学习路径",
            "rail_caption" => "按顺序推进即可。每个知识点都配 Rust Book 章节、短 demo 和即时反馈题。",
            "total_progress" => "总进度",
            "minute_suffix" => "分钟",
            "lesson_count_suffix" => "课",
            "completed" => "完成",
            "hero_eyebrow" => "按 Rust Book 渐进学习 · 在线做题 · 小 demo",
            "hero_title_prefix" => "把 Rust 拆成",
            "hero_title_accent" => "能完成的下一步",
            "hero_copy" => "学习顺序直接对齐 The Rust Programming Language。先看一个书中同源的小 demo，再做一道能立刻反馈的题，把所有权、借用、Result、trait 和生命周期逐层吃透。",
            "continue" => "继续：",
            "view_path" => "查看课程路径",
            "review_cards" => "复习高频易错点",
            "done_exercises" => "已完成练习",
            "accuracy" => "累计正确率",
            "streak" => "连续学习",
            "days" => "天",
            "four_stages" => "四阶段路线",
            "four_stages_copy" => "每一阶段都锚到 Rust Book 对应章节，先能读懂，再能写对。",
            "learn_title" => "课程路径",
            "learn_copy" => "从变量到迭代器，每一课都对应 Rust Book 原章节、两道小题和一个可读 demo。",
            "difficulty" => "难度",
            "enter_exercise" => "进入练习",
            "book_ref" => "Rust Book",
            "submit" => "提交答案",
            "next_exercise" => "下一题",
            "expected" => "参考答案：",
            "correct_title" => "答对了，推进进度",
            "wrong_title" => "还差一点，看解释再试",
            "hint" => "提示",
            "input_blank" => "输入空白处代码",
            "input_output" => "输入你认为的输出",
            "current_order" => "当前顺序",
            "empty_order" => "还没有选择步骤",
            "reset_order" => "重排",
            "demo_output" => "输出",
            "cards_title" => "知识卡片",
            "cards_copy" => "把最容易卡住的概念拆成错误示例和修正方式，做错题时回来对照。",
            "completion_rate" => "完成率",
            "done" => "已完成",
            "weak_lessons" => "薄弱章节",
            "weak_copy" => "根据最近答题正确率推断。样本少于 2 次的章节不会被误判为薄弱。",
            "weak_empty" => "暂时没有明显薄弱章节。继续做题，统计会更准确。",
            "samples" => "样本",
            "times" => "次",
            "not_found_title" => "没找到这个学习节点",
            "not_found_copy" => "回到课程路径，选择一个可用的练习继续。",
            "back_to_path" => "回到课程路径",
            _ => "",
        },
        Language::En => match key {
            "brand_title" => "Rust Ladder",
            "brand_subtitle" => "100% Rust Frontend · WASM",
            "home" => "Home",
            "learn" => "Path",
            "cards" => "Cards",
            "stats" => "Stats",
            "theme_light" => "Light",
            "theme_dark" => "Dark",
            "rail_title" => "Learning Path",
            "rail_caption" => "Move in order. Every topic links to The Rust Book, a short demo, and instant feedback.",
            "total_progress" => "Progress",
            "minute_suffix" => "min",
            "lesson_count_suffix" => "lessons",
            "completed" => "Completed",
            "hero_eyebrow" => "The Rust Book path · Practice · Mini demos",
            "hero_title_prefix" => "Break Rust into",
            "hero_title_accent" => "one doable next step",
            "hero_copy" => "The learning order follows The Rust Programming Language. Read a book-aligned demo first, then solve one fast-feedback exercise so ownership, borrowing, Result, traits, and lifetimes become concrete.",
            "continue" => "Continue: ",
            "view_path" => "View path",
            "review_cards" => "Review pain points",
            "done_exercises" => "Exercises done",
            "accuracy" => "Accuracy",
            "streak" => "Learning streak",
            "days" => "days",
            "four_stages" => "Four-stage path",
            "four_stages_copy" => "Every stage maps to a chapter in The Rust Book: read first, then write correctly.",
            "learn_title" => "Learning Path",
            "learn_copy" => "From variables to iterators, every lesson has a Rust Book chapter, two exercises, and one readable demo.",
            "difficulty" => "Level",
            "enter_exercise" => "Practice",
            "book_ref" => "Rust Book",
            "submit" => "Submit",
            "next_exercise" => "Next",
            "expected" => "Expected: ",
            "correct_title" => "Correct. Progress saved.",
            "wrong_title" => "Not yet. Read the explanation and try again.",
            "hint" => "Hint",
            "input_blank" => "Type the missing code",
            "input_output" => "Type the output",
            "current_order" => "Current order",
            "empty_order" => "No step selected yet",
            "reset_order" => "Reset",
            "demo_output" => "Output",
            "cards_title" => "Knowledge Cards",
            "cards_copy" => "The concepts that usually block learners, shown as wrong code and a concrete fix.",
            "completion_rate" => "Completion",
            "done" => "Done",
            "weak_lessons" => "Weak Lessons",
            "weak_copy" => "Inferred from recent answer accuracy. Lessons with fewer than two attempts are ignored.",
            "weak_empty" => "No obvious weak lesson yet. Keep practicing to make the stats useful.",
            "samples" => "samples",
            "times" => "attempts",
            "not_found_title" => "Learning node not found",
            "not_found_copy" => "Return to the path and choose an available exercise.",
            "back_to_path" => "Back to path",
            _ => "",
        },
    }
}

pub fn stage_label(stage: Stage, language: Language) -> &'static str {
    match language {
        Language::Zh => stage.label(),
        Language::En => match stage {
            Stage::Foundation => "Foundation",
            Stage::Ownership => "Ownership",
            Stage::Patterns => "Patterns",
            Stage::Production => "Production",
        },
    }
}

pub fn stage_description(stage: Stage, language: Language) -> &'static str {
    match language {
        Language::Zh => stage.description(),
        Language::En => match stage {
            Stage::Foundation => {
                "Turn variables, expressions, and control flow into muscle memory."
            }
            Stage::Ownership => {
                "Focus on move, borrow, mutable references, and the rules behind them."
            }
            Stage::Patterns => "Use enums, Result, and iterators to write more idiomatic Rust.",
            Stage::Production => {
                "Build engineering intuition for traits, lifetimes, and async Rust."
            }
        },
    }
}

pub fn exercise_kind_label(kind: learning_core::ExerciseKind, language: Language) -> &'static str {
    match language {
        Language::Zh => kind.label(),
        Language::En => match kind {
            learning_core::ExerciseKind::SingleChoice => "Single choice",
            learning_core::ExerciseKind::FillBlank => "Fill blank",
            learning_core::ExerciseKind::OrderSteps => "Order steps",
            learning_core::ExerciseKind::CodeOutput => "Output",
        },
    }
}

pub fn lesson_title(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.title;
    }
    match lesson.id {
        "syntax-basics" => "Variables, Mutability, and Expressions",
        "control-flow" => "match, if let, and Patterns",
        "ownership" => "Ownership, Move, and Clone",
        "borrowing" => "Borrowing and Mutable References",
        "result-option" => "Option, Result, and Error Handling",
        "iterators-traits" => "Iterators, Traits, and Lifetime Intuition",
        _ => lesson.title,
    }
}

pub fn lesson_summary(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.summary;
    }
    match lesson.id {
        "syntax-basics" => "Understand let, mut, shadowing, and expression return values before syntax feels heavy.",
        "control-flow" => "Rust branches are expressions, and match must cover every possible case.",
        "ownership" => "The core Rust gate: each value has one owner, and moved values cannot be reused.",
        "borrowing" => "Learn why Rust allows many immutable references or one mutable reference at a time.",
        "result-option" => "Use types to express absence and recoverable failure instead of null or hidden exceptions.",
        "iterators-traits" => "Move from code that works to code that feels like Rust: iterator chains, trait bounds, and reference lifetimes.",
        _ => lesson.summary,
    }
}

pub fn lesson_goal(lesson: &Lesson, index: usize, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.goals[index];
    }
    match (lesson.id, index) {
        ("syntax-basics", 0) => "Tell immutable bindings from mutable bindings",
        ("syntax-basics", 1) => "Understand block expression return values",
        ("syntax-basics", 2) => "Read println! placeholders",
        ("control-flow", 0) => "Know why match must be exhaustive",
        ("control-flow", 1) => "Use if let when only one pattern matters",
        ("control-flow", 2) => "Keep branch return types compatible",
        ("ownership", 0) => "Detect when String moves",
        ("ownership", 1) => "Separate Copy from Clone",
        ("ownership", 2) => "Use borrowing to avoid ownership transfer",
        ("borrowing", 0) => "Read through &T without taking ownership",
        ("borrowing", 1) => "Mutate through &mut T",
        ("borrowing", 2) => "Explain why mutable and immutable borrows cannot overlap",
        ("result-option", 0) => "Read Option<T> and Result<T, E>",
        ("result-option", 1) => "Handle success and failure with match",
        ("result-option", 2) => "Understand that ? returns early on failure",
        ("iterators-traits", 0) => "Use map, filter, and collect for data flow",
        ("iterators-traits", 1) => "Read impl Trait and trait bounds",
        ("iterators-traits", 2) => {
            "Know that lifetime annotations describe relationships, not extensions"
        }
        _ => lesson.goals[index],
    }
}

pub fn demo_title(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.demo.title;
    }
    match lesson.id {
        "syntax-basics" => "The Rust Book: mutable variable",
        "control-flow" => "The Rust Book: match returns a value",
        "ownership" => "The Rust Book: move instead of shallow copy",
        "borrowing" => "The Rust Book: borrowing a String",
        "result-option" => "The Rust Book: matching on Result",
        "iterators-traits" => "The Rust Book: iterator sum",
        _ => lesson.demo.title,
    }
}

pub fn demo_takeaway(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.demo.takeaway;
    }
    match lesson.id {
        "syntax-basics" => "Bindings are immutable by default; write mut only when the same binding must change.",
        "control-flow" => "match is exhaustive and each arm can produce the value assigned to a variable.",
        "ownership" => "A String owns heap data. Moving it prevents two owners from freeing the same allocation.",
        "borrowing" => "Passing &String lets a function read the value while the caller keeps ownership.",
        "result-option" => "Result forces success and failure paths into the type system and the control flow.",
        "iterators-traits" => "Iterator adapters are lazy; consuming methods such as sum or collect run the chain.",
        _ => lesson.demo.takeaway,
    }
}

pub fn exercise_title(exercise: &Exercise, language: Language) -> &'static str {
    if language == Language::Zh {
        return exercise.title;
    }
    match exercise.id {
        "syntax-let-mut" => "Make a variable mutable",
        "syntax-output" => "Read a block expression",
        "match-exhaustive" => "Why match needs a fallback",
        "if-let-some" => "Care only about Some",
        "ownership-move" => "Find the invalid use after move",
        "ownership-clone" => "Keep two Strings",
        "borrowing-mut-ref" => "Mutate through a mutable borrow",
        "borrowing-rule" => "Borrowing rule in one sentence",
        "option-match" => "Handle None safely",
        "result-question-mark" => "What ? really means",
        "iterator-chain" => "Iterator chain result",
        "lifetime-meaning" => "What lifetime annotations mean",
        _ => exercise.title,
    }
}

pub fn exercise_prompt(exercise: &Exercise, language: Language) -> &'static str {
    if language == Language::Zh {
        return exercise.prompt;
    }
    match exercise.id {
        "syntax-let-mut" => "The code wants to add 1 to score. What fills the blank?",
        "syntax-output" => "What does this code print?",
        "match-exhaustive" => "Which statement is correct?",
        "if-let-some" => "Which pattern prints the value only when maybe is Some?",
        "ownership-move" => "Which line fails to compile?",
        "ownership-clone" => {
            "What should fill the blank if both name and alias must remain usable?"
        }
        "borrowing-mut-ref" => "Put the steps in an order that compiles and mutates the Vec.",
        "borrowing-rule" => "Which sentence is most accurate?",
        "option-match" => "What branch safely handles None?",
        "result-question-mark" => "What does ? do on Result?",
        "iterator-chain" => "What does this code print?",
        "lifetime-meaning" => "What do lifetime annotations actually express?",
        _ => exercise.prompt,
    }
}

pub fn exercise_option(exercise: &Exercise, index: usize, language: Language) -> &'static str {
    if language == Language::Zh {
        return exercise.options[index];
    }
    match (exercise.id, index) {
        ("match-exhaustive", 0) => "match must cover every case; _ catches the rest",
        ("match-exhaustive", 1) => "match only checks values that appeared at runtime",
        ("match-exhaustive", 2) => "_ disables the previous arms",
        ("ownership-move", 0) => "`let name = String::from(\"Rust\");`",
        ("ownership-move", 1) => "`let alias = name;`",
        ("ownership-move", 2) => "`println!(\"{name}\");`",
        ("borrowing-mut-ref", 0) => "let mut nums = vec![1, 2];",
        ("borrowing-mut-ref", 1) => "let view = &mut nums;",
        ("borrowing-mut-ref", 2) => "view.push(3);",
        ("borrowing-mut-ref", 3) => "println!(\"{:?}\", nums);",
        ("borrowing-rule", 0) => {
            "At a time, you can have many immutable references or one mutable reference"
        }
        ("borrowing-rule", 1) => "If a variable is mut, unlimited mutable references are allowed",
        ("borrowing-rule", 2) => "Immutable references automatically clone the underlying data",
        ("result-question-mark", 0) => {
            "On Ok, unwrap the value; on Err, return Err from the current function"
        }
        ("result-question-mark", 1) => "Catch the error and keep running the next line",
        ("result-question-mark", 2) => "Convert Err into None automatically",
        ("lifetime-meaning", 0) => "The returned reference will not outlive the input references",
        ("lifetime-meaning", 1) => "The function extends the lifetime of left and right",
        ("lifetime-meaning", 2) => "All strings are copied into static memory",
        _ => exercise.options[index],
    }
}

pub fn exercise_explanation(exercise: &Exercise, language: Language) -> &'static str {
    if language == Language::Zh {
        return exercise.explanation;
    }
    match exercise.id {
        "syntax-let-mut" => "`let` bindings are immutable by default. Use `mut` when the same binding changes.",
        "syntax-output" => "The last line has no semicolon, so `base + 2` is the block's value.",
        "match-exhaustive" => "Rust requires match to cover all cases; `_` matches everything left.",
        "if-let-some" => "`if let Some(value) = maybe` is for the case where only one pattern matters.",
        "ownership-move" => "`String` is not Copy. Assigning it to alias moves ownership, so name cannot be used.",
        "ownership-clone" => "`clone()` explicitly copies heap data so both Strings own separate allocations.",
        "borrowing-mut-ref" => "Create the mutable owner, create the mutable reference, mutate through it, then read nums after the borrow ends.",
        "borrowing-rule" => "The active-reference rule is many readers or one writer.",
        "option-match" => "Option has exactly Some and None, so match must handle both.",
        "result-question-mark" => "`?` is early-return sugar, not exception handling.",
        "iterator-chain" => "filter keeps 2 and 4; map multiplies each by 10.",
        "lifetime-meaning" => "Lifetime annotations describe reference relationships; they do not extend actual lifetimes.",
        _ => exercise.explanation,
    }
}

pub fn exercise_hint(exercise: &Exercise, language: Language) -> &'static str {
    if language == Language::Zh {
        return exercise.hint;
    }
    match exercise.id {
        "syntax-let-mut" => "This changes the same binding with `+=`; it is not shadowing.",
        "syntax-output" => "Look for the missing semicolon after `base + 2`.",
        "match-exhaustive" => "Ask what happens if n is 100.",
        "if-let-some" => "The pattern must match Some and bind the inner value.",
        "ownership-move" => {
            "The assignment is legal. The error appears when the old owner is used again."
        }
        "ownership-clone" => "This is an explicit deep copy, and Rust makes that cost visible.",
        "borrowing-mut-ref" => "`nums` must be declared mut before you create `&mut nums`.",
        "borrowing-rule" => "The key phrase is active references at the same time.",
        "option-match" => "Rust uses an enum variant, not null, to express absence.",
        "result-question-mark" => {
            "The current function's return type must be able to carry the error."
        }
        "iterator-chain" => "Process the chain from top to bottom.",
        "lifetime-meaning" => "`'a` is a constraint, not an allocation strategy.",
        _ => exercise.hint,
    }
}

pub fn answer_summary(answer: Answer, language: Language) -> String {
    match language {
        Language::Zh => answer.summary(),
        Language::En => match answer {
            Answer::Choice(index) => format!("option {}", index + 1),
            Answer::Text(value) => value.to_owned(),
            Answer::Ordered(items) => items.join(" -> "),
            Answer::Output(output) => output.trim().to_owned(),
        },
    }
}

pub fn card_title(card: &KnowledgeCard, language: Language) -> &'static str {
    if language == Language::Zh {
        return card.title;
    }
    match card.id {
        "move-after-use" => "Why a moved variable cannot be used",
        "mut-ref-exclusive" => "Why mutable references are exclusive",
        "question-mark" => "? is not try-catch",
        "lifetime-not-extend" => "Lifetime annotations do not extend references",
        _ => card.title,
    }
}

pub fn card_summary(card: &KnowledgeCard, language: Language) -> &'static str {
    if language == Language::Zh {
        return card.summary;
    }
    match card.id {
        "move-after-use" => "String owns heap memory. After move, ownership transfers and the old binding is invalid.",
        "mut-ref-exclusive" => "Exclusive mutable references prevent read/write overlap and stale reads.",
        "question-mark" => "? returns early on Err or None; it does not catch and continue.",
        "lifetime-not-extend" => "A lifetime annotation describes validity relationships; it never makes a local value live longer.",
        _ => card.summary,
    }
}

pub fn card_fix(card: &KnowledgeCard, language: Language) -> &'static str {
    if language == Language::Zh {
        return card.fix;
    }
    match card.id {
        "move-after-use" => {
            "Borrow with `&a`, or explicitly call `a.clone()` when you need another owned value."
        }
        "mut-ref-exclusive" => "Shorten the immutable borrow scope before creating `&mut value`.",
        "question-mark" => {
            "Return `Result<_, _>` or write match when you need to handle the error locally."
        }
        "lifetime-not-extend" => {
            "Return an owned `String`, or return a reference that came from the caller."
        }
        _ => card.fix,
    }
}
