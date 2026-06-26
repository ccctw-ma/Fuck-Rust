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
            "brand_title" => "ripgrep 源码 Rust 学习站",
            "home" => "首页",
            "learn" => "课程路径",
            "cards" => "知识卡片",
            "stats" => "学习统计",
            "theme_light" => "亮色",
            "theme_dark" => "暗色",
            "rail_title" => "学习路径",
            "rail_caption" => "按 ripgrep 源码阅读顺序推进：先看真实代码职责，再对照 Rust Book 规则做题。",
            "open_categories" => "展开分类",
            "close_categories" => "收起分类",
            "path_short" => "路径",
            "total_progress" => "总进度",
            "minute_suffix" => "分钟",
            "lesson_count_suffix" => "课",
            "completed" => "完成",
            "hero_eyebrow" => "跟着 ripgrep 源码读 Rust · Rust Book 对照 · 在线做题",
            "hero_title_prefix" => "从真实源码里",
            "hero_title_accent" => "学会 Rust",
            "hero_copy" => "学习路径改成围绕 ripgrep 展开：入口、参数、pattern、writer、解压、遍历、glob 和并行搜索都对应真实源码片段。每道题先说明这段源码在做什么，再把 Rust Book 规则落到具体代码证据上。",
            "continue" => "继续：",
            "view_path" => "查看课程路径",
            "review_cards" => "复习高频易错点",
            "done_exercises" => "已完成练习",
            "accuracy" => "累计正确率",
            "streak" => "连续学习",
            "days" => "天",
            "four_stages" => "四阶段路线",
            "four_stages_copy" => "每一阶段都锚到 ripgrep 的真实模块：先读懂源码职责，再掌握背后的 Rust 规则。",
            "learn_title" => "课程路径",
            "learn_copy" => "从 `crates/core/main.rs` 到 globset、writer 和并行搜索，每一课都用 ripgrep 源码承载一个 Rust 知识模块。",
            "difficulty" => "难度",
            "enter_exercise" => "进入练习",
            "book_ref" => "Rust Book",
            "source_ref" => "查看 ripgrep 源码 ↗",
            "playground" => "Rust Playground",
            "playground_title" => "Rust Playground",
            "playground_copy" => "右侧即时实验区，写一段 Rust 后用官方 Playground 执行。支持 Tab 缩进和 Ctrl/⌘ + Enter 运行。",
            "playground_open" => "打开 Playground",
            "playground_close" => "收起 Playground",
            "playground_run" => "运行代码",
            "playground_running" => "运行中...",
            "playground_output" => "运行输出",
            "playground_width" => "面板宽度",
            "playground_resize" => "拖拽调整 Playground 宽度",
            "playground_drag_resize" => "拖拽左侧边框调整宽度",
            "playground_ready" => "准备运行。",
            "playground_success" => "执行成功",
            "playground_failed" => "执行失败",
            "playground_error" => "Playground 请求失败，请稍后再试。",
            "submit" => "提交答案",
            "previous_exercise" => "上一题",
            "next_exercise" => "下一题",
            "expected" => "参考答案：",
            "correct_title" => "答对了，推进进度",
            "wrong_title" => "还差一点，看解释再试",
            "hint" => "提示",
            "how_to_solve" => "怎么拆解这道题",
            "common_trap" => "常见误区",
            "input_blank" => "输入空白处代码",
            "input_output" => "输入你认为的输出",
            "current_order" => "当前顺序",
            "empty_order" => "还没有选择步骤",
            "reset_order" => "重排",
            "demo_output" => "输出",
            "quick_guide" => "源码阅读模块",
            "learning_goals" => "这题对应",
            "primer_title" => "答题前知识模块",
            "book_points" => "Rust Book 规则",
            "exercise_link" => "题目落点",
            "answer_hint" => "答题提示",
            "source_anchor" => "当前源码锚点",
            "source_role" => "源码职责",
            "source_rule" => "对应规则",
            "source_question" => "题目关联",
            "cards_title" => "知识卡片",
            "cards_copy" => "把读 ripgrep 源码时最容易卡住的所有权、借用、错误处理和生命周期问题，拆成错误示例和修正方式。",
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
            "brand_title" => "Rust via ripgrep",
            "home" => "Home",
            "learn" => "Path",
            "cards" => "Cards",
            "stats" => "Stats",
            "theme_light" => "Light",
            "theme_dark" => "Dark",
            "rail_title" => "Learning Path",
            "rail_caption" => "Move through ripgrep source: read the real module, map it to The Rust Book, then practice.",
            "open_categories" => "Open categories",
            "close_categories" => "Collapse categories",
            "path_short" => "Path",
            "total_progress" => "Progress",
            "minute_suffix" => "min",
            "lesson_count_suffix" => "lessons",
            "completed" => "Completed",
            "hero_eyebrow" => "Learn Rust through ripgrep source · Book rules · Practice",
            "hero_title_prefix" => "Learn Rust from",
            "hero_title_accent" => "real source code",
            "hero_copy" => "The path now follows ripgrep: entry points, arguments, patterns, writers, decompression, walking, glob matching, and parallel search. Each exercise starts from a concrete source snippet, then maps the Rust Book rule back to code evidence.",
            "continue" => "Continue: ",
            "view_path" => "View path",
            "review_cards" => "Review pain points",
            "done_exercises" => "Exercises done",
            "accuracy" => "Accuracy",
            "streak" => "Learning streak",
            "days" => "days",
            "four_stages" => "Four-stage path",
            "four_stages_copy" => "Every stage maps to real ripgrep modules: understand the source responsibility, then learn the Rust rule behind it.",
            "learn_title" => "Learning Path",
            "learn_copy" => "From `crates/core/main.rs` to globset, writers, and parallel search, every lesson anchors one Rust topic in ripgrep source.",
            "difficulty" => "Level",
            "enter_exercise" => "Practice",
            "book_ref" => "Rust Book",
            "source_ref" => "Open ripgrep source ↗",
            "playground" => "Rust Playground",
            "playground_title" => "Rust Playground",
            "playground_copy" => "A right-side scratchpad powered by the official Rust Playground. Tab indents; Ctrl/⌘ + Enter runs.",
            "playground_open" => "Open Playground",
            "playground_close" => "Close Playground",
            "playground_run" => "Run code",
            "playground_running" => "Running...",
            "playground_output" => "Output",
            "playground_width" => "Panel width",
            "playground_resize" => "Drag to resize Playground",
            "playground_drag_resize" => "Drag the left edge to resize",
            "playground_ready" => "Ready to run.",
            "playground_success" => "Run succeeded",
            "playground_failed" => "Run failed",
            "playground_error" => "Playground request failed. Try again later.",
            "submit" => "Submit",
            "previous_exercise" => "Previous",
            "next_exercise" => "Next",
            "expected" => "Expected: ",
            "correct_title" => "Correct. Progress saved.",
            "wrong_title" => "Not yet. Read the explanation and try again.",
            "hint" => "Hint",
            "how_to_solve" => "How to break it down",
            "common_trap" => "Common trap",
            "input_blank" => "Type the missing code",
            "input_output" => "Type the output",
            "current_order" => "Current order",
            "empty_order" => "No step selected yet",
            "reset_order" => "Reset",
            "demo_output" => "Output",
            "quick_guide" => "Source reading module",
            "learning_goals" => "Question mapping",
            "primer_title" => "Pre-question module",
            "book_points" => "Rust Book rule",
            "exercise_link" => "Question focus",
            "answer_hint" => "Answer hint",
            "source_anchor" => "Current source anchor",
            "source_role" => "Source role",
            "source_rule" => "Mapped rule",
            "source_question" => "Question link",
            "cards_title" => "Knowledge Cards",
            "cards_copy" => "The ownership, borrowing, error, and lifetime traps that show up while reading ripgrep source.",
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
                "Start from ripgrep's entry point, argument parsing, and mode dispatch."
            }
            Stage::Ownership => {
                "Trace ownership and borrowing through patterns, process pipes, slices, and writers."
            }
            Stage::Patterns => "Read ripgrep builders, Option/Result boundaries, and collection-heavy config parsing.",
            Stage::Production => {
                "Connect iterators, traits, generics, lifetimes, and threads in production search code."
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
        "syntax-basics" => "Read variables and expressions from ripgrep main",
        "control-flow" => "Read match and guards from ripgrep mode dispatch",
        "data-functions" => "Read function signatures from pattern loading",
        "ownership" => "Read ownership transfer from CommandReader",
        "slices" => "Read &[u8] and &str from pattern bytes",
        "borrowing" => "Read borrowing from ripgrep writers",
        "structs-enums" => "Read structs, impls, and enums from decompression",
        "result-option" => "Read Option and Result from decompressor lookup",
        "collections" => "Read Vec, String, and maps from config parsing",
        "iterators-traits" => "Read iterators and traits from the search pipeline",
        "generics-traits" => "Read generics and lifetimes from globset Candidate",
        "concurrency" => "Read threads and channels from parallel search",
        _ => lesson.title,
    }
}

pub fn lesson_summary(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.summary;
    }
    match lesson.id {
        "syntax-basics" => "Use `crates/core/main.rs` to see how let bindings, match expressions, and exit codes shape the executable.",
        "control-flow" => "Follow ripgrep's `Mode` dispatch to understand exhaustive match arms, guards, and early returns.",
        "data-functions" => "Read `patterns_from_reader` to connect parameters, trait bounds, Vec returns, and `?` propagation.",
        "ownership" => "Inspect `Option::take` on child stdout to see a real resource move instead of an abstract String example.",
        "slices" => "Read byte patterns becoming `&str` and learn why slices borrow data instead of owning it.",
        "borrowing" => "Use the writer implementation to see `&mut self` mutate output state while `&[u8]` is only read.",
        "structs-enums" => "Read the decompression builder and related enum states to see data shape plus behavior.",
        "result-option" => "Use decompressor lookup to separate normal absence (`None`) from recoverable failure (`Result`).",
        "collections" => "Follow config parsing as it pushes owned arguments and errors into growable collections.",
        "iterators-traits" => "Read the search pipeline where walking, filtering, sorting, and searching are chained together.",
        "generics-traits" => "Use globset's `Candidate<'a>` to make trait bounds and lifetimes concrete.",
        "concurrency" => "Read parallel file listing where workers send owned haystacks to a single printer thread.",
        _ => lesson.summary,
    }
}

