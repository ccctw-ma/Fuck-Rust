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
            "learn_copy" => "从变量到并发，每一课都对应 Rust Book 原章节、多道小题和一个可读 demo。",
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
            "learn_copy" => "From variables to concurrency, every lesson has a Rust Book chapter, several exercises, and one readable demo.",
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

pub fn exercise_level_label(exercise: &Exercise, language: Language) -> &'static str {
    match (exercise.level(), language) {
        (1, Language::Zh) => "基础",
        (2, Language::Zh) => "进阶",
        (_, Language::Zh) => "挑战",
        (1, Language::En) => "Basic",
        (2, Language::En) => "Practice",
        (_, Language::En) => "Challenge",
    }
}

pub fn lesson_title(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.title;
    }
    match lesson.id {
        "syntax-basics" => "Variables, Mutability, and Expressions",
        "control-flow" => "match, if let, and Patterns",
        "data-functions" => "Data Types, Functions, and Returns",
        "ownership" => "Ownership, Move, and Clone",
        "slices" => "Slices and first_word",
        "borrowing" => "Borrowing and Mutable References",
        "structs-enums" => "Structs, Methods, and Enums",
        "result-option" => "Option, Result, and Error Handling",
        "collections" => "Collections: Vec, String, and HashMap",
        "iterators-traits" => "Iterators, Traits, and Lifetime Intuition",
        "generics-traits" => "Generics, Trait Bounds, and Lifetimes",
        "concurrency" => "Threads, Message Passing, and Shared State",
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
        "data-functions" => "Connect scalar types, compound types, function parameters, and expression return values.",
        "ownership" => "The core Rust gate: each value has one owner, and moved values cannot be reused.",
        "slices" => "Slices borrow a continuous region of a collection and make string boundaries safer.",
        "borrowing" => "Learn why Rust allows many immutable references or one mutable reference at a time.",
        "structs-enums" => "Use structs for domain data, impl blocks for behavior, and enums for finite states.",
        "result-option" => "Use types to express absence and recoverable failure instead of null or hidden exceptions.",
        "collections" => "Use standard collections for real data flow while practicing mutability and ownership.",
        "iterators-traits" => "Move from code that works to code that feels like Rust: iterator chains, trait bounds, and reference lifetimes.",
        "generics-traits" => "Abstract repeated logic with generics, describe capabilities with trait bounds, and relate references with lifetimes.",
        "concurrency" => "Use threads, channels, Mutex, and Arc to see how Rust moves concurrency errors to compile time.",
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
        ("data-functions", 0) => "Read tuple and array type syntax",
        ("data-functions", 1) => "Tell statements from expressions",
        ("data-functions", 2) => "Know that function returns come from the last expression",
        ("ownership", 0) => "Detect when String moves",
        ("ownership", 1) => "Separate Copy from Clone",
        ("ownership", 2) => "Use borrowing to avoid ownership transfer",
        ("slices", 0) => "Read string slice range syntax",
        ("slices", 1) => "Know that &str is usually a borrowed view",
        ("slices", 2) => "Explain why first_word returns a safer slice",
        ("borrowing", 0) => "Read through &T without taking ownership",
        ("borrowing", 1) => "Mutate through &mut T",
        ("borrowing", 2) => "Explain why mutable and immutable borrows cannot overlap",
        ("structs-enums", 0) => "Use field init shorthand and struct update syntax",
        ("structs-enums", 1) => "Tell &self, &mut self, and self apart",
        ("structs-enums", 2) => "Use enums and match for variants",
        ("result-option", 0) => "Read Option<T> and Result<T, E>",
        ("result-option", 1) => "Handle success and failure with match",
        ("result-option", 2) => "Understand that ? returns early on failure",
        ("collections", 0) => "Create and update Vec<T>",
        ("collections", 1) => "Update String safely",
        ("collections", 2) => "Use HashMap::entry for insert-or-update",
        ("iterators-traits", 0) => "Use map, filter, and collect for data flow",
        ("iterators-traits", 1) => "Read impl Trait and trait bounds",
        ("iterators-traits", 2) => {
            "Know that lifetime annotations describe relationships, not extensions"
        }
        ("generics-traits", 0) => "Read generic function signatures",
        ("generics-traits", 1) => "Add trait bounds to generic parameters",
        ("generics-traits", 2) => "Explain the lifetime parameter in longest",
        ("concurrency", 0) => "Know when thread::spawn needs move",
        ("concurrency", 1) => "Transfer ownership between threads with channels",
        ("concurrency", 2) => "Understand Arc<Mutex<T>> for shared mutable state",
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
        "data-functions" => "The Rust Book: tuple destructuring",
        "ownership" => "The Rust Book: move instead of shallow copy",
        "slices" => "The Rust Book: string slices",
        "borrowing" => "The Rust Book: borrowing a String",
        "structs-enums" => "The Rust Book: struct update syntax",
        "result-option" => "The Rust Book: matching on Result",
        "collections" => "The Rust Book: HashMap entry",
        "iterators-traits" => "The Rust Book: iterator sum",
        "generics-traits" => "The Rust Book: longest lifetime",
        "concurrency" => "The Rust Book: move closures into threads",
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
        "data-functions" => "Tuples are fixed-size compound values. Pattern destructuring binds each position at once.",
        "ownership" => "A String owns heap data. Moving it prevents two owners from freeing the same allocation.",
        "slices" => "A slice does not own data; it borrows a region and ties the returned value to the source.",
        "borrowing" => "Passing &String lets a function read the value while the caller keeps ownership.",
        "structs-enums" => "Struct update syntax can move non-Copy fields from the old value into the new value.",
        "result-option" => "Result forces success and failure paths into the type system and the control flow.",
        "collections" => "entry(...).or_insert(...) is the standard insert-or-update pattern for maps.",
        "iterators-traits" => "Iterator adapters are lazy; consuming methods such as sum or collect run the chain.",
        "generics-traits" => "A lifetime parameter relates returned references to input references; it does not extend storage.",
        "concurrency" => "move gives the spawned thread ownership of captured values so it cannot outlive borrowed data.",
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
        "tuple-destructure" => "Destructure a tuple",
        "function-return" => "Return the last expression",
        "array-type" => "Array types include length",
        "match-exhaustive" => "Why match needs a fallback",
        "if-let-some" => "Care only about Some",
        "ownership-move" => "Find the invalid use after move",
        "ownership-clone" => "Keep two Strings",
        "ownership-copy-trap" => "Copy or move?",
        "slice-range" => "Take the hello slice",
        "first-word-return" => "What should first_word return?",
        "string-literal-slice" => "Type of a string literal",
        "borrowing-mut-ref" => "Mutate through a mutable borrow",
        "borrowing-rule" => "Borrowing rule in one sentence",
        "borrow-scope-release" => "Shorten a borrow's active scope",
        "dangling-reference" => "Why dangling references are rejected",
        "struct-update" => "Struct update syntax",
        "method-self" => "Read-only methods borrow self",
        "enum-match-option" => "Option is an enum",
        "option-match" => "Handle None safely",
        "result-question-mark" => "What ? really means",
        "vec-mut-push" => "Vec push needs mutability",
        "string-update" => "Append a string slice",
        "hashmap-entry" => "Insert only when the key is absent",
        "iterator-chain" => "Iterator chain result",
        "lifetime-meaning" => "What lifetime annotations mean",
        "iter-vs-into-iter" => "iter vs into_iter ownership",
        "collect-type" => "collect needs a target type",
        "generic-largest" => "largest needs a capability bound",
        "trait-bound-display" => "Trait bound for printing",
        "lifetime-longest" => "The lifetime of longest's return",
        "where-clause" => "Use a where clause",
        "static-lifetime-myth" => "'static is not magic",
        "thread-move" => "Why a thread closure needs move",
        "channel-send" => "Sending through a channel moves values",
        "mutex-lock" => "Shared mutable counter",
        "arc-clone" => "Arc::clone clones the pointer",
        "mutex-guard-drop" => "When MutexGuard releases the lock",
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
        "tuple-destructure" => "Following The Rust Book tuple example, what pattern binds all three values?",
        "function-return" => "What does this function call print?",
        "array-type" => "Which type means an array of five i32 values?",
        "match-exhaustive" => "Which statement is correct?",
        "if-let-some" => "Which pattern prints the value only when maybe is Some?",
        "ownership-move" => "Which line fails to compile?",
        "ownership-clone" => {
            "What should fill the blank if both name and alias must remain usable?"
        }
        "ownership-copy-trap" => "After which assignment can the original variable still be used?",
        "slice-range" => "Following The Rust Book string slice example, what range extracts hello?",
        "first-word-return" => "Why does The Rust Book change first_word to return &str instead of usize?",
        "string-literal-slice" => "What is the most accurate type of the string literal \"hello\"?",
        "borrowing-mut-ref" => "Put the steps in an order that compiles and mutates the Vec.",
        "borrowing-rule" => "Which sentence is most accurate?",
        "borrow-scope-release" => "Put the steps in an order that reads and then mutates a String.",
        "dangling-reference" => "Why does this function fail to compile?",
        "struct-update" => "What fills the blank to reuse the remaining fields from user1?",
        "method-self" => "If a method only reads struct fields and does not take ownership, what does it usually take?",
        "enum-match-option" => "What match arm extracts the Some value and adds 1?",
        "option-match" => "What branch safely handles None?",
        "result-question-mark" => "What does ? do on Result?",
        "vec-mut-push" => "What fills the blank if we want to append to a Vec?",
        "string-update" => "Which method appends `bar` to a String?",
        "hashmap-entry" => "Following The Rust Book entry pattern, what fills the blank?",
        "iterator-chain" => "What does this code print?",
        "lifetime-meaning" => "What do lifetime annotations actually express?",
        "iter-vs-into-iter" => "After calling `into_iter()` on a `Vec<String>`, what happens to the original vector?",
        "collect-type" => "What fills the blank to collect into `Vec<i32>`?",
        "generic-largest" => "If largest compares arbitrary T values, what trait ability does T need?",
        "trait-bound-display" => "If the function prints item with `{}`, which trait bound is needed?",
        "lifetime-longest" => "What does the signature of longest express?",
        "where-clause" => "What trait fills the where clause if the function prints item with `{}`?",
        "static-lifetime-myth" => "What does `&'static str` most accurately mean?",
        "thread-move" => "Why does The Rust Book add `move` to the thread closure that prints a vector?",
        "channel-send" => "After sending a String through `tx.send(val)`, what happens to val?",
        "mutex-lock" => "Which smart pointer usually wraps Mutex for sharing across threads?",
        "arc-clone" => "Why do Rust examples often write `Arc::clone(&counter)` inside a thread loop?",
        "mutex-guard-drop" => "When does the guard returned by `counter.lock().unwrap()` release the lock?",
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
        ("array-type", 0) => "[i32; 5]",
        ("array-type", 1) => "Vec<i32>",
        ("array-type", 2) => "(i32, 5)",
        ("ownership-move", 0) => "`let name = String::from(\"Rust\");`",
        ("ownership-move", 1) => "`let alias = name;`",
        ("ownership-move", 2) => "`println!(\"{name}\");`",
        ("ownership-copy-trap", 0) => "`a` is still usable because i32 implements Copy",
        ("ownership-copy-trap", 1) => "`s1` is still usable because String automatically copies",
        ("ownership-copy-trap", 2) => "Neither original variable can be used",
        ("first-word-return", 0) => {
            "&str ties the result to the source borrow, avoiding stale indexes"
        }
        ("first-word-return", 1) => "&str copies the first word and is faster",
        ("first-word-return", 2) => "&str bypasses UTF-8 checks",
        ("string-literal-slice", 0) => "&str",
        ("string-literal-slice", 1) => "String",
        ("string-literal-slice", 2) => "[char; 5]",
        ("borrowing-mut-ref", 0) => "let mut nums = vec![1, 2];",
        ("borrowing-mut-ref", 1) => "let view = &mut nums;",
        ("borrowing-mut-ref", 2) => "view.push(3);",
        ("borrowing-mut-ref", 3) => "println!(\"{:?}\", nums);",
        ("borrowing-rule", 0) => {
            "At a time, you can have many immutable references or one mutable reference"
        }
        ("borrowing-rule", 1) => "If a variable is mut, unlimited mutable references are allowed",
        ("borrowing-rule", 2) => "Immutable references automatically clone the underlying data",
        ("borrow-scope-release", 0) => "let mut s = String::from(\"hello\");",
        ("borrow-scope-release", 1) => "let len = s.len();",
        ("borrow-scope-release", 2) => "println!(\"{len}\");",
        ("borrow-scope-release", 3) => "s.push_str(\" world\");",
        ("dangling-reference", 0) => {
            "s is dropped when the function ends, so the returned reference would dangle"
        }
        ("dangling-reference", 1) => "String cannot be referenced",
        ("dangling-reference", 2) => "The return type must be &'static String",
        ("method-self", 0) => "&self",
        ("method-self", 1) => "&mut self",
        ("method-self", 2) => "self",
        ("result-question-mark", 0) => {
            "On Ok, unwrap the value; on Err, return Err from the current function"
        }
        ("result-question-mark", 1) => "Catch the error and keep running the next line",
        ("result-question-mark", 2) => "Convert Err into None automatically",
        ("lifetime-meaning", 0) => "The returned reference will not outlive the input references",
        ("lifetime-meaning", 1) => "The function extends the lifetime of left and right",
        ("lifetime-meaning", 2) => "All strings are copied into static memory",
        ("iter-vs-into-iter", 0) => "names is consumed and cannot be used again",
        ("iter-vs-into-iter", 1) => "names is still usable because it was only borrowed",
        ("iter-vs-into-iter", 2) => "into_iter returns indexes",
        ("generic-largest", 0) => "PartialOrd",
        ("generic-largest", 1) => "Iterator",
        ("generic-largest", 2) => "Default",
        ("lifetime-longest", 0) => "The returned reference cannot outlive the shorter of x and y",
        ("lifetime-longest", 1) => "The returned reference is always static",
        ("lifetime-longest", 2) => "The function copies the longer string",
        ("static-lifetime-myth", 0) => "The referenced data is valid for the entire program",
        ("static-lifetime-myth", 1) => "The variable binding cannot be moved",
        ("static-lifetime-myth", 2) => "The string is allocated on every call",
        ("thread-move", 0) => {
            "Move v into the thread so it cannot reference an expired stack value"
        }
        ("thread-move", 1) => "Make the thread run faster",
        ("thread-move", 2) => "Convert Vec into a static array",
        ("channel-send", 0) => {
            "Ownership moves to the receiver, so the sender cannot use val again"
        }
        ("arc-clone", 0) => "Increase the reference count so many threads share the same Mutex",
        ("arc-clone", 1) => "Deep-copy the number inside the Mutex",
        ("arc-clone", 2) => "Make the Mutex immutable",
        ("mutex-guard-drop", 0) => "The guard releases the lock when it leaves scope",
        ("mutex-guard-drop", 1) => "The lock is released when println! is called",
        ("mutex-guard-drop", 2) => "The lock is never released before the thread ends",
        ("channel-send", 1) => "send automatically clones val",
        ("channel-send", 2) => "val becomes an empty string",
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
        "tuple-destructure" => "A tuple can be destructured with a pattern: `let (x, y, z) = tup;`.",
        "function-return" => "The function body's final expression has no semicolon, so it becomes the return value.",
        "array-type" => "Array types are written `[T; N]`; the length N is part of the type.",
        "match-exhaustive" => "Rust requires match to cover all cases; `_` matches everything left.",
        "if-let-some" => "`if let Some(value) = maybe` is for the case where only one pattern matters.",
        "ownership-move" => "`String` is not Copy. Assigning it to alias moves ownership, so name cannot be used.",
        "ownership-clone" => "`clone()` explicitly copies heap data so both Strings own separate allocations.",
        "ownership-copy-trap" => "Simple scalar values such as i32 implement Copy. String owns heap memory, so assignment moves it.",
        "slice-range" => "`&s[0..5]` borrows bytes 0 through 4, which spell `hello`.",
        "first-word-return" => "Returning a slice lets the compiler keep the result tied to the original string borrow.",
        "string-literal-slice" => "A string literal is stored in the binary, and the variable has type `&str`.",
        "borrowing-mut-ref" => "Create the mutable owner, create the mutable reference, mutate through it, then read nums after the borrow ends.",
        "borrowing-rule" => "The active-reference rule is many readers or one writer.",
        "borrow-scope-release" => "After the last use of the immutable read, the borrow ends and mutation can happen.",
        "dangling-reference" => "A local value is dropped at function end. Returning a reference to it would create a dangling reference.",
        "struct-update" => "`..user1` takes the remaining fields from user1; non-Copy fields move.",
        "method-self" => "`&self` is short for `self: &Self`, which fits read-only methods.",
        "enum-match-option" => "Option<T> is an enum. `Some(n)` matches the variant and binds the inner value.",
        "option-match" => "Option has exactly Some and None, so match must handle both.",
        "result-question-mark" => "`?` is early-return sugar, not exception handling.",
        "vec-mut-push" => "`push` changes the Vec, so the binding must be `mut`.",
        "string-update" => "`push_str` appends a string slice without taking ownership of the argument.",
        "hashmap-entry" => "`or_insert` inserts only when the key is absent and returns a mutable reference to the value.",
        "iterator-chain" => "filter keeps 2 and 4; map multiplies each by 10.",
        "lifetime-meaning" => "Lifetime annotations describe reference relationships; they do not extend actual lifetimes.",
        "iter-vs-into-iter" => "`into_iter()` consumes the collection and yields owned values. Use `iter()` when you only want borrowed values.",
        "collect-type" => "`collect` can build many collection types, so the target type must be inferred or written explicitly.",
        "generic-largest" => "Comparing T with `>` or `<` requires a trait bound such as `PartialOrd`.",
        "trait-bound-display" => "`{}` formatting uses `std::fmt::Display`; `{:?}` uses Debug.",
        "lifetime-longest" => "`'a` relates the output reference to both input references and enforces the shorter valid lifetime.",
        "where-clause" => "A where clause moves trait bounds out of the angle brackets and keeps complex signatures readable.",
        "static-lifetime-myth" => "`'static` means the referenced data can live for the whole program; it does not freeze the binding.",
        "thread-move" => "A spawned thread may outlive the current scope. `move` makes the closure own v.",
        "channel-send" => "Channels transfer ownership; using val after send would be rejected.",
        "mutex-lock" => "`Arc<T>` gives thread-safe shared ownership, while `Mutex<T>` provides mutually exclusive access.",
        "arc-clone" => "`Arc::clone` increases the atomic reference count so threads share the same allocation.",
        "mutex-guard-drop" => "MutexGuard releases the lock when it is dropped at the end of its scope.",
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
        "tuple-destructure" => "The left side is a pattern, not a type.",
        "function-return" => "Expressions have no trailing semicolon; statements do.",
        "array-type" => "Vec can grow; arrays have a fixed length in the type.",
        "match-exhaustive" => "Ask what happens if n is 100.",
        "if-let-some" => "The pattern must match Some and bind the inner value.",
        "ownership-move" => {
            "The assignment is legal. The error appears when the old owner is used again."
        }
        "ownership-clone" => "This is an explicit deep copy, and Rust makes that cost visible.",
        "ownership-copy-trap" => {
            "Ask whether the type owns heap resources or is a simple Copy scalar."
        }
        "slice-range" => "Rust ranges exclude the right endpoint.",
        "first-word-return" => "An index is only a number; a slice carries a borrow relationship.",
        "string-literal-slice" => "This is why string literals are immutable.",
        "borrowing-mut-ref" => "`nums` must be declared mut before you create `&mut nums`.",
        "borrowing-rule" => "The key phrase is active references at the same time.",
        "borrow-scope-release" => {
            "Look for the last use of `len`; after that, the immutable borrow is done."
        }
        "dangling-reference" => "Returning ownership avoids the invalid reference entirely.",
        "struct-update" => "The syntax begins with two dots.",
        "method-self" => "area does not mutate or consume the Rectangle.",
        "enum-match-option" => "This is like if let, but match must also cover None.",
        "option-match" => "Rust uses an enum variant, not null, to express absence.",
        "result-question-mark" => {
            "The current function's return type must be able to carry the error."
        }
        "vec-mut-push" => "Appending mutates the collection.",
        "string-update" => "push appends one char; push_str appends &str.",
        "hashmap-entry" => "The word-frequency example uses `entry(...).or_insert(0)`.",
        "iterator-chain" => "Process the chain from top to bottom.",
        "lifetime-meaning" => "`'a` is a constraint, not an allocation strategy.",
        "iter-vs-into-iter" => "The name `into` usually means ownership conversion.",
        "collect-type" => "The turbofish form is `collect::<Vec<i32>>()`.",
        "generic-largest" => "Generics require you to state the capabilities you use.",
        "trait-bound-display" => "Display is for user-facing `{}` formatting.",
        "lifetime-longest" => {
            "A lifetime parameter describes a relationship, not ownership or allocation."
        }
        "where-clause" => "Same Display requirement, different syntax location.",
        "static-lifetime-myth" => {
            "String literals are the common beginner example of `&'static str`."
        }
        "thread-move" => "The concurrency chapter is still ownership in action.",
        "channel-send" => "Message passing hands data from one thread to another.",
        "mutex-lock" => "Rc<T> is not safe to share across threads; the book uses Arc<Mutex<T>>.",
        "arc-clone" => "Arc is atomic reference counting.",
        "mutex-guard-drop" => "RAII means cleanup happens when the guard leaves scope.",
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
