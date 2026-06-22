#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Stage {
    Foundation,
    Ownership,
    Patterns,
    Production,
}

impl Stage {
    pub const fn all() -> &'static [Stage] {
        &[
            Stage::Foundation,
            Stage::Ownership,
            Stage::Patterns,
            Stage::Production,
        ]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Stage::Foundation => "语法地基",
            Stage::Ownership => "所有权核心",
            Stage::Patterns => "表达力提升",
            Stage::Production => "工程化入口",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Stage::Foundation => "先把变量、表达式和控制流变成肌肉记忆。",
            Stage::Ownership => "集中攻克 move、borrow 和可变引用。",
            Stage::Patterns => "用枚举、Result、迭代器写出更 Rust 的代码。",
            Stage::Production => "补齐 trait、生命周期和 async 的工程直觉。",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Demo {
    pub title: &'static str,
    pub code: &'static str,
    pub output: &'static str,
    pub takeaway: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lesson {
    pub id: &'static str,
    pub title: &'static str,
    pub stage: Stage,
    pub minutes: u8,
    pub difficulty: u8,
    pub summary: &'static str,
    pub goals: &'static [&'static str],
    pub exercise_ids: &'static [&'static str],
    pub book_url: &'static str,
    pub demo: Demo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KnowledgeCard {
    pub id: &'static str,
    pub title: &'static str,
    pub tag: &'static str,
    pub summary: &'static str,
    pub wrong: &'static str,
    pub fix: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StageSummary {
    pub stage: Stage,
    pub lesson_count: usize,
    pub exercise_count: usize,
    pub completed_count: usize,
}

impl StageSummary {
    pub fn completion_rate(self) -> f32 {
        if self.exercise_count == 0 {
            0.0
        } else {
            self.completed_count as f32 / self.exercise_count as f32
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LessonProgress {
    pub lesson: &'static Lesson,
    pub total: usize,
    pub completed: usize,
    pub locked: bool,
}

impl LessonProgress {
    pub fn rate(self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            self.completed as f32 / self.total as f32
        }
    }
}

pub const LESSONS: &[Lesson] = &[
    Lesson {
        id: "syntax-basics",
        title: "变量、可变性与表达式",
        stage: Stage::Foundation,
        minutes: 18,
        difficulty: 1,
        summary: "理解 let、mut、shadowing 和表达式返回值，先让语法不再陌生。",
        goals: &[
            "区分不可变绑定和可变绑定",
            "理解 block 表达式的返回值",
            "读懂 println! 宏中的占位符",
        ],
        exercise_ids: &["syntax-let-mut", "syntax-output"],
        book_url: "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html",
        demo: Demo {
            title: "Rust Book：可变变量",
            code: "let mut x = 5;\nprintln!(\"The value of x is: {x}\");\nx = 6;\nprintln!(\"The value of x is: {x}\");",
            output: "The value of x is: 5\nThe value of x is: 6",
            takeaway: "Rust Book 从 `let mut x = 5` 开始说明：绑定默认不可变，需要修改同一个绑定时显式写 `mut`。",
        },
    },
    Lesson {
        id: "control-flow",
        title: "match、if let 与模式",
        stage: Stage::Foundation,
        minutes: 20,
        difficulty: 2,
        summary: "Rust 的分支是表达式，match 要穷尽所有可能。",
        goals: &[
            "知道 match 必须穷尽",
            "用 if let 处理只关心一种情况的分支",
            "理解分支表达式的统一返回类型",
        ],
        exercise_ids: &["match-exhaustive", "if-let-some"],
        book_url: "https://doc.rust-lang.org/book/ch06-02-match.html",
        demo: Demo {
            title: "Rust Book：match 返回一个值",
            code: "enum Coin { Penny, Nickel, Dime, Quarter }\n\nlet coin = Coin::Quarter;\nlet cents = match coin {\n    Coin::Penny => 1,\n    Coin::Nickel => 5,\n    Coin::Dime => 10,\n    Coin::Quarter => 25,\n};",
            output: "cents = 25",
            takeaway: "Rust Book 的 Coin 例子展示了 `match` 的穷尽性，以及每个分支都能产出同一类型的值。",
        },
    },
    Lesson {
        id: "data-functions",
        title: "数据类型、函数与返回值",
        stage: Stage::Foundation,
        minutes: 24,
        difficulty: 2,
        summary: "把标量、复合类型、函数参数和表达式返回值连起来，建立 Rust 语法的基本手感。",
        goals: &[
            "读懂元组和数组的类型写法",
            "区分语句和表达式",
            "知道函数返回值来自最后一个表达式",
        ],
        exercise_ids: &["tuple-destructure", "function-return", "array-type"],
        book_url: "https://doc.rust-lang.org/book/ch03-02-data-types.html",
        demo: Demo {
            title: "Rust Book：元组解构",
            code: "let tup: (i32, f64, u8) = (500, 6.4, 1);\nlet (x, y, z) = tup;",
            output: "x = 500, y = 6.4, z = 1",
            takeaway: "Rust Book 用元组展示复合类型：类型和元素个数固定，可以用模式一次性解构。",
        },
    },
    Lesson {
        id: "ownership",
        title: "所有权、move 与 clone",
        stage: Stage::Ownership,
        minutes: 28,
        difficulty: 3,
        summary: "Rust 最关键的一关：值默认只有一个所有者，移动后原变量不能再用。",
        goals: &[
            "判断 String 何时发生 move",
            "区分 Copy 和 Clone",
            "知道借用能避免转移所有权",
        ],
        exercise_ids: &["ownership-move", "ownership-clone", "ownership-copy-trap"],
        book_url: "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html",
        demo: Demo {
            title: "Rust Book：move 而不是浅拷贝",
            code: "let s1 = String::from(\"hello\");\nlet s2 = s1;\n// println!(\"{s1}\"); // 编译失败",
            output: "s2 owns the String",
            takeaway: "Rust Book 用 `let s2 = s1` 说明堆上资源会移动所有权，旧绑定被视为无效。",
        },
    },
    Lesson {
        id: "slices",
        title: "切片与 first_word",
        stage: Stage::Ownership,
        minutes: 24,
        difficulty: 3,
        summary: "切片是对集合一段连续内容的借用，是理解字符串和借用边界的关键例子。",
        goals: &[
            "读懂字符串切片范围语法",
            "知道 &str 通常是借用视图",
            "解释 first_word 为什么返回切片更安全",
        ],
        exercise_ids: &["slice-range", "first-word-return", "string-literal-slice"],
        book_url: "https://doc.rust-lang.org/book/ch04-03-slices.html",
        demo: Demo {
            title: "Rust Book：字符串切片",
            code: "let s = String::from(\"hello world\");\nlet hello = &s[0..5];\nlet world = &s[6..11];",
            output: "hello, world",
            takeaway: "切片不拥有数据，只借用集合中的一段；这能让返回值和原字符串的有效期绑定起来。",
        },
    },
    Lesson {
        id: "borrowing",
        title: "借用与可变引用",
        stage: Stage::Ownership,
        minutes: 30,
        difficulty: 4,
        summary: "掌握同一时间只能有一个可变引用，或多个不可变引用。",
        goals: &[
            "用 &T 读取但不拿走所有权",
            "用 &mut T 修改值",
            "解释可变引用和不可变引用不能同时活跃",
        ],
        exercise_ids: &[
            "borrowing-mut-ref",
            "borrowing-rule",
            "borrow-scope-release",
            "dangling-reference",
        ],
        book_url: "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html",
        demo: Demo {
            title: "Rust Book：借用 String",
            code: "fn calculate_length(s: &String) -> usize {\n    s.len()\n}\n\nlet s1 = String::from(\"hello\");\nlet len = calculate_length(&s1);",
            output: "len = 5",
            takeaway: "Rust Book 的 `calculate_length(&s1)` 例子说明：传引用只借用，不拿走调用者的所有权。",
        },
    },
    Lesson {
        id: "structs-enums",
        title: "结构体、方法与枚举",
        stage: Stage::Patterns,
        minutes: 30,
        difficulty: 3,
        summary: "用 struct 表达领域数据，用 impl 组织行为，用 enum 表达有限状态。",
        goals: &[
            "使用字段初始化和结构体更新语法",
            "区分方法里的 &self、&mut self 和 self",
            "用枚举和 match 表达不同变体",
        ],
        exercise_ids: &["struct-update", "method-self", "enum-match-option"],
        book_url: "https://doc.rust-lang.org/book/ch05-01-defining-structs.html",
        demo: Demo {
            title: "Rust Book：结构体更新语法",
            code: "let user2 = User {\n    email: String::from(\"another@example.com\"),\n    ..user1\n};",
            output: "user2 reuses remaining fields from user1",
            takeaway: "`..user1` 会移动未显式赋值的非 Copy 字段，因此 user1 之后可能不再完整可用。",
        },
    },
    Lesson {
        id: "result-option",
        title: "Option、Result 与错误处理",
        stage: Stage::Patterns,
        minutes: 26,
        difficulty: 3,
        summary: "用类型表达可能失败，替代 null 和隐藏异常。",
        goals: &[
            "读懂 Option<T> 和 Result<T, E>",
            "用 match 处理成功和失败",
            "知道 ? 会提前返回错误",
        ],
        exercise_ids: &["option-match", "result-question-mark"],
        book_url: "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
        demo: Demo {
            title: "Rust Book：match 处理 Result",
            code: "use std::fs::File;\n\nlet greeting_file_result = File::open(\"hello.txt\");\nlet greeting_file = match greeting_file_result {\n    Ok(file) => file,\n    Err(error) => panic!(\"Problem opening the file: {error:?}\"),\n};",
            output: "Ok(file) or panic with error",
            takeaway: "Rust Book 先用 `match` 展示 Result 的成功和失败分支，再引出更简洁的错误传播写法。",
        },
    },
    Lesson {
        id: "collections",
        title: "常用集合：Vec、String、HashMap",
        stage: Stage::Patterns,
        minutes: 32,
        difficulty: 3,
        summary: "用标准库集合处理真实数据流，同时理解可变性、所有权和 entry API。",
        goals: &[
            "创建并修改 Vec<T>",
            "安全更新 String",
            "用 HashMap::entry 处理插入或更新",
        ],
        exercise_ids: &["vec-mut-push", "string-update", "hashmap-entry"],
        book_url: "https://doc.rust-lang.org/book/ch08-00-common-collections.html",
        demo: Demo {
            title: "Rust Book：HashMap entry",
            code: "let mut scores = HashMap::new();\nscores.insert(String::from(\"Blue\"), 10);\nscores.entry(String::from(\"Yellow\")).or_insert(50);",
            output: "{\"Blue\": 10, \"Yellow\": 50}",
            takeaway: "`entry(...).or_insert(...)` 只在 key 不存在时插入，是统计词频等场景的常用写法。",
        },
    },
    Lesson {
        id: "iterators-traits",
        title: "迭代器、trait 与生命周期直觉",
        stage: Stage::Production,
        minutes: 35,
        difficulty: 5,
        summary: "从能写到写得像 Rust：组合迭代器，理解 trait bound 和引用有效期。",
        goals: &[
            "用 map/filter/collect 组织数据流",
            "读懂 impl Trait 和 trait bound",
            "知道生命周期标注描述引用关系，不延长生命",
        ],
        exercise_ids: &[
            "iterator-chain",
            "lifetime-meaning",
            "iter-vs-into-iter",
            "collect-type",
        ],
        book_url: "https://doc.rust-lang.org/book/ch13-02-iterators.html",
        demo: Demo {
            title: "Rust Book：消费迭代器",
            code: "let v1 = vec![1, 2, 3];\nlet v1_iter = v1.iter();\nlet total: i32 = v1_iter.sum();",
            output: "6",
            takeaway: "Rust Book 用 `sum` 展示消费适配器：迭代器本身是惰性的，消费方法会真正执行遍历。",
        },
    },
    Lesson {
        id: "generics-traits",
        title: "泛型、trait bound 与生命周期",
        stage: Stage::Production,
        minutes: 36,
        difficulty: 5,
        summary: "把重复逻辑抽象成泛型函数，用 trait bound 描述能力，用生命周期描述引用关系。",
        goals: &[
            "读懂泛型函数签名",
            "给泛型参数加 trait bound",
            "解释 longest 中生命周期参数的含义",
        ],
        exercise_ids: &[
            "generic-largest",
            "trait-bound-display",
            "lifetime-longest",
            "where-clause",
            "static-lifetime-myth",
        ],
        book_url: "https://doc.rust-lang.org/book/ch10-00-generics.html",
        demo: Demo {
            title: "Rust Book：longest 生命周期",
            code: "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {\n    if x.len() > y.len() { x } else { y }\n}",
            output: "returns one of the borrowed inputs",
            takeaway: "`'a` 表示返回引用的有效期不能超过两个入参中较短的那个，而不是延长任何值的生命。",
        },
    },
    Lesson {
        id: "concurrency",
        title: "线程、消息传递与共享状态",
        stage: Stage::Production,
        minutes: 38,
        difficulty: 5,
        summary: "用 thread、channel、Mutex 和 Arc 理解 Rust 如何把并发错误前移到编译期。",
        goals: &[
            "知道 thread::spawn 何时需要 move",
            "用 channel 在线程间传递所有权",
            "理解 Arc<Mutex<T>> 的共享可变状态模式",
        ],
        exercise_ids: &[
            "thread-move",
            "channel-send",
            "mutex-lock",
            "arc-clone",
            "mutex-guard-drop",
        ],
        book_url: "https://doc.rust-lang.org/book/ch16-00-concurrency.html",
        demo: Demo {
            title: "Rust Book：move 闭包线程",
            code: "let v = vec![1, 2, 3];\nlet handle = thread::spawn(move || {\n    println!(\"Here's a vector: {:?}\", v);\n});\nhandle.join().unwrap();",
            output: "prints the moved vector from the spawned thread",
            takeaway: "`move` 把闭包用到的值移动进线程，避免线程比原作用域活得更久时出现悬垂引用。",
        },
    },
];

pub const CARDS: &[KnowledgeCard] = &[
    KnowledgeCard {
        id: "move-after-use",
        title: "为什么 move 后不能再用原变量",
        tag: "ownership",
        summary: "String 持有堆内存，move 后所有权转移，原变量被视为无效。",
        wrong: "let b = a; println!(\"{}\", a);",
        fix: "需要继续使用原值时传引用 `&a`，或显式 `a.clone()`。",
    },
    KnowledgeCard {
        id: "mut-ref-exclusive",
        title: "可变引用为什么必须独占",
        tag: "borrow",
        summary: "独占可变引用能防止读写交叉导致的数据竞争和失效读取。",
        wrong: "let r1 = &value; let r2 = &mut value;",
        fix: "缩短不可变引用使用范围，再创建 `&mut value`。",
    },
    KnowledgeCard {
        id: "question-mark",
        title: "? 不是 try-catch",
        tag: "error",
        summary: "? 会在 Err 或 None 时提前返回，而不是捕获后继续执行。",
        wrong: "fn f() { let n = \"x\".parse::<i32>()?; }",
        fix: "让函数返回 `Result<_, _>` 或手写 match 处理错误。",
    },
    KnowledgeCard {
        id: "lifetime-not-extend",
        title: "生命周期标注不延长引用",
        tag: "lifetime",
        summary: "生命周期只描述多个引用之间的有效期关系，不会让局部变量活得更久。",
        wrong: "fn make_ref<'a>() -> &'a String { &String::from(\"x\") }",
        fix: "返回拥有所有权的 `String`，或引用由调用者传入的值。",
    },
];

pub const fn lessons() -> &'static [Lesson] {
    LESSONS
}

pub const fn cards() -> &'static [KnowledgeCard] {
    CARDS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_metadata_is_stable() {
        assert_eq!(Stage::all().len(), 4);
        assert_eq!(Stage::Foundation.label(), "语法地基");
        assert!(Stage::Production.description().contains("async"));
    }

    #[test]
    fn lessons_have_unique_ids_and_valid_difficulty() {
        let mut ids = Vec::new();

        for lesson in lessons() {
            assert!(!lesson.id.is_empty());
            assert!(!ids.contains(&lesson.id));
            assert!((1..=5).contains(&lesson.difficulty));
            assert!(!lesson.exercise_ids.is_empty());
            assert!(!lesson.demo.code.is_empty());
            ids.push(lesson.id);
        }
    }

    #[test]
    fn cards_are_grouped_by_core_rust_pain_points() {
        let tags: Vec<&str> = cards().iter().map(|card| card.tag).collect();

        assert!(tags.contains(&"ownership"));
        assert!(tags.contains(&"borrow"));
        assert!(tags.contains(&"error"));
        assert!(tags.contains(&"lifetime"));
    }

    #[test]
    fn empty_summary_rates_are_zero() {
        let summary = StageSummary {
            stage: Stage::Foundation,
            lesson_count: 0,
            exercise_count: 0,
            completed_count: 0,
        };

        assert_eq!(summary.completion_rate(), 0.0);
    }

    #[test]
    fn summary_rate_uses_completed_over_total() {
        let summary = StageSummary {
            stage: Stage::Ownership,
            lesson_count: 2,
            exercise_count: 4,
            completed_count: 3,
        };

        assert_eq!(summary.completion_rate(), 0.75);
    }

    #[test]
    fn empty_lesson_progress_rate_is_zero() {
        let progress = LessonProgress {
            lesson: &LESSONS[0],
            total: 0,
            completed: 0,
            locked: false,
        };

        assert_eq!(progress.rate(), 0.0);
    }
}