#[allow(dead_code)]
pub fn lesson_guide(lesson: &Lesson, language: Language) -> &'static [&'static str] {
    match language {
        Language::Zh => match lesson.id {
            "syntax-basics" => &[
                "Rust 默认让绑定不可变：`let x = 5` 之后不能直接改 x。需要原地修改时写 `let mut x = 5`。",
                "`let` 可以 shadowing：重新声明同名变量会创建新绑定，常用于类型转换或临时加工。",
                "没有分号的 block 最后一行是表达式返回值，这是后面函数返回值和 match 表达式的基础。",
            ],
            "control-flow" => &[
                "`if` 和 `match` 在 Rust 里都可以是表达式，因此分支可以直接产出一个值。",
                "`match` 必须覆盖所有可能，这让遗漏状态在编译期就暴露出来。",
                "只关心一种模式时用 `if let`，它牺牲穷尽检查，换来更少样板代码。",
            ],
            "data-functions" => &[
                "标量类型如整数、浮点、布尔和字符通常很直观；复合类型如元组和数组会把多个值组合起来。",
                "数组类型写作 `[T; N]`，长度是类型的一部分；元组可以包含不同类型，并可用模式解构。",
                "函数返回值通常来自函数体最后一个表达式；加分号会把表达式变成语句，返回 `()`。",
            ],
            "ownership" => &[
                "Rust 的每个值都有唯一所有者，所有者离开作用域时值会被释放。",
                "把 `String` 赋给另一个变量会 move 所有权，旧变量不能再用；整数这类 Copy 类型会复制。",
                "需要保留原值时显式 `clone`；只想读取时优先借用，避免不必要的所有权转移。",
            ],
            "slices" => &[
                "切片不是新集合，而是对原集合一段连续区域的借用视图。",
                "字符串切片范围按字节索引计算，`&s[0..5]` 借用 `hello` 这段内容。",
                "`first_word` 返回 `&str` 比返回索引更安全，因为借用关系能阻止原字符串被错误修改。",
            ],
            "borrowing" => &[
                "引用让函数临时访问值而不取得所有权，`&T` 用于读取，`&mut T` 用于修改。",
                "同一时间可以有多个不可变引用，或者一个可变引用，但两者不能同时活跃。",
                "借用检查器关注引用最后一次使用的位置；缩短引用使用范围，往往能让后续可变操作通过。",
            ],
            "structs-enums" => &[
                "struct 适合把相关字段组织成领域对象，字段初始化可以清楚表达每个值的含义。",
                "`impl` 把行为放到类型旁边；只读方法用 `&self`，修改方法用 `&mut self`，消费方法用 `self`。",
                "enum 表达一组有限变体，配合 `match` 能强制你处理每种状态。",
            ],
            "result-option" => &[
                "`Option<T>` 表达可能没有值，替代很多语言里的 null。",
                "`Result<T, E>` 表达可能失败的操作，成功是 `Ok(T)`，失败是 `Err(E)`。",
                "`?` 会在错误时提前返回，在成功时拆出内部值，是错误传播的常用写法。",
            ],
            "collections" => &[
                "`Vec<T>` 是可增长数组，追加元素需要可变绑定。",
                "`String` 是可增长 UTF-8 文本，追加字符串切片常用 `push_str`。",
                "`HashMap::entry(...).or_insert(...)` 适合统计和缓存：存在就复用，不存在才插入。",
            ],
            "iterators-traits" => &[
                "迭代器默认是惰性的，只有 `sum`、`collect`、`for` 这类消费操作才真正执行遍历。",
                "`iter()` 借用元素，`into_iter()` 通常消费集合并交出元素所有权。",
                "trait 描述类型能做什么；生命周期标注描述引用之间的有效期关系，不会延长任何值。",
            ],
            "generics-traits" => &[
                "泛型让函数和类型复用逻辑，但你对泛型值做了什么操作，就要声明相应 trait bound。",
                "复杂约束可以写进 `where` 子句，让签名更可读。",
                "生命周期参数把输入引用和输出引用关联起来，告诉编译器返回值不能活得比来源更久。",
            ],
            "concurrency" => &[
                "`thread::spawn` 的闭包可能比当前函数活得更久，因此捕获外部值时经常需要 `move`。",
                "channel 通过发送值来在线程间转移所有权，发送后原线程不能继续使用该值。",
                "共享可变状态通常用 `Arc<Mutex<T>>`：Arc 负责多所有者，Mutex 负责互斥修改。",
            ],
            _ => &[],
        },
        Language::En => match lesson.id {
            "syntax-basics" => &[
                "Bindings are immutable by default. Use `let mut x = 5` only when the same binding must change.",
                "Shadowing creates a new binding with the same name, which is useful for transformations and type changes.",
                "A block's final line without a semicolon is its return expression, which also explains function returns and match values.",
            ],
            "control-flow" => &[
                "`if` and `match` can be expressions in Rust, so branches can directly produce values.",
                "`match` must cover every possible case, turning missed states into compile-time errors.",
                "Use `if let` when only one pattern matters; it trades exhaustiveness for less boilerplate.",
            ],
            "data-functions" => &[
                "Scalar types are single values; tuples and arrays combine values into fixed compound structures.",
                "Array types are written `[T; N]`, with the length in the type. Tuples can hold different types and be destructured.",
                "Function return values usually come from the final expression. Adding a semicolon turns it into a statement returning `()`.",
            ],
            "ownership" => &[
                "Every value has one owner, and the value is dropped when that owner leaves scope.",
                "Assigning a `String` moves ownership, while small Copy types such as integers are copied.",
                "Use `clone` only when you need another owned copy; otherwise borrow to avoid unnecessary moves.",
            ],
            "slices" => &[
                "A slice is not a new collection; it is a borrowed view into a continuous region of an existing collection.",
                "String slices use byte ranges, so `&s[0..5]` borrows the `hello` part.",
                "Returning `&str` from `first_word` is safer than returning an index because the borrow keeps the source string constrained.",
            ],
            "borrowing" => &[
                "References let code access values without taking ownership: `&T` reads and `&mut T` mutates.",
                "At one time, you can have many immutable references or one mutable reference, but not both active together.",
                "The borrow checker follows the last use of a reference; shortening that use often unlocks later mutation.",
            ],
            "structs-enums" => &[
                "Structs group related fields into domain objects with explicit names.",
                "`impl` keeps behavior next to the type: `&self` reads, `&mut self` mutates, and `self` consumes.",
                "Enums represent a fixed set of variants, and `match` forces every state to be handled.",
            ],
            "result-option" => &[
                "`Option<T>` expresses a value that may be absent, replacing many uses of null.",
                "`Result<T, E>` represents fallible operations: `Ok(T)` for success and `Err(E)` for failure.",
                "The `?` operator returns early on errors and unwraps successful values, making error propagation concise.",
            ],
            "collections" => &[
                "`Vec<T>` is a growable array, and pushing elements requires a mutable binding.",
                "`String` is growable UTF-8 text; `push_str` appends a string slice.",
                "`HashMap::entry(...).or_insert(...)` is ideal for counts and caches: reuse existing entries and insert missing ones.",
            ],
            "iterators-traits" => &[
                "Iterators are lazy until consumed by operations such as `sum`, `collect`, or `for`.",
                "`iter()` borrows elements, while `into_iter()` usually consumes the collection and yields owned elements.",
                "Traits describe capabilities; lifetime annotations describe reference relationships and do not extend values.",
            ],
            "generics-traits" => &[
                "Generics reuse logic, but every operation on a generic value requires the matching trait bound.",
                "Complex bounds often read better in a `where` clause.",
                "Lifetime parameters relate input and output references so returned references cannot outlive their sources.",
            ],
            "concurrency" => &[
                "A `thread::spawn` closure may outlive the current function, so captured values often need `move`.",
                "Channels transfer ownership between threads; after sending a value, the sender cannot use it again.",
                "Shared mutable state commonly uses `Arc<Mutex<T>>`: Arc for multiple owners and Mutex for exclusive mutation.",
            ],
            _ => &[],
        },
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
        "syntax-basics" => "ripgrep: crates/core/main.rs",
        "control-flow" => "ripgrep: Mode dispatch in run",
        "data-functions" => "ripgrep: pattern reader",
        "ownership" => "ripgrep: CommandReader stdout ownership",
        "slices" => "ripgrep: bytes to pattern str",
        "borrowing" => "ripgrep: writer borrowing",
        "structs-enums" => "ripgrep: decompression builder",
        "result-option" => "ripgrep: decompressor lookup",
        "collections" => "ripgrep: config args parser",
        "iterators-traits" => "ripgrep: search iterator pipeline",
        "generics-traits" => "ripgrep: globset Candidate<'a>",
        "concurrency" => "ripgrep: parallel file listing",
        _ => lesson.demo.title,
    }
}

pub fn demo_takeaway(lesson: &Lesson, language: Language) -> &'static str {
    if language == Language::Zh {
        return lesson.demo.takeaway;
    }
    match lesson.id {
        "syntax-basics" => "The ripgrep entry point shows match as the returned expression of `main`, not a toy snippet.",
        "control-flow" => "Mode dispatch uses guards and early returns to choose the actual search path.",
        "data-functions" => "`patterns_from_reader` makes the input reader, collected Vec, and error boundary visible in one signature.",
        "ownership" => "`Option::take` moves a real stdout pipe out before waiting for the child process.",
        "slices" => "The pattern parser borrows bytes and returns a borrowed text view only after UTF-8 validation.",
        "borrowing" => "The writer mutates its own output state while borrowing the incoming byte buffer read-only.",
        "structs-enums" => "The decompression builder stores configuration and exposes behavior through impl methods.",
        "result-option" => "A missing decompressor is `None`; process and I/O failures remain `Result` errors.",
        "collections" => "Config parsing pushes owned OsString values and parse errors into separate Vec collections.",
        "iterators-traits" => "ripgrep describes the walk/search pipeline with iterator adapters and consumes it when searching.",
        "generics-traits" => "Candidate<'a> ties borrowed path data to the input lifetime while staying generic over path-like values.",
        "concurrency" => "Parallel workers send owned haystacks to one printing thread through a channel.",
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
        "syntax-shadowing" => "Shadowing creates a new binding",
        "syntax-println-placeholder" => "println! captures variable names",
        "syntax-semicolon-unit" => "A semicolon turns an expression into a statement",
        "syntax-const-binding" => "const needs an explicit type",
        "syntax-shadow-mutability" => "Shadowing does not need mut",
        "syntax-block-scope" => "Blocks have their own scope",
        "tuple-destructure" => "Destructure a tuple",
        "function-return" => "Return the last expression",
        "array-type" => "Array types include length",
        "statement-vs-expression" => "Statements do not return values",
        "function-param-type" => "Function parameters need types",
        "tuple-index" => "Access tuple elements with dot syntax",
        "array-repeat-init" => "Repeat-initialize an array",
        "function-explicit-return" => "return exits early",
        "match-exhaustive" => "Why match needs a fallback",
        "if-let-some" => "Care only about Some",
        "match-return-type" => "match arms need compatible types",
        "if-expression-value" => "if can produce a value",
        "match-option-none" => "match handles None",
        "match-guard" => "Add a condition with a match guard",
        "if-let-else" => "if let can have else",
        "while-let-pop" => "Loop with while let",
        "ownership-move" => "Find the invalid use after move",
        "ownership-clone" => "Keep two Strings",
        "ownership-copy-trap" => "Copy or move?",
        "ownership-borrow-read" => "Borrow to read without moving",
        "ownership-function-takes" => "Function parameters can take ownership",
        "ownership-return-ownership" => "Return ownership to the caller",
        "ownership-borrow-vs-clone" => "Borrow instead of cloning",
        "ownership-copy-types" => "Which types are usually Copy?",
        "slice-range" => "Take the hello slice",
        "first-word-return" => "What should first_word return?",
        "string-literal-slice" => "Type of a string literal",
        "slice-borrow-blocks-clear" => "A slice borrow blocks mutation",
        "slice-open-ended-range" => "Omit a range boundary",
        "slice-full-range" => "Full-range slice",
        "slice-string-param" => "Prefer &str parameters",
        "slice-utf8-boundary" => "String slices need UTF-8 boundaries",
        "borrowing-mut-ref" => "Mutate through a mutable borrow",
        "borrowing-rule" => "Borrowing rule in one sentence",
        "borrow-scope-release" => "Shorten a borrow's active scope",
        "dangling-reference" => "Why dangling references are rejected",
        "borrow-immutable-many" => "Many immutable borrows can coexist",
        "borrow-mut-exclusive-error" => "Mutable borrows must be exclusive",
        "borrow-reborrow-shared" => "Read and then mutate through one mutable reference",
        "borrow-function-mut-param" => "A function mutates the caller's value",
        "struct-update" => "Struct update syntax",
        "method-self" => "Read-only methods borrow self",
        "enum-match-option" => "Option is an enum",
        "struct-field-init-shorthand" => "Field init shorthand",
        "method-mut-self" => "Mutating methods borrow mut self",
        "tuple-struct-access" => "Access tuple struct fields",
        "enum-variant-data" => "Enum variants can carry data",
        "match-enum-method" => "match self inside a method",
        "option-match" => "Handle None safely",
        "result-question-mark" => "What ? really means",
        "result-match-ok-err" => "Result handles success and failure",
        "option-unwrap-risk" => "The risk of unwrap",
        "question-mark-return-type" => "? needs a compatible return type",
        "result-unwrap-or" => "unwrap_or provides a default",
        "option-map" => "Transform an Option with map",
        "question-mark-option" => "? can propagate None",
        "vec-mut-push" => "Vec push needs mutability",
        "string-update" => "Append a string slice",
        "hashmap-entry" => "Insert only when the key is absent",
        "vec-index-bounds" => "get is safer than indexing",
        "hashmap-count-entry" => "Count words with entry",
        "vec-iterate-borrow" => "Borrow while iterating a Vec",
        "string-push-char" => "push appends one character",
        "hashmap-insert-overwrite" => "insert overwrites old values",
        "iterator-chain" => "Iterator chain result",
        "lifetime-meaning" => "What lifetime annotations mean",
        "iter-vs-into-iter" => "iter vs into_iter ownership",
        "collect-type" => "collect needs a target type",
        "iterator-lazy" => "Iterator adapters are lazy",
        "iterator-filter-borrow" => "filter receives references",
        "iterator-sum-consumes" => "sum consumes the iterator",
        "iterator-enumerate" => "enumerate yields indexes and values",
        "generic-largest" => "largest needs a capability bound",
        "trait-bound-display" => "Trait bound for printing",
        "lifetime-longest" => "The lifetime of longest's return",
        "where-clause" => "Use a where clause",
        "static-lifetime-myth" => "'static is not magic",
        "impl-trait-param" => "impl Trait simplifies parameter bounds",
        "derive-debug-bound" => "Debug printing needs Debug",
        "lifetime-elision" => "Lifetime elision rules",
        "thread-move" => "Why a thread closure needs move",
        "channel-send" => "Sending through a channel moves values",
        "mutex-lock" => "Shared mutable counter",
        "arc-clone" => "Arc::clone clones the pointer",
        "mutex-guard-drop" => "When MutexGuard releases the lock",
        "thread-join" => "join waits for a thread",
        "channel-recv-block" => "recv blocks while waiting",
        "mutex-poison-unwrap" => "lock returns Result",
        "syntax-type-inference" => "Type inference is not dynamic typing",
        "syntax-mut-reassign" => "mut allows reassignment with the same type",
        "syntax-const-uppercase" => "Constant naming convention",
        "syntax-expression-parentheses" => "Parenthesized expression value",
        "match-literal-pattern" => "Literal patterns",
        "match-binding-value" => "Pattern binding",
        "if-else-no-semicolon" => "Assigning an if expression",
        "loop-break-value" => "loop break can return a value",
        "integer-overflow-debug" => "Integer overflow in debug builds",
        "float-default-f64" => "Default floating-point type",
        "char-four-bytes" => "char is a Unicode scalar value",
        "function-implicit-unit" => "Functions default to unit",
        "ownership-drop-at-scope-end" => "drop at the end of scope",
        "ownership-reassign-drops-old" => "Reassignment drops the old value",
        "ownership-tuple-move-field" => "Moving a String field out of a tuple",
        "ownership-reference-no-drop" => "References do not own values",
        "slice-array-slice" => "Array slice type",
        "slice-len-method" => "Slices have len",
        "slice-mut-slice" => "Mutable slices",
        "slice-first-word-signature" => "A more general first_word signature",
        "borrow-last-use-nll" => "Borrows end after last use",
        "borrow-mut-then-read-owner" => "Read owner after mutable borrow ends",
        "borrow-cannot-move-while-borrowed" => "Cannot move while borrowed",
        "borrow-mut-argument-call" => "Calling a mutable-reference parameter",
        "struct-associated-function" => "Associated functions do not take self",
        "struct-debug-print" => "Debug-printing a struct",
        "enum-if-let-method" => "if let matches an enum variant",
        "method-takes-self" => "Methods that take self",
        "result-map-err" => "map_err transforms errors",
        "result-ok-method" => "Result::ok converts to Option",
        "option-and-then" => "Chaining Option with and_then",
        "result-expect-message" => "expect provides a panic message",
        "vec-new-type-annotation" => "Vec::new may need a type",
        "vec-macro-init" => "Initialize with vec! macro",
        "string-from-vs-literal" => "String::from vs string literal",
        "hashmap-get-option" => "HashMap::get returns Option",
        "iterator-any" => "any short-circuits",
        "iterator-find" => "find returns Option",
        "iterator-collect-string" => "Collect into String",
        "iterator-closure-capture" => "Iterator closures capture environment",
        "trait-default-method" => "Trait default methods",
        "trait-impl-for-type" => "Implement a trait for a type",
        "generic-multiple-bounds" => "Multiple trait bounds",
        "lifetime-struct-ref" => "Structs holding references need lifetimes",
        "thread-spawn-return" => "Thread closures can return values",
        "channel-multiple-send" => "Sending multiple messages",
        "arc-needed-not-rc" => "Use Arc instead of Rc across threads",
        "mutex-scope-release-early" => "Release a lock earlier with scope",
        "advanced-syntax-numeric-suffix" => "Advanced: numeric literal suffixes",
        "advanced-syntax-never-semicolon" => "Advanced: panic and the never type",
        "advanced-syntax-macro-vs-function" => "Advanced: macro calls use exclamation marks",
        "advanced-control-destructure-enum" => "Advanced: destructuring enum data",
        "advanced-control-at-binding" => "Advanced: @ bindings",
        "advanced-control-matches-macro" => "Advanced: matches! macro",
        "advanced-data-diverging-return" => "Advanced: diverging functions",
        "advanced-data-array-copy-init" => "Advanced: repeated array initialization and Copy",
        "advanced-data-tuple-trailing-comma" => "Advanced: one-element tuples need a comma",
        "advanced-ownership-partial-move-struct" => "Advanced: partial moves from structs",
        "advanced-ownership-mem-take" => "Advanced: mem::take moves out with a default",
        "advanced-ownership-closure-move" => "Advanced: move closures capture ownership",
        "advanced-slice-split-at-mut" => "Advanced: split_at_mut returns disjoint mutable slices",
        "advanced-slice-pattern-match" => "Advanced: slice pattern matching",
        "advanced-slice-as-bytes" => "Advanced: as_bytes reveals UTF-8 bytes",
        "advanced-borrow-two-phase" => "Advanced: two-phase borrows",
        "advanced-borrow-reborrow-mut" => "Advanced: mutable reborrowing",
        "advanced-borrow-interior-mutability" => "Advanced: RefCell checks borrows at runtime",
        "advanced-struct-enum-size" => "Advanced: enum size and largest variant",
        "advanced-struct-option-niche" => "Advanced: niche optimization for Option references",
        "advanced-struct-match-ref" => "Advanced: borrow fields in patterns",
        "advanced-result-transpose" => "Advanced: transpose flips Option<Result>",
        "advanced-result-ok-or-else" => "Advanced: ok_or_else lazily builds errors",
        "advanced-result-thiserror-idea" => "Advanced: libraries return errors, not panics",
        "advanced-collections-entry-and-modify" => "Advanced: entry().and_modify",
        "advanced-collections-drain" => "Advanced: drain removes a range",
        "advanced-collections-hashmap-key-move" => "Advanced: HashMap insert moves the key",
        "advanced-iterator-flat-map" => "Advanced: flat_map flattens iterators",
        "advanced-iterator-by-ref" => "Advanced: by_ref temporarily borrows an iterator",
        "advanced-iterator-partition" => "Advanced: partition splits into two collections",
        "advanced-generics-associated-type" => "Advanced: associated type Iterator::Item",
        "advanced-generics-blanket-impl" => "Advanced: blanket implementations",
        "advanced-generics-phantom-lifetime" => {
            "Advanced: PhantomData expresses type relationships"
        }
        "advanced-concurrency-send-sync" => "Advanced: Send vs Sync",
        "advanced-concurrency-mpsc-clone-sender" => "Advanced: clone Sender for multiple producers",
        "advanced-concurrency-deadlock-order" => "Advanced: inconsistent lock order can deadlock",
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
        "syntax-shadowing" => "What does this shadowing code print?",
        "syntax-println-placeholder" => {
            "In Rust 2021 formatting style, what fills the blank to print name?"
        }
        "syntax-semicolon-unit" => "Which block has the value `()`?",
        "tuple-destructure" => "Following The Rust Book tuple example, what pattern binds all three values?",
        "function-return" => "What does this function call print?",
        "array-type" => "Which type means an array of five i32 values?",
        "statement-vs-expression" => "Why can this function not return i32 as written?",
        "function-param-type" => "Rust function parameters need explicit types. What fills the blank?",
        "match-exhaustive" => "Which statement is correct?",
        "if-let-some" => "Which pattern prints the value only when maybe is Some?",
        "match-return-type" => "Why does this code fail to compile?",
        "if-expression-value" => "What does this if expression print?",
        "match-option-none" => "What match arm covers the no-value case?",
        "ownership-move" => "Which line fails to compile?",
        "ownership-clone" => {
            "What should fill the blank if both name and alias must remain usable?"
        }
        "ownership-copy-trap" => "After which assignment can the original variable still be used?",
        "ownership-borrow-read" => "What fills the call if we only want to read name without moving it?",
        "ownership-function-takes" => "Why can s not be printed after calling takes(s)?",
        "slice-range" => "Following The Rust Book string slice example, what range extracts hello?",
        "first-word-return" => "Why does The Rust Book change first_word to return &str instead of usize?",
        "string-literal-slice" => "What is the most accurate type of the string literal \"hello\"?",
        "slice-borrow-blocks-clear" => "Why can't we clear the original String while word is still used?",
        "slice-open-ended-range" => "What fills the blank to slice from index 6 to the end?",
        "borrowing-mut-ref" => "Put the steps in an order that compiles and mutates the Vec.",
        "borrowing-rule" => "Which sentence is most accurate?",
        "borrow-scope-release" => "Put the steps in an order that reads and then mutates a String.",
        "dangling-reference" => "Why does this function fail to compile?",
        "borrow-immutable-many" => "Why does this code compile?",
        "struct-update" => "What fills the blank to reuse the remaining fields from user1?",
        "method-self" => "If a method only reads struct fields and does not take ownership, what does it usually take?",
        "enum-match-option" => "What match arm extracts the Some value and adds 1?",
        "struct-field-init-shorthand" => "When the variable name and field name match, what shorthand fills the blank?",
        "method-mut-self" => "When a method mutates a field, what should the self parameter usually be?",
        "option-match" => "What branch safely handles None?",
        "result-question-mark" => "What does ? do on Result?",
        "result-match-ok-err" => "What branch returns 0 when parsing fails?",
        "option-unwrap-risk" => "What happens if you call unwrap() on None?",
        "question-mark-return-type" => "Why can't this function use `?` directly?",
        "vec-mut-push" => "What fills the blank if we want to append to a Vec?",
        "string-update" => "Which method appends `bar` to a String?",
        "hashmap-entry" => "Following The Rust Book entry pattern, what fills the blank?",
        "vec-index-bounds" => "Which access style is safer when the Vec index may be out of bounds?",
        "hashmap-count-entry" => "Following the word-count pattern, what fills the blank to add 1?",
        "iterator-chain" => "What does this code print?",
        "lifetime-meaning" => "What do lifetime annotations actually express?",
        "iter-vs-into-iter" => "After calling `into_iter()` on a `Vec<String>`, what happens to the original vector?",
        "collect-type" => "What fills the blank to collect into `Vec<i32>`?",
        "iterator-lazy" => "Why does the println! inside map not run when only map is called?",
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
        "syntax-const-binding" => "When defining a compile-time constant, what type annotation fills the blank?",
        "syntax-shadow-mutability" => "Why doesn't this code need spaces to be declared mut?",
        "syntax-block-scope" => "What does this code print?",
        "match-guard" => "What guard fills the blank to match only positive numbers?",
        "if-let-else" => "What does this code print?",
        "while-let-pop" => "When does `while let Some(top) = stack.pop()` stop?",
        "tuple-index" => "What fills the blank to access the tuple's second element?",
        "array-repeat-init" => "What fills the blank to create an array of five 3 values?",
        "function-explicit-return" => "What does this function call print?",
        "ownership-return-ownership" => "If a function takes a String and gives it back, what fills the blank?",
        "ownership-borrow-vs-clone" => "When you only need to read a string's length, which call is usually better?",
        "ownership-copy-types" => "Which group of values usually remains usable after assignment?",
        "slice-full-range" => "What range borrows the entire string slice?",
        "slice-string-param" => "For a read-only string function, which parameter type is usually more flexible?",
        "slice-utf8-boundary" => "Why can slicing a Chinese string by arbitrary byte ranges panic?",
        "borrow-mut-exclusive-error" => "Why does this code fail to compile?",
        "borrow-reborrow-shared" => "What does this code print?",
        "borrow-function-mut-param" => "If a function mutates the caller's String, what parameter type fills the blank?",
        "tuple-struct-access" => "What fills the blank to access Color's first field?",
        "enum-variant-data" => "What does `IpAddr::V4(127, 0, 0, 1)` show?",
        "match-enum-method" => "If a method only reads an enum and matches variants, what usually fills the blank?",
        "result-unwrap-or" => "What does this code print?",
        "option-map" => "What does this code print?",
        "question-mark-option" => "In a function returning Option, what does `?` do on None?",
        "vec-iterate-borrow" => "If values must remain usable after the loop, what fills the blank?",
        "string-push-char" => "Which method appends one exclamation mark character to a String?",
        "hashmap-insert-overwrite" => "What does this code print at the end?",
        "iterator-filter-borrow" => "When filtering the result of `iter()`, what expression fills the blank?",
        "iterator-sum-consumes" => "Why can't iter be used after calling `iter.sum()`?",
        "iterator-enumerate" => "What does this code print?",
        "impl-trait-param" => "If a parameter accepts any Display value, what fills the blank?",
        "derive-debug-bound" => "If a generic function prints T with `{:?}`, what trait bound is needed?",
        "lifetime-elision" => "Why does `fn first_word(s: &str) -> &str` usually not need written lifetimes?",
        "thread-join" => "What is the main purpose of `handle.join().unwrap()`?",
        "channel-recv-block" => "What does `rx.recv()` do while no message is available but a sender still exists?",
        "mutex-poison-unwrap" => "Mutex::lock returns Result; what method usually extracts the guard in examples?",
        "syntax-type-inference" => "Why can x not be assigned a string after `let x = 5;`?",
        "syntax-mut-reassign" => "What does this code print?",
        "syntax-const-uppercase" => "Which naming style is commonly used for Rust constants?",
        "syntax-expression-parentheses" => "What does this code print?",
        "match-literal-pattern" => "Which arm matches when x is 2?",
        "match-binding-value" => "What fills the blank to bind the matched number?",
        "if-else-no-semicolon" => "Why can this code assign a result to label?",
        "loop-break-value" => "What does this code print?",
        "integer-overflow-debug" => "What usually happens when u8 exceeds 255 in a debug build?",
        "float-default-f64" => "What floating-point type is inferred for `let x = 2.0;` by default?",
        "char-four-bytes" => "What does Rust char most accurately represent?",
        "function-implicit-unit" => "What does a function return when it has no return type and no tail expression?",
        "ownership-drop-at-scope-end" => "When is the memory owned by s released?",
        "ownership-reassign-drops-old" => "What happens to the old String when assigning a new one?",
        "ownership-tuple-move-field" => "Which statement is correct?",
        "ownership-reference-no-drop" => "Why does leaving r's scope not drop the String?",
        "slice-array-slice" => "What is the most accurate type of `&numbers[1..3]`?",
        "slice-len-method" => "What does this code print?",
        "slice-mut-slice" => "What fills the blank to mutate an array through a slice?",
        "slice-first-word-signature" => "What parameter type lets first_word accept String slices and string literals?",
        "borrow-last-use-nll" => "Why does this code compile?",
        "borrow-mut-then-read-owner" => "What does this code print?",
        "borrow-cannot-move-while-borrowed" => "Why does this code fail to compile?",
        "borrow-mut-argument-call" => "What fills the blank when calling a function that needs `&mut String`?",
        "struct-associated-function" => "Why is a function like `String::from(\"hi\")` called with `::`?",
        "struct-debug-print" => "What is usually needed to print a custom struct with `{:?}`?",
        "enum-if-let-method" => "What fills the blank if we only care about the Quit variant?",
        "method-takes-self" => "What does the signature `fn into_inner(self)` mean?",
        "result-map-err" => "What does map_err do?",
        "result-ok-method" => "What does this code print?",
        "option-and-then" => "What is the key difference between and_then and map?",
        "result-expect-message" => "What does `expect(\"config missing\")` add compared with unwrap?",
        "vec-new-type-annotation" => "What fills the blank when an empty Vec has no elements for inference?",
        "vec-macro-init" => "What does this code print?",
        "string-from-vs-literal" => "What is the main difference between `String::from(\"hi\")` and `\"hi\"`?",
        "hashmap-get-option" => "Why does `scores.get(\"Blue\")` return Option?",
        "iterator-any" => "What does this code print?",
        "iterator-find" => "Why does find return Option?",
        "iterator-collect-string" => "What does this code print?",
        "iterator-closure-capture" => "What does this code print?",
        "trait-default-method" => "What does a trait method with a body mean?",
        "trait-impl-for-type" => "What keyword fills the blank to implement Summary for NewsArticle?",
        "generic-multiple-bounds" => "What connector fills the blank when T needs Display and Clone?",
        "lifetime-struct-ref" => "Why does this struct need a lifetime parameter?",
        "thread-spawn-return" => "What does this code print?",
        "channel-multiple-send" => "What does this code print?",
        "arc-needed-not-rc" => "Why not use `Rc<Mutex<T>>` for a shared counter across threads?",
        "mutex-scope-release-early" => "Why wrap a MutexGuard in an extra block?",
        "advanced-syntax-numeric-suffix" => "What does the `u8` suffix do in `let x = 10u8;`?",
        "advanced-syntax-never-semicolon" => "Why can `let x: i32 = panic!(\"boom\");` type-check?",
        "advanced-syntax-macro-vs-function" => "What symbol must follow println when calling the macro?",
        "advanced-control-destructure-enum" => "What pattern extracts fields x and y?",
        "advanced-control-at-binding" => "What does `id @ 3..=7` mean?",
        "advanced-control-matches-macro" => "What does this code print?",
        "advanced-data-diverging-return" => "What does `fn fail() -> !` mean?",
        "advanced-data-array-copy-init" => "Why does `[String::from(\"x\"); 3]` not work?",
        "advanced-data-tuple-trailing-comma" => "What is the difference between `(5,)` and `(5)`?",
        "advanced-ownership-partial-move-struct" => "After moving `person.name`, what is usually still usable?",
        "advanced-ownership-mem-take" => "What is the core effect of `std::mem::take(&mut s)`?",
        "advanced-ownership-closure-move" => "What does `move || s.len()` do to String s?",
        "advanced-slice-split-at-mut" => "Why can split_at_mut return two mutable slices?",
        "advanced-slice-pattern-match" => "What does the slice pattern `[first, .., last]` mean?",
        "advanced-slice-as-bytes" => "What does this code print?",
        "advanced-borrow-two-phase" => "Why does `v.push(v.len())` usually compile?",
        "advanced-borrow-reborrow-mut" => "Why can the original mutable reference be used after passing it to a function?",
        "advanced-borrow-interior-mutability" => "What is the key difference between RefCell<T> and normal borrowing?",
        "advanced-struct-enum-size" => "Why must an enum value be able to hold its largest variant?",
        "advanced-struct-option-niche" => "Why is `Option<&T>` usually the same size as `&T`?",
        "advanced-struct-match-ref" => "What pattern keyword borrows name instead of moving it?",
        "advanced-result-transpose" => "What does `Option<Result<T,E>>::transpose()` produce?",
        "advanced-result-ok-or-else" => "What is the advantage of ok_or_else over ok_or?",
        "advanced-result-thiserror-idea" => "What design is preferred for recoverable failures in library functions?",
        "advanced-collections-entry-and-modify" => "What fills the blank to increment an existing count and insert 1 otherwise?",
        "advanced-collections-drain" => "What is the core effect of `v.drain(1..3)`?",
        "advanced-collections-hashmap-key-move" => "Why is key usually unusable after `map.insert(key, 1)`?",
        "advanced-iterator-flat-map" => "What does this code print?",
        "advanced-iterator-by-ref" => "What does this code print?",
        "advanced-iterator-partition" => "What does partition return?",
        "advanced-generics-associated-type" => "What is `Item` in `Iterator<Item = i32>`?",
        "advanced-generics-blanket-impl" => "What is an impl like `impl<T: Display> ToString for T` called?",
        "advanced-generics-phantom-lifetime" => "What is `PhantomData<&'a T>` often used to express?",
        "advanced-concurrency-send-sync" => "What does `T: Send` most accurately mean?",
        "advanced-concurrency-mpsc-clone-sender" => "Why clone tx in mpsc examples?",
        "advanced-concurrency-deadlock-order" => "What is the main risk when two threads lock two mutexes in opposite orders?",
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
        ("syntax-semicolon-unit", 0) => "the block for `a`",
        ("syntax-semicolon-unit", 1) => "the block for `b`",
        ("syntax-semicolon-unit", 2) => "both blocks are 42",
        ("array-type", 0) => "[i32; 5]",
        ("array-type", 1) => "Vec<i32>",
        ("array-type", 2) => "(i32, 5)",
        ("statement-vs-expression", 0) => {
            "`42;` is a statement, so the function actually returns `()`"
        }
        ("statement-vs-expression", 1) => "i32 cannot be used as a return type",
        ("statement-vs-expression", 2) => "functions must always write return",
        ("match-return-type", 0) => "the two arms return different types",
        ("match-return-type", 1) => "match cannot match bool",
        ("match-return-type", 2) => "the false arm can never run",
        ("ownership-move", 0) => "`let name = String::from(\"Rust\");`",
        ("ownership-move", 1) => "`let alias = name;`",
        ("ownership-move", 2) => "`println!(\"{name}\");`",
        ("ownership-copy-trap", 0) => "`a` is still usable because i32 implements Copy",
        ("ownership-copy-trap", 1) => "`s1` is still usable because String automatically copies",
        ("ownership-copy-trap", 2) => "Neither original variable can be used",
        ("ownership-function-takes", 0) => "ownership of s moves into the parameter value",
        ("ownership-function-takes", 1) => "println! can only print a String once",
        ("ownership-function-takes", 2) => "function calls automatically clone s",
        ("first-word-return", 0) => {
            "&str ties the result to the source borrow, avoiding stale indexes"
        }
        ("first-word-return", 1) => "&str copies the first word and is faster",
        ("first-word-return", 2) => "&str bypasses UTF-8 checks",
        ("string-literal-slice", 0) => "&str",
        ("string-literal-slice", 1) => "String",
        ("string-literal-slice", 2) => "[char; 5]",
        ("slice-borrow-blocks-clear", 0) => {
            "word still borrows s, while clear needs a mutable borrow"
        }
        ("slice-borrow-blocks-clear", 1) => "clear deletes the variable name",
        ("slice-borrow-blocks-clear", 2) => "a slice copies the string, so it conflicts",
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
        ("borrow-immutable-many", 0) => {
            "many immutable references only read, so they cannot corrupt the data"
        }
        ("borrow-immutable-many", 1) => "String was automatically copied twice",
        ("borrow-immutable-many", 2) => "println! releases all references",
        ("method-self", 0) => "&self",
        ("method-self", 1) => "&mut self",
        ("method-self", 2) => "self",
        ("method-mut-self", 0) => "&mut self",
        ("method-mut-self", 1) => "&self",
        ("method-mut-self", 2) => "self: &Self",
        ("result-question-mark", 0) => {
            "On Ok, unwrap the value; on Err, return Err from the current function"
        }
        ("result-question-mark", 1) => "Catch the error and keep running the next line",
        ("result-question-mark", 2) => "Convert Err into None automatically",
        ("option-unwrap-risk", 0) => "runtime panic",
        ("option-unwrap-risk", 1) => "return 0",
        ("option-unwrap-risk", 2) => "the compiler inserts a default value automatically",
        ("question-mark-return-type", 0) => {
            "the function returns i32, so it cannot carry an early Err"
        }
        ("question-mark-return-type", 1) => "parse cannot be used with ?",
        ("question-mark-return-type", 2) => "? can only appear in main",
        ("vec-index-bounds", 0) => "get returns Option and does not panic on out-of-bounds access",
        ("vec-index-bounds", 1) => "values[99] automatically returns the last element",
        ("vec-index-bounds", 2) => "Vec does not allow indexed access",
        ("iterator-lazy", 0) => {
            "map is lazy; collect, sum, for, or another consumer must pull values"
        }
        ("iterator-lazy", 1) => "println! cannot be written inside map",
        ("iterator-lazy", 2) => "iter must be declared mut before it runs",
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
        ("syntax-shadow-mutability", 0) => {
            "the second line creates a new binding with the same name"
        }
        ("syntax-shadow-mutability", 1) => "string literals are mutable by default",
        ("syntax-shadow-mutability", 2) => "usize automatically mutates the original string",
        ("while-let-pop", 0) => "when pop returns None",
        ("while-let-pop", 1) => "when top equals 0",
        ("while-let-pop", 2) => "after the loop body runs once",
        ("ownership-borrow-vs-clone", 0) => "len(&name)",
        ("ownership-borrow-vs-clone", 1) => "len(&name.clone())",
        ("ownership-borrow-vs-clone", 2) => "len(name)",
        ("ownership-copy-types", 0) => "bool, char, and integers",
        ("ownership-copy-types", 1) => "String and Vec",
        ("ownership-copy-types", 2) => "all owned types behave the same",
        ("slice-string-param", 0) => "&str",
        ("slice-string-param", 1) => "&String",
        ("slice-string-param", 2) => "String",
        ("slice-utf8-boundary", 0) => "the range does not land on UTF-8 character boundaries",
        ("slice-utf8-boundary", 1) => "String cannot be sliced",
        ("slice-utf8-boundary", 2) => "Chinese text cannot be stored in String",
        ("borrow-mut-exclusive-error", 0) => {
            "two active mutable references cannot exist at the same time"
        }
        ("borrow-mut-exclusive-error", 1) => "String cannot be mutably borrowed",
        ("borrow-mut-exclusive-error", 2) => "println! cannot print references",
        ("enum-variant-data", 0) => "enum variants can carry different types and amounts of data",
        ("enum-variant-data", 1) => "enums can only store numbers",
        ("enum-variant-data", 2) => "V4 and V6 must have exactly the same fields",
        ("question-mark-option", 0) => "return None early from the current function",
        ("question-mark-option", 1) => "convert None into 0",
        ("question-mark-option", 2) => "panic",
        ("iterator-sum-consumes", 0) => "sum takes ownership of the iterator and consumes it",
        ("iterator-sum-consumes", 1) => "sum clears the original Vec",
        ("iterator-sum-consumes", 2) => "i32 values cannot be summed",
        ("derive-debug-bound", 0) => "Debug",
        ("derive-debug-bound", 1) => "Display",
        ("derive-debug-bound", 2) => "Clone",
        ("lifetime-elision", 0) => {
            "there is one input reference, so the compiler can infer the output comes from it"
        }
        ("lifetime-elision", 1) => "lifetimes no longer exist",
        ("lifetime-elision", 2) => "&str is always static",
        ("thread-join", 0) => "wait for the child thread to finish",
        ("thread-join", 1) => "force-kill the child thread",
        ("thread-join", 2) => "turn the thread into an async task",
        ("channel-recv-block", 0) => "block and wait for a message",
        ("channel-recv-block", 1) => "return an empty string immediately",
        ("channel-recv-block", 2) => "create a default message automatically",
        ("syntax-type-inference", 0) => {
            "x has been inferred as an integer, so it cannot become &str"
        }
        ("syntax-type-inference", 1) => "mut variables cannot be reassigned",
        ("syntax-type-inference", 2) => "strings cannot be assigned to variables",
        ("syntax-const-uppercase", 0) => "SCREAMING_SNAKE_CASE",
        ("syntax-const-uppercase", 1) => "camelCase",
        ("syntax-const-uppercase", 2) => "kebab-case",
        ("match-literal-pattern", 0) => "`2 => \"two\"`",
        ("match-literal-pattern", 1) => "`1 => \"one\"`",
        ("match-literal-pattern", 2) => "`_ => \"other\"`",
        ("if-else-no-semicolon", 0) => "both branches are tail expressions producing &str",
        ("if-else-no-semicolon", 1) => "if can only return bool",
        ("if-else-no-semicolon", 2) => "the semicolon turns label into a string",
        ("integer-overflow-debug", 0) => "panic",
        ("integer-overflow-debug", 1) => "automatically become u16",
        ("integer-overflow-debug", 2) => "produce the string \"256\"",
        ("float-default-f64", 0) => "f64",
        ("float-default-f64", 1) => "f32",
        ("float-default-f64", 2) => "i32",
        ("char-four-bytes", 0) => "a Unicode scalar value, four bytes wide",
        ("char-four-bytes", 1) => "an ASCII byte",
        ("char-four-bytes", 2) => "a string slice",
        ("function-implicit-unit", 0) => "()",
        ("function-implicit-unit", 1) => "String",
        ("function-implicit-unit", 2) => "i32",
        ("ownership-drop-at-scope-end", 0) => "automatically dropped when leaving scope",
        ("ownership-drop-at-scope-end", 1) => "immediately after creation",
        ("ownership-drop-at-scope-end", 2) => "only after manual free",
        ("ownership-reassign-drops-old", 0) => "the old value is dropped and s owns the new value",
        ("ownership-reassign-drops-old", 1) => "s owns both old and new",
        ("ownership-reassign-drops-old", 2) => "the old value leaks on the heap",
        ("ownership-tuple-move-field", 0) => {
            "pair.0 is moved out, so pair is no longer fully usable"
        }
        ("ownership-tuple-move-field", 1) => "pair is fully copied",
        ("ownership-tuple-move-field", 2) => "the i32 must also become unusable",
        ("ownership-reference-no-drop", 0) => "r is only a reference and does not own the String",
        ("ownership-reference-no-drop", 1) => "references clone Strings",
        ("ownership-reference-no-drop", 2) => "println! recreates s",
        ("slice-array-slice", 0) => "&[i32]",
        ("slice-array-slice", 1) => "Vec<i32>",
        ("slice-array-slice", 2) => "[i32; 2]",
        ("borrow-last-use-nll", 0) => "the immutable borrow ends after r's last use",
        ("borrow-last-use-nll", 1) => "push_str does not need a mutable borrow",
        ("borrow-last-use-nll", 2) => "println! clones s",
        ("borrow-cannot-move-while-borrowed", 0) => {
            "s is still borrowed by r, so ownership cannot move"
        }
        ("borrow-cannot-move-while-borrowed", 1) => "String can never move",
        ("borrow-cannot-move-while-borrowed", 2) => "the reference must be mut",
        ("struct-associated-function", 0) => "it is an associated function and takes no self",
        ("struct-associated-function", 1) => "it mutates an existing String",
        ("struct-associated-function", 2) => "it can only be used in match",
        ("struct-debug-print", 0) => "derive or implement Debug",
        ("struct-debug-print", 1) => "derive Display",
        ("struct-debug-print", 2) => "turn the struct into a tuple",
        ("method-takes-self", 0) => "calling the method consumes Wrapper",
        ("method-takes-self", 1) => "the method only reads Wrapper",
        ("method-takes-self", 2) => "the method borrows and mutates Wrapper",
        ("result-map-err", 0) => "transform only the error inside Err",
        ("result-map-err", 1) => "transform only the success inside Ok",
        ("result-map-err", 2) => "ignore every error",
        ("option-and-then", 0) => "the closure returns Option, avoiding nested Option",
        ("option-and-then", 1) => "and_then panics",
        ("option-and-then", 2) => "and_then only works on Result",
        ("result-expect-message", 0) => "a custom panic message",
        ("result-expect-message", 1) => "automatic error recovery",
        ("result-expect-message", 2) => "conversion from Err to None",
        ("string-from-vs-literal", 0) => "String owns a growable heap-allocated string",
        ("string-from-vs-literal", 1) => "String is always static",
        ("string-from-vs-literal", 2) => "string literals can push_str",
        ("hashmap-get-option", 0) => "the key may be absent",
        ("hashmap-get-option", 1) => "HashMap can only store Option",
        ("hashmap-get-option", 2) => "get deletes the key",
        ("iterator-find", 0) => "there may be no matching item",
        ("iterator-find", 1) => "find returns a Vec",
        ("iterator-find", 2) => "find always panics",
        ("trait-default-method", 0) => {
            "a default implementation that implementors can use or override"
        }
        ("trait-default-method", 1) => "every impl must rewrite it",
        ("trait-default-method", 2) => "traits cannot have method bodies",
        ("lifetime-struct-ref", 0) => {
            "it stores a reference, so the validity relationship must be stated"
        }
        ("lifetime-struct-ref", 1) => "all structs need lifetimes",
        ("lifetime-struct-ref", 2) => "&str cannot be a field",
        ("arc-needed-not-rc", 0) => "Rc reference counting is not thread-safe",
        ("arc-needed-not-rc", 1) => "Mutex only works with Rc",
        ("arc-needed-not-rc", 2) => "Arc is single-thread-only",
        ("mutex-scope-release-early", 0) => {
            "to make the guard leave scope and release the lock earlier"
        }
        ("mutex-scope-release-early", 1) => "to make Mutex Copy",
        ("mutex-scope-release-early", 2) => "to avoid compiler checks",
        ("advanced-syntax-numeric-suffix", 0) => "specify the concrete integer type of the literal",
        ("advanced-syntax-numeric-suffix", 1) => "convert the number to a string",
        ("advanced-syntax-numeric-suffix", 2) => "make the variable mutable",
        ("advanced-syntax-never-semicolon", 0) => {
            "panic has the never type and can coerce to any type"
        }
        ("advanced-syntax-never-semicolon", 1) => "panic returns 0",
        ("advanced-syntax-never-semicolon", 2) => "i32 ignores the right side",
        ("advanced-control-at-binding", 0) => "match 3 through 7 and bind the actual value to id",
        ("advanced-control-at-binding", 1) => "match only the number 3",
        ("advanced-control-at-binding", 2) => "create a variable named 3..=7",
        ("advanced-data-diverging-return", 0) => "the function never returns normally",
        ("advanced-data-diverging-return", 1) => "the function returns unit",
        ("advanced-data-diverging-return", 2) => "the function returns any integer",
        ("advanced-data-array-copy-init", 0) => "repeated initialization needs a Copy element",
        ("advanced-data-array-copy-init", 1) => "String cannot be stored in arrays",
        ("advanced-data-array-copy-init", 2) => "array length cannot be 3",
        ("advanced-data-tuple-trailing-comma", 0) => {
            "`(5,)` is a one-element tuple, `(5)` is just a parenthesized expression"
        }
        ("advanced-data-tuple-trailing-comma", 1) => "both are i32 arrays",
        ("advanced-data-tuple-trailing-comma", 2) => "they are identical",
        ("advanced-ownership-partial-move-struct", 0) => "person.age",
        ("advanced-ownership-partial-move-struct", 1) => "person as a whole",
        ("advanced-ownership-partial-move-struct", 2) => "person.name",
        ("advanced-ownership-mem-take", 0) => {
            "move the old value out and leave the default empty String"
        }
        ("advanced-ownership-mem-take", 1) => "clone the heap data",
        ("advanced-ownership-mem-take", 2) => "turn s into &'static str",
        ("advanced-ownership-closure-move", 0) => "move s into the closure environment",
        ("advanced-ownership-closure-move", 1) => "borrow s once",
        ("advanced-ownership-closure-move", 2) => "convert s into &str",
        ("advanced-slice-split-at-mut", 0) => "it guarantees the two slices do not overlap",
        ("advanced-slice-split-at-mut", 1) => "it disables borrow checking",
        ("advanced-slice-split-at-mut", 2) => "it copies the whole array",
        ("advanced-slice-pattern-match", 0) => {
            "match at least two elements and bind the first and last"
        }
        ("advanced-slice-pattern-match", 1) => "match exactly two elements",
        ("advanced-slice-pattern-match", 2) => "match an empty slice",
        ("advanced-borrow-two-phase", 0) => {
            "the mutable borrow for the method call is activated after arguments are evaluated"
        }
        ("advanced-borrow-two-phase", 1) => "push does not need a mutable borrow",
        ("advanced-borrow-two-phase", 2) => "len consumes v",
        ("advanced-borrow-reborrow-mut", 0) => {
            "the call temporarily reborrows r instead of permanently moving it"
        }
        ("advanced-borrow-reborrow-mut", 1) => "&mut String implements Copy",
        ("advanced-borrow-reborrow-mut", 2) => "touch returns a new String",
        ("advanced-borrow-interior-mutability", 0) => {
            "borrow rules are checked at runtime and violations panic"
        }
        ("advanced-borrow-interior-mutability", 1) => "borrow rules are removed entirely",
        ("advanced-borrow-interior-mutability", 2) => "it is only for multithreading",
        ("advanced-struct-enum-size", 0) => {
            "all values of the same enum type need one uniform size"
        }
        ("advanced-struct-enum-size", 1) => "every variant is separately heap-allocated",
        ("advanced-struct-enum-size", 2) => "small variants compress to zero bytes",
        ("advanced-struct-option-niche", 0) => {
            "None can be represented with the null-pointer niche"
        }
        ("advanced-struct-option-niche", 1) => "Option is always zero-sized",
        ("advanced-struct-option-niche", 2) => "the reference is copied twice",
        ("advanced-result-transpose", 0) => "Result<Option<T>, E>",
        ("advanced-result-transpose", 1) => "Option<Option<T>>",
        ("advanced-result-transpose", 2) => "Result<T, Option<E>>",
        ("advanced-result-ok-or-else", 0) => "it runs the closure to build the error only on None",
        ("advanced-result-ok-or-else", 1) => "it always ignores errors",
        ("advanced-result-ok-or-else", 2) => "it converts Result back to Option",
        ("advanced-result-thiserror-idea", 0) => "return Result so callers choose how to handle it",
        ("advanced-result-thiserror-idea", 1) => "panic directly",
        ("advanced-result-thiserror-idea", 2) => "return bool and discard the reason",
        ("advanced-collections-drain", 0) => {
            "remove that range from the Vec and yield removed elements"
        }
        ("advanced-collections-drain", 1) => "borrow the range without changing the Vec",
        ("advanced-collections-drain", 2) => "copy the range while keeping originals",
        ("advanced-collections-hashmap-key-move", 0) => {
            "ownership of the String key moves into the HashMap"
        }
        ("advanced-collections-hashmap-key-move", 1) => "insert automatically clones the key",
        ("advanced-collections-hashmap-key-move", 2) => "HashMap does not store keys",
        ("advanced-iterator-partition", 0) => "two collections: matching and non-matching items",
        ("advanced-iterator-partition", 1) => "only the first matching item",
        ("advanced-iterator-partition", 2) => "a bool",
        ("advanced-generics-associated-type", 0) => "an associated type of the Iterator trait",
        ("advanced-generics-associated-type", 1) => "the generic function name",
        ("advanced-generics-associated-type", 2) => "a lifetime parameter",
        ("advanced-generics-blanket-impl", 0) => "blanket implementation",
        ("advanced-generics-blanket-impl", 1) => "orphan rule",
        ("advanced-generics-blanket-impl", 2) => "lifetime elision",
        ("advanced-generics-phantom-lifetime", 0) => {
            "the type is logically tied to a lifetime/type even without storing such a field"
        }
        ("advanced-generics-phantom-lifetime", 1) => "force heap allocation",
        ("advanced-generics-phantom-lifetime", 2) => "make T automatically Copy",
        ("advanced-concurrency-send-sync", 0) => {
            "ownership of T can be safely transferred to another thread"
        }
        ("advanced-concurrency-send-sync", 1) => {
            "references to T can always be shared across threads"
        }
        ("advanced-concurrency-send-sync", 2) => "T can never panic",
        ("advanced-concurrency-mpsc-clone-sender", 0) => {
            "create another sender so multiple producers can send messages"
        }
        ("advanced-concurrency-mpsc-clone-sender", 1) => "deep-copy the channel buffer",
        ("advanced-concurrency-mpsc-clone-sender", 2) => "close the original sender",
        ("advanced-concurrency-deadlock-order", 0) => "deadlock",
        ("advanced-concurrency-deadlock-order", 1) => "automatic data cloning",
        ("advanced-concurrency-deadlock-order", 2) => "the compiler rejects every such program",
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
        "syntax-shadowing" => "The second `let spaces` creates a new binding, so the type can change from `&str` to `usize`.",
        "syntax-println-placeholder" => "`{name}` inside a format string captures the variable with the same name.",
        "syntax-semicolon-unit" => "`40 + 2;` is a statement. A block with no tail expression evaluates to `()`.",
        "tuple-destructure" => "A tuple can be destructured with a pattern: `let (x, y, z) = tup;`.",
        "function-return" => "The function body's final expression has no semicolon, so it becomes the return value.",
        "array-type" => "Array types are written `[T; N]`; the length N is part of the type.",
        "statement-vs-expression" => "The semicolon discards the value of 42, leaving the function body with no i32 tail expression.",
        "function-param-type" => "Rust does not infer function parameter types from the body; they must be written in the signature.",
        "match-exhaustive" => "Rust requires match to cover all cases; `_` matches everything left.",
        "if-let-some" => "`if let Some(value) = maybe` is for the case where only one pattern matters.",
        "match-return-type" => "Because match is an expression, all arms must produce compatible types; here one is an integer and one is a string slice.",
        "if-expression-value" => "Rust `if` can be an expression. Since the condition is true, this one produces `pass`.",
        "match-option-none" => "Option has only `Some` and `None`; matching both variants makes the match exhaustive.",
        "ownership-move" => "`String` is not Copy. Assigning it to alias moves ownership, so name cannot be used.",
        "ownership-clone" => "`clone()` explicitly copies heap data so both Strings own separate allocations.",
        "ownership-copy-trap" => "Simple scalar values such as i32 implement Copy. String owns heap memory, so assignment moves it.",
        "ownership-borrow-read" => "Passing `&name` only borrows the value, so the caller still owns name afterward.",
        "ownership-function-takes" => "Passing a String by value moves it into the function parameter, making the old binding unusable.",
        "slice-range" => "`&s[0..5]` borrows bytes 0 through 4, which spell `hello`.",
        "first-word-return" => "Returning a slice lets the compiler keep the result tied to the original string borrow.",
        "string-literal-slice" => "A string literal is stored in the binary, and the variable has type `&str`.",
        "slice-borrow-blocks-clear" => "word is an immutable borrow of s; before its last use, s cannot also be mutably borrowed for clear.",
        "slice-open-ended-range" => "`6..` omits the right boundary, so it means from index 6 to the end.",
        "borrowing-mut-ref" => "Create the mutable owner, create the mutable reference, mutate through it, then read nums after the borrow ends.",
        "borrowing-rule" => "The active-reference rule is many readers or one writer.",
        "borrow-scope-release" => "After the last use of the immutable read, the borrow ends and mutation can happen.",
        "dangling-reference" => "A local value is dropped at function end. Returning a reference to it would create a dangling reference.",
        "borrow-immutable-many" => "Many immutable references can coexist because none of them can mutate the referenced data.",
        "struct-update" => "`..user1` takes the remaining fields from user1; non-Copy fields move.",
        "method-self" => "`&self` is short for `self: &Self`, which fits read-only methods.",
        "enum-match-option" => "Option<T> is an enum. `Some(n)` matches the variant and binds the inner value.",
        "struct-field-init-shorthand" => "When a local variable has the same name as a struct field, writing just the field name is shorthand for `field: field`.",
        "method-mut-self" => "Mutating a field requires a mutable borrow of the receiver, so the method uses `&mut self`.",
        "option-match" => "Option has exactly Some and None, so match must handle both.",
        "result-question-mark" => "`?` is early-return sugar, not exception handling.",
        "result-match-ok-err" => "Result has Ok and Err. `Err(_)` matches any error while ignoring the error value.",
        "option-unwrap-risk" => "unwrap is only safe when you are sure a value exists; calling it on None panics.",
        "question-mark-return-type" => "Because `?` returns early on Err, the current function's return type must be compatible with that error.",
        "vec-mut-push" => "`push` changes the Vec, so the binding must be `mut`.",
        "string-update" => "`push_str` appends a string slice without taking ownership of the argument.",
        "hashmap-entry" => "`or_insert` inserts only when the key is absent and returns a mutable reference to the value.",
        "vec-index-bounds" => "`get` returns `Option<&T>` and yields None out of bounds; direct indexing would panic.",
        "hashmap-count-entry" => "`or_insert` returns a mutable reference, so `*count += 1` updates the integer stored in the map.",
        "iterator-chain" => "filter keeps 2 and 4; map multiplies each by 10.",
        "lifetime-meaning" => "Lifetime annotations describe reference relationships; they do not extend actual lifetimes.",
        "iter-vs-into-iter" => "`into_iter()` consumes the collection and yields owned values. Use `iter()` when you only want borrowed values.",
        "collect-type" => "`collect` can build many collection types, so the target type must be inferred or written explicitly.",
        "iterator-lazy" => "Iterator adapters describe a pipeline. They run only when a consumer pulls values from the iterator.",
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
        "syntax-const-binding" => "`const` items require an explicit type annotation; unlike many `let` bindings, the type cannot be omitted.",
        "syntax-shadow-mutability" => "Shadowing introduces a new binding with `let`; it does not mutate the previous binding and can change the type.",
        "syntax-block-scope" => "The inner block shadows x only inside that block. After the block, the outer x is still 5.",
        "match-guard" => "A match arm can add an `if` guard after the pattern. The arm matches only when both pattern and guard pass.",
        "if-let-else" => "maybe is None, so it does not match `Some(n)` and the else branch runs.",
        "while-let-pop" => "while let loops as long as the expression matches the pattern; when pop returns None, it stops.",
        "tuple-index" => "Tuple positions are zero-based and accessed with dot-number syntax, so the second element is `tup.1`.",
        "array-repeat-init" => "`[value; count]` creates an array with count copies of value, so `[3; 5]` has five 3s.",
        "function-explicit-return" => "`return 7;` exits the function immediately, so the later tail expression does not decide the result.",
        "ownership-return-ownership" => "After the function receives ownership of s, returning s moves ownership back to the caller.",
        "ownership-borrow-vs-clone" => "A read-only operation can borrow the String. Borrowing avoids both moving ownership and cloning heap data.",
        "ownership-copy-types" => "Simple scalar types such as bool, char, and integers are Copy; heap-owning types such as String and Vec usually move.",
        "slice-full-range" => "`..` means the full range from start to end, so `&s[..]` borrows the whole string.",
        "slice-string-param" => "`&str` accepts string literals and String slices, making read-only string APIs more flexible than `&String`.",
        "slice-utf8-boundary" => "String slice ranges are byte indexes but must align to UTF-8 character boundaries; splitting a character panics.",
        "borrow-mut-exclusive-error" => "A mutable reference must be exclusive. Two active `&mut` references to the same value are rejected.",
        "borrow-reborrow-shared" => "The code reads through the mutable reference, then mutates through the same reference; no conflicting active borrow remains.",
        "borrow-function-mut-param" => "To let a function mutate the caller's String without taking ownership, pass a mutable reference: `&mut String`.",
        "tuple-struct-access" => "Tuple struct fields are positional, so they are accessed with `.0`, `.1`, and so on.",
        "enum-variant-data" => "Each Rust enum variant can define its own associated data shape, with different types and arities.",
        "match-enum-method" => "A method that only reads the enum can take `&self` and then match on self to inspect the variant.",
        "result-unwrap-or" => "Parsing fails, producing Err, so `unwrap_or(0)` returns the provided default instead of panicking.",
        "option-map" => "`Option::map` transforms the value inside Some and wraps the result back in Some.",
        "question-mark-option" => "In a function returning Option, `?` propagates None by returning None early from the current function.",
        "vec-iterate-borrow" => "Iterating with `&values` borrows the Vec, so the collection is still usable after the loop.",
        "string-push-char" => "`push` appends a single char, while `push_str` appends a string slice.",
        "hashmap-insert-overwrite" => "Inserting the same key again replaces the old value. Use entry when you only want to insert if absent.",
        "iterator-filter-borrow" => "`iter()` yields references, and `filter` passes references to items, so `n` is effectively a double reference here.",
        "iterator-sum-consumes" => "`sum` is a consuming adapter: it takes ownership of the iterator and pulls all items from it.",
        "iterator-enumerate" => "`enumerate` pairs each item with a zero-based index.",
        "impl-trait-param" => "`impl Display` means this parameter can be any type implementing Display, similar to a simple trait bound.",
        "derive-debug-bound" => "`{:?}` uses the Debug formatter. `{}` uses Display.",
        "lifetime-elision" => "Lifetime elision rules let the compiler fill in obvious lifetime relationships; they do not remove the constraints.",
        "thread-join" => "join blocks the current thread until the spawned thread completes and returns its result.",
        "channel-recv-block" => "`recv` waits for a message while senders still exist; if all senders disconnect, it returns an error.",
        "mutex-poison-unwrap" => "`lock()` returns a Result because a panic while holding the lock can poison the mutex; examples often unwrap it for brevity.",
        "syntax-type-inference" => "Rust determines one concrete type for each binding at compile time; mutability changes values, not types.",
        "syntax-mut-reassign" => "count is mutable, so it can be reassigned to another integer value of the same type.",
        "syntax-const-uppercase" => "Rust constants conventionally use SCREAMING_SNAKE_CASE, such as MAX_RETRIES.",
        "syntax-expression-parentheses" => "The parenthesized expression evaluates first: 1 + 2 is 3, then 3 * 3 is 9.",
        "match-literal-pattern" => "match tries patterns and the literal pattern 2 matches the value 2.",
        "match-binding-value" => "A bare identifier pattern matches any value and binds it to that name; here the name is n.",
        "if-else-no-semicolon" => "Rust if can be an expression; both branches produce &str values.",
        "loop-break-value" => "loop can produce a value with `break value`; when n reaches 3, it breaks with 30.",
        "integer-overflow-debug" => "Rust checks integer overflow in debug builds and panics; release builds wrap unless explicit checked methods are used.",
        "float-default-f64" => "Rust defaults floating-point literals to f64 because it is generally as fast and more precise on modern CPUs.",
        "char-four-bytes" => "Rust char is a Unicode scalar value and is four bytes wide; it is not a UTF-8 byte.",
        "function-implicit-unit" => "A function with no explicit return type returns the unit type `()`.",
        "ownership-drop-at-scope-end" => "Values that own resources are dropped automatically when they leave scope.",
        "ownership-reassign-drops-old" => "Reassignment drops the old value and then stores the new value in the binding.",
        "ownership-tuple-move-field" => "Moving a non-Copy field out of a tuple invalidates that field and the tuple as a complete value.",
        "ownership-reference-no-drop" => "A reference is a borrowed view and does not own the data, so dropping the reference does not drop the String.",
        "slice-array-slice" => "An array slice is a borrowed view into a contiguous part of an array, with type `&[T]`.",
        "slice-len-method" => "&nums[1..] contains indexes 1, 2, and 3, so its length is 3.",
        "slice-mut-slice" => "A mutable slice is written `&mut nums[0..2]` and allows modifying the underlying elements.",
        "slice-first-word-signature" => "Using &str is more general than &String; callers can pass string slices or literals.",
        "borrow-last-use-nll" => "Non-lexical lifetimes end the borrow after the reference's last use, so s can be mutably borrowed afterward.",
        "borrow-mut-then-read-owner" => "The last use of r is the push call; after that the mutable borrow ends and s can be read again.",
        "borrow-cannot-move-while-borrowed" => "As long as r will be used later, the borrow of s is active, so s cannot be moved.",
        "borrow-mut-argument-call" => "The call site must pass `&mut text`, explicitly lending a mutable reference to the function.",
        "struct-associated-function" => "Associated functions that do not take self belong to the type itself and are called with `Type::function(...)`.",
        "struct-debug-print" => "`{:?}` uses Debug; custom types commonly derive it with `#[derive(Debug)]`.",
        "enum-if-let-method" => "`if let Message::Quit = msg` runs only when msg is the Quit variant.",
        "method-takes-self" => "Taking self means the method takes ownership of the receiver, often to extract or convert it.",
        "result-map-err" => "map_err transforms the error value only when the Result is Err; Ok is kept as-is.",
        "result-ok-method" => "ok() converts Result<T, E> into Option<T>: Ok becomes Some and Err becomes None.",
        "option-and-then" => "and_then takes a closure returning Option<U>, useful for chaining fallible steps without nesting.",
        "result-expect-message" => "expect panics like unwrap on failure, but includes clearer context in the panic message.",
        "vec-new-type-annotation" => "Vec::new() has no element information, so a variable annotation or later push must provide the element type.",
        "vec-macro-init" => "vec![1, 2, 3] creates a Vec with three elements, so its length is 3.",
        "string-from-vs-literal" => "String owns a growable UTF-8 string on the heap; a string literal is usually &str.",
        "hashmap-get-option" => "A map lookup can miss, so get returns Option<&V> and forces callers to handle absence.",
        "iterator-any" => "any returns true when it finds an item matching the predicate; 4 is even.",
        "iterator-find" => "find may not find a matching item, so it returns Option<Item>.",
        "iterator-collect-string" => "collect can build a String from an iterator of char values; the variable annotation supplies the target type.",
        "iterator-closure-capture" => "The closure captures factor from the environment and multiplies each item by 3.",
        "trait-default-method" => "Trait methods can provide default implementations; implementors may use or override them.",
        "trait-impl-for-type" => "Implementing a trait for a type uses `impl TraitName for TypeName { ... }`.",
        "generic-multiple-bounds" => "Multiple trait bounds are joined with +, such as `T: Display + Clone`.",
        "lifetime-struct-ref" => "A struct holding a reference must express how long that reference is valid relative to the struct.",
        "thread-spawn-return" => "A thread closure can return a value; join retrieves it when the thread finishes successfully.",
        "channel-multiple-send" => "The same sender can send multiple messages, and the receiver receives them in send order.",
        "arc-needed-not-rc" => "Rc uses non-atomic reference counting and is not safe across threads; use Arc for cross-thread sharing.",
        "mutex-scope-release-early" => "MutexGuard releases the lock when dropped; a smaller block drops it before later work.",
        "advanced-syntax-numeric-suffix" => "Numeric literals can carry type suffixes; `10u8` makes the literal a u8.",
        "advanced-syntax-never-semicolon" => "Expressions that never return have type `!`, which can coerce to any required type during type checking.",
        "advanced-syntax-macro-vs-function" => "`println!` is a macro call rather than a normal function call, so it needs `!` after the name.",
        "advanced-control-destructure-enum" => "Struct-like enum variants can be destructured by field name; `{ x, y }` binds both fields.",
        "advanced-control-at-binding" => "The @ pattern binds the matched value while also checking a subpattern.",
        "advanced-control-matches-macro" => "matches! checks whether an expression matches a pattern; Some(3) satisfies the guard n > 2.",
        "advanced-data-diverging-return" => "Return type `!` means the function never returns normally, for example panic or an infinite loop.",
        "advanced-data-array-copy-init" => "The `[expr; N]` syntax repeats the value and generally requires Copy; String is not Copy.",
        "advanced-data-tuple-trailing-comma" => "A one-element tuple needs a trailing comma; without it, this is just parentheses.",
        "advanced-ownership-partial-move-struct" => "After moving a non-Copy field, the whole struct is no longer usable, but unmoved Copy fields can still be accessed.",
        "advanced-ownership-mem-take" => "mem::take replaces the place with Default::default() and returns the old owned value.",
        "advanced-ownership-closure-move" => "A move closure captures used variables by ownership; for String, the closure owns it.",
        "advanced-slice-split-at-mut" => "The safe API guarantees the two &mut slices are disjoint, satisfying Rust's aliasing rules.",
        "advanced-slice-pattern-match" => "In slice patterns, `..` matches any number of middle elements while first and last bind the ends.",
        "advanced-slice-as-bytes" => "The string `é` occupies 2 bytes in UTF-8, so as_bytes().len() returns 2.",
        "advanced-borrow-two-phase" => "Two-phase borrows let some method calls reserve the mutable borrow, compute arguments, then activate it for the call.",
        "advanced-borrow-reborrow-mut" => "The call creates a temporary reborrow; after it ends, the original mutable reference can be used again.",
        "advanced-borrow-interior-mutability" => "RefCell still enforces many-readers-or-one-writer, but at runtime instead of compile time.",
        "advanced-struct-enum-size" => "All values of an enum type need a uniform compile-time size, usually enough for the largest variant plus a discriminant.",
        "advanced-struct-option-niche" => "References cannot be null, so the compiler can use the null pointer as None without extra tag storage.",
        "advanced-struct-match-ref" => "Using `ref name` in a pattern binds a reference to the field, avoiding moving the String out.",
        "advanced-result-transpose" => "transpose flips Option<Result<T,E>> into Result<Option<T>,E>, useful for optional fallible values.",
        "advanced-result-ok-or-else" => "ok_or_else takes a closure and builds the error only when the Option is None, avoiding unnecessary work.",
        "advanced-result-thiserror-idea" => "Recoverable failures should be represented with Result so callers can retry, degrade, report, or exit.",
        "advanced-collections-entry-and-modify" => "and_modify runs only when the key exists and receives &mut V, so the value must be dereferenced to mutate it.",
        "advanced-collections-drain" => "drain removes the specified range and returns an iterator over removed elements.",
        "advanced-collections-hashmap-key-move" => "insert takes key and value by value; String is not Copy, so key ownership moves into the map.",
        "advanced-iterator-flat-map" => "flat_map maps each item into an iterator and flattens the inner iterators into one sequence.",
        "advanced-iterator-by-ref" => "by_ref lets adapters borrow the iterator temporarily; after take consumes two items, the original iterator still has the rest.",
        "advanced-iterator-partition" => "partition consumes the iterator and separates items into two collections based on the predicate.",
        "advanced-generics-associated-type" => "Item is an associated type defined by Iterator, representing the type yielded by the iterator.",
        "advanced-generics-blanket-impl" => "Implementing a trait for all types satisfying a bound is called a blanket implementation.",
        "advanced-generics-phantom-lifetime" => "PhantomData tells the compiler about logical ownership, lifetime, or variance relationships even when no real field stores T.",
        "advanced-concurrency-send-sync" => "Send means ownership can move between threads; Sync means &T can be shared between threads.",
        "advanced-concurrency-mpsc-clone-sender" => "mpsc means multiple producer, single consumer; cloning Sender lets multiple threads own send handles.",
        "advanced-concurrency-deadlock-order" => "If A holds left and waits for right while B holds right and waits for left, they can deadlock.",
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
        "syntax-shadowing" => "This is not mutation; it is another `let` with the same name.",
        "syntax-println-placeholder" => "Write the variable name inside the braces.",
        "syntax-semicolon-unit" => "A semicolon discards the expression value.",
        "tuple-destructure" => "The left side is a pattern, not a type.",
        "function-return" => "Expressions have no trailing semicolon; statements do.",
        "array-type" => "Vec can grow; arrays have a fixed length in the type.",
        "statement-vs-expression" => {
            "Remove the semicolon if you want 42 to be the tail expression."
        }
        "function-param-type" => "The return type already suggests this practice uses an integer.",
        "match-exhaustive" => "Ask what happens if n is 100.",
        "if-let-some" => "The pattern must match Some and bind the inner value.",
        "match-return-type" => "Ask what type value should have after the match finishes.",
        "if-expression-value" => "Both branches are &str, so the result can be assigned to level.",
        "match-option-none" => {
            "Write the absence variant explicitly, not a wildcard for this exercise."
        }
        "ownership-move" => {
            "The assignment is legal. The error appears when the old owner is used again."
        }
        "ownership-clone" => "This is an explicit deep copy, and Rust makes that cost visible.",
        "ownership-copy-trap" => {
            "Ask whether the type owns heap resources or is a simple Copy scalar."
        }
        "ownership-borrow-read" => "The parameter type is already `&String`.",
        "ownership-function-takes" => {
            "A function parameter is a new owner when the argument is passed by value."
        }
        "slice-range" => "Rust ranges exclude the right endpoint.",
        "first-word-return" => "An index is only a number; a slice carries a borrow relationship.",
        "string-literal-slice" => "This is why string literals are immutable.",
        "slice-borrow-blocks-clear" => {
            "A slice is not a copy; it still points into the original string."
        }
        "slice-open-ended-range" => "Rust ranges can omit the start or the end.",
        "borrowing-mut-ref" => "`nums` must be declared mut before you create `&mut nums`.",
        "borrowing-rule" => "The key phrase is active references at the same time.",
        "borrow-scope-release" => {
            "Look for the last use of `len`; after that, the immutable borrow is done."
        }
        "dangling-reference" => "Returning ownership avoids the invalid reference entirely.",
        "borrow-immutable-many" => "This is the “many readers” half of the borrowing rule.",
        "struct-update" => "The syntax begins with two dots.",
        "method-self" => "area does not mutate or consume the Rectangle.",
        "enum-match-option" => "This is like if let, but match must also cover None.",
        "struct-field-init-shorthand" => "The long form would be `email: email`.",
        "method-mut-self" => "Read-only uses `&self`; mutation needs a mutable borrow.",
        "option-match" => "Rust uses an enum variant, not null, to express absence.",
        "result-question-mark" => {
            "The current function's return type must be able to carry the error."
        }
        "result-match-ok-err" => "Use `_` when the exact error value is irrelevant.",
        "option-unwrap-risk" => "Prefer match, if let, unwrap_or, or ? when absence is expected.",
        "question-mark-return-type" => "Think about where the Err would go.",
        "vec-mut-push" => "Appending mutates the collection.",
        "string-update" => "push appends one char; push_str appends &str.",
        "hashmap-entry" => "The word-frequency example uses `entry(...).or_insert(0)`.",
        "vec-index-bounds" => "Prefer get when the index comes from outside the program.",
        "hashmap-count-entry" => "count has type `&mut i32`, not i32 itself.",
        "iterator-chain" => "Process the chain from top to bottom.",
        "lifetime-meaning" => "`'a` is a constraint, not an allocation strategy.",
        "iter-vs-into-iter" => "The name `into` usually means ownership conversion.",
        "collect-type" => "The turbofish form is `collect::<Vec<i32>>()`.",
        "iterator-lazy" => "The Rust Book phrase is: iterators are lazy.",
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
        "syntax-const-binding" => "The Rust Book's const example writes `: u32`.",
        "syntax-shadow-mutability" => "Look for the second `let`.",
        "syntax-block-scope" => "Evaluate the inner scope and outer scope separately.",
        "match-guard" => "The guard goes after the pattern and before `=>`.",
        "if-let-else" => "if let enters the first branch only when the pattern matches.",
        "while-let-pop" => "What does pop return when the Vec is empty?",
        "tuple-index" => "Tuples use `.1`, not `[1]`.",
        "array-repeat-init" => "The syntax is value, semicolon, length.",
        "function-explicit-return" => {
            "Rust usually uses tail expressions, but return still exits immediately."
        }
        "ownership-return-ownership" => {
            "A tail expression can move a local variable out as the return value."
        }
        "ownership-borrow-vs-clone" => "clone has a cost; borrowing is lighter.",
        "ownership-copy-types" => "Ask whether the type owns heap resources.",
        "slice-full-range" => "Omit both range boundaries.",
        "slice-string-param" => "The Book later improves `&String` to `&str` for this reason.",
        "slice-utf8-boundary" => "String indexes are byte positions, not character numbers.",
        "borrow-mut-exclusive-error" => "One writer or many readers; not many writers.",
        "borrow-reborrow-shared" => "No second reference lives until the final println.",
        "borrow-function-mut-param" => "Read-only is `&String`; mutation needs mut.",
        "tuple-struct-access" => "The first field is position 0.",
        "enum-variant-data" => "The Rust Book's IP address example demonstrates this exact idea.",
        "match-enum-method" => "If it does not mutate or consume self, borrow self.",
        "result-unwrap-or" => "Unlike unwrap, unwrap_or has a fallback value.",
        "option-map" => "If maybe were None, map would keep it as None.",
        "question-mark-option" => {
            "The key is compatibility with the current function's return type."
        }
        "vec-iterate-borrow" => "Borrow the collection if you do not want to move it.",
        "string-push-char" => "Single quotes mean char.",
        "hashmap-insert-overwrite" => {
            "Contrast insert overwriting with entry inserting only when absent."
        }
        "iterator-filter-borrow" => "You can also write the closure pattern as `|&&n| n % 2 == 0`.",
        "iterator-sum-consumes" => "Consuming adapters usually take self by value.",
        "iterator-enumerate" => "The first index is 0.",
        "impl-trait-param" => "This expresses the same capability as `T: Display`.",
        "derive-debug-bound" => "Question-mark formatting uses Debug.",
        "lifetime-elision" => "Elision means the compiler fills in the annotation by rule.",
        "thread-join" => "Without joining, main may finish first.",
        "channel-recv-block" => "The non-blocking variant is try_recv.",
        "mutex-poison-unwrap" => "The Book's counter example uses `lock().unwrap()`.",
        "syntax-type-inference" => "mut is not a dynamic typing switch.",
        "syntax-mut-reassign" => "The type stays the same; only the value changes.",
        "syntax-const-uppercase" => "Look at the constant examples in The Rust Book.",
        "syntax-expression-parentheses" => "This is about expression evaluation order.",
        "match-literal-pattern" => "_ catches what earlier arms did not match.",
        "match-binding-value" => "This binding pattern matches every value.",
        "if-else-no-semicolon" => "Check whether each branch has a trailing semicolon.",
        "loop-break-value" => "Rust loop can be an expression.",
        "integer-overflow-debug" => {
            "The Rust Book discusses integer overflow in the data types chapter."
        }
        "float-default-f64" => "The Rust Book: floating-point types.",
        "char-four-bytes" => "Single quotes are char; double quotes are string slices.",
        "function-implicit-unit" => "println! is mainly for a side effect.",
        "ownership-drop-at-scope-end" => "This is central to Rust memory management without a GC.",
        "ownership-reassign-drops-old" => "Ownership also applies to reassignment.",
        "ownership-tuple-move-field" => {
            "Moving non-Copy fields affects the compound value that contains them."
        }
        "ownership-reference-no-drop" => "The owner of the String is responsible for releasing it.",
        "slice-array-slice" => "The slice type does not include the length.",
        "slice-len-method" => "An omitted end bound means until the end.",
        "slice-mut-slice" => "The array itself must also be mut.",
        "slice-first-word-signature" => "Prefer accepting the more abstract slice type.",
        "borrow-last-use-nll" => "Find the last use of r.",
        "borrow-mut-then-read-owner" => "NLL shortens borrows to their last use.",
        "borrow-cannot-move-while-borrowed" => "A reference cannot point to a moved value.",
        "borrow-mut-argument-call" => "The function signature tells you the argument type.",
        "struct-associated-function" => "Constructors are often associated functions.",
        "struct-debug-print" => "Question-mark formatting means Debug.",
        "enum-if-let-method" => "if let is useful when only one variant matters.",
        "method-takes-self" => "The `into_` prefix often hints at ownership conversion.",
        "result-map-err" => "Similar to map for Some, but on Err.",
        "result-ok-method" => "The method name ok means keeping the success value.",
        "option-and-then" => "Often called flatMap in other languages.",
        "result-expect-message" => "It is not recovery; it is a better crash message.",
        "vec-new-type-annotation" => "Empty collections often need a type hint.",
        "vec-macro-init" => "vec! is a macro, not a normal function.",
        "string-from-vs-literal" => "String in the collections chapter is growable.",
        "hashmap-get-option" => "This is similar to Vec::get.",
        "iterator-any" => "any is a consuming adapter and short-circuits.",
        "iterator-find" => "No match means None.",
        "iterator-collect-string" => "The target type for collect is important.",
        "iterator-closure-capture" => {
            "Closures can capture environment, unlike plain function pointers."
        }
        "trait-default-method" => "The Rust Book Summary trait has an example.",
        "trait-impl-for-type" => "impl is used for inherent impls and trait impls.",
        "generic-multiple-bounds" => "impl Trait can also write `impl Display + Clone`.",
        "lifetime-struct-ref" => {
            "Lifetime parameters are for reference fields, not owned String fields."
        }
        "thread-spawn-return" => "join returns a Result.",
        "channel-multiple-send" => "mpsc means multiple producer, single consumer.",
        "arc-needed-not-rc" => "The A in Arc means Atomic.",
        "mutex-scope-release-early" => "Lock release is tied to Drop.",
        "advanced-syntax-numeric-suffix" => {
            "The suffix belongs to the literal, not the variable name."
        }
        "advanced-syntax-never-semicolon" => "This is about the never type.",
        "advanced-syntax-macro-vs-function" => "When you see `!`, think macro.",
        "advanced-control-destructure-enum" => "Field shorthand works in patterns too.",
        "advanced-control-at-binding" => "This is a useful but easy-to-miss pattern feature.",
        "advanced-control-matches-macro" => "matches! is useful for boolean pattern checks.",
        "advanced-data-diverging-return" => "`!` is not unit `()`.",
        "advanced-data-array-copy-init" => "Use `std::array::from_fn` to create separate Strings.",
        "advanced-data-tuple-trailing-comma" => "This is a common Rust syntax trap.",
        "advanced-ownership-partial-move-struct" => "This is a partial move.",
        "advanced-ownership-mem-take" => "It is useful for moving out through a mutable reference.",
        "advanced-ownership-closure-move" => {
            "This is the same ownership idea as move thread closures."
        }
        "advanced-slice-split-at-mut" => {
            "Multiple mutable references are okay when they point to disjoint data."
        }
        "advanced-slice-pattern-match" => "This is more declarative than manual indexing.",
        "advanced-slice-as-bytes" => "String length is often byte length, not character count.",
        "advanced-borrow-two-phase" => {
            "This is a refined borrow-checking rule for common method calls."
        }
        "advanced-borrow-reborrow-mut" => {
            "This explains why many &mut calls do not consume the reference."
        }
        "advanced-borrow-interior-mutability" => {
            "RefCell is for single-threaded interior mutability."
        }
        "advanced-struct-enum-size" => "This matters for enum memory layout.",
        "advanced-struct-option-niche" => "This is called niche optimization.",
        "advanced-struct-match-ref" => "Pattern `ref` is not placed like expression `&`.",
        "advanced-result-transpose" => "Common when parsing optional fields.",
        "advanced-result-ok-or-else" => "The else hints at lazy fallback logic.",
        "advanced-result-thiserror-idea" => {
            "panic is better for unrecoverable bugs or invariant violations."
        }
        "advanced-collections-entry-and-modify" => "This is an advanced entry API pattern.",
        "advanced-collections-drain" => "It mutates the original collection.",
        "advanced-collections-hashmap-key-move" => {
            "If you still need the key, borrow for lookup or clone explicitly."
        }
        "advanced-iterator-flat-map" => "It is like map followed by flatten.",
        "advanced-iterator-by-ref" => "Useful for consuming one iterator in phases.",
        "advanced-iterator-partition" => "The target collection types often need annotations.",
        "advanced-generics-associated-type" => {
            "Associated types let traits say implementors provide a type."
        }
        "advanced-generics-blanket-impl" => "The standard library uses many of these.",
        "advanced-generics-phantom-lifetime" => {
            "Common in unsafe abstractions and zero-sized marker types."
        }
        "advanced-concurrency-send-sync" => {
            "Many concurrency errors are Send/Sync boundary errors."
        }
        "advanced-concurrency-mpsc-clone-sender" => {
            "Receiver sees closure only after all Senders are dropped."
        }
        "advanced-concurrency-deadlock-order" => {
            "Rust prevents data races, but not all logical deadlocks."
        }
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
