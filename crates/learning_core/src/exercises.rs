#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseKind {
    SingleChoice,
    FillBlank,
    OrderSteps,
    CodeOutput,
}

impl ExerciseKind {
    pub const fn label(self) -> &'static str {
        match self {
            ExerciseKind::SingleChoice => "单选题",
            ExerciseKind::FillBlank => "填空题",
            ExerciseKind::OrderSteps => "排序题",
            ExerciseKind::CodeOutput => "输出判断",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Choice(usize),
    Text(&'static str),
    Ordered(&'static [&'static str]),
    Output(&'static str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserAnswer {
    Choice(usize),
    Text(String),
    Ordered(Vec<String>),
    Output(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exercise {
    pub id: &'static str,
    pub lesson_id: &'static str,
    pub title: &'static str,
    pub kind: ExerciseKind,
    pub prompt: &'static str,
    pub code: &'static str,
    pub options: &'static [&'static str],
    pub answer: Answer,
    pub explanation: &'static str,
    pub hint: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckOutcome {
    pub correct: bool,
    pub expected: String,
    pub explanation: &'static str,
}

impl Exercise {
    pub fn check(&self, answer: &UserAnswer) -> CheckOutcome {
        let correct = match (&self.answer, answer) {
            (Answer::Choice(expected), UserAnswer::Choice(actual)) => expected == actual,
            (Answer::Text(expected), UserAnswer::Text(actual)) => {
                normalize_inline(expected) == normalize_inline(actual)
            }
            (Answer::Ordered(expected), UserAnswer::Ordered(actual)) => {
                expected.len() == actual.len()
                    && expected
                        .iter()
                        .zip(actual.iter())
                        .all(|(left, right)| normalize_inline(left) == normalize_inline(right))
            }
            (Answer::Output(expected), UserAnswer::Output(actual)) => {
                normalize_output(expected) == normalize_output(actual)
            }
            _ => false,
        };

        CheckOutcome {
            correct,
            expected: self.answer.summary(),
            explanation: self.explanation,
        }
    }

    pub fn level(&self) -> u8 {
        match self.id {
            "borrowing-mut-ref"
            | "borrowing-rule"
            | "ownership-copy-trap"
            | "borrow-scope-release"
            | "first-word-return"
            | "struct-update"
            | "hashmap-entry"
            | "iterator-chain"
            | "generic-largest"
            | "trait-bound-display"
            | "thread-move"
            | "channel-send" => 2,
            "dangling-reference"
            | "lifetime-meaning"
            | "iter-vs-into-iter"
            | "collect-type"
            | "lifetime-longest"
            | "where-clause"
            | "static-lifetime-myth"
            | "mutex-lock"
            | "arc-clone"
            | "mutex-guard-drop" => 3,
            _ => 1,
        }
    }
}

impl Answer {
    pub fn summary(self) -> String {
        match self {
            Answer::Choice(index) => format!("选择第 {} 项", index + 1),
            Answer::Text(value) => value.to_owned(),
            Answer::Ordered(items) => items.join(" -> "),
            Answer::Output(output) => normalize_output(output),
        }
    }
}

pub fn exercise_by_id(id: &str) -> Option<&'static Exercise> {
    EXERCISES.iter().find(|exercise| exercise.id == id)
}

pub fn exercises_for_lesson(lesson_id: &str) -> Vec<&'static Exercise> {
    EXERCISES
        .iter()
        .filter(|exercise| exercise.lesson_id == lesson_id)
        .collect()
}

pub const fn exercises() -> &'static [Exercise] {
    EXERCISES
}

fn normalize_inline(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_output(input: &str) -> String {
    input
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned()
}

pub const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "syntax-let-mut",
        lesson_id: "syntax-basics",
        title: "让变量真的可变",
        kind: ExerciseKind::FillBlank,
        prompt: "下面代码想让 score 加 1，空白处应该填什么？",
        code: "let ____ score = 41;\nscore += 1;",
        options: &[],
        answer: Answer::Text("mut"),
        explanation: "`let` 默认不可变。需要修改同一个绑定时，加 `mut`。",
        hint: "不是 shadowing，这里是对同一个绑定做 `+=`。",
    },
    Exercise {
        id: "syntax-output",
        lesson_id: "syntax-basics",
        title: "读懂 block 表达式",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let answer = {\n    let base = 40;\n    base + 2\n};\nprintln!(\"{answer}\");",
        options: &[],
        answer: Answer::Output("42"),
        explanation: "代码块最后一行没有分号，所以 `base + 2` 是这个 block 的返回值。",
        hint: "注意 `base + 2` 后面没有分号。",
    },
    Exercise {
        id: "tuple-destructure",
        lesson_id: "data-functions",
        title: "元组解构",
        kind: ExerciseKind::FillBlank,
        prompt: "按照 Rust Book 的元组例子，空白处应如何绑定三个值？",
        code: "let tup = (500, 6.4, 1);\nlet ____ = tup;\nprintln!(\"{x} {y} {z}\");",
        options: &[],
        answer: Answer::Text("(x, y, z)"),
        explanation: "元组可以用模式解构，`let (x, y, z) = tup;` 会把三个位置的值分别绑定出来。",
        hint: "左侧不是类型，而是一个能匹配元组三个元素的模式。",
    },
    Exercise {
        id: "function-return",
        lesson_id: "data-functions",
        title: "函数返回最后一个表达式",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段函数调用会输出什么？",
        code: "fn plus_one(x: i32) -> i32 {\n    x + 1\n}\nprintln!(\"{}\", plus_one(5));",
        options: &[],
        answer: Answer::Output("6"),
        explanation: "函数体最后的 `x + 1` 没有分号，因此它是返回值。",
        hint: "Rust Book 特别强调：表达式没有结尾分号，语句有。",
    },
    Exercise {
        id: "array-type",
        lesson_id: "data-functions",
        title: "数组类型包含长度",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪个是包含 5 个 i32 的数组类型？",
        code: "let a = [1, 2, 3, 4, 5];",
        options: &["[i32; 5]", "Vec<i32>", "(i32, 5)"],
        answer: Answer::Choice(0),
        explanation: "数组类型写作 `[T; N]`，长度 N 是类型的一部分。",
        hint: "Vec 是可增长集合；数组长度固定。",
    },
    Exercise {
        id: "match-exhaustive",
        lesson_id: "control-flow",
        title: "match 为什么要有兜底",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪个说法正确？",
        code: "let n = 3;\nlet label = match n {\n    1 => \"one\",\n    2 => \"two\",\n    _ => \"many\",\n};",
        options: &[
            "match 必须覆盖所有可能，`_` 负责兜底",
            "match 只会检查运行时出现过的值",
            "`_` 会让前面的分支失效",
        ],
        answer: Answer::Choice(0),
        explanation: "Rust 要求 `match` 穷尽所有情况；`_` 可以匹配剩余所有值。",
        hint: "想想如果 n 是 100，编译器是否允许没有分支处理它。",
    },
    Exercise {
        id: "if-let-some",
        lesson_id: "control-flow",
        title: "只关心 Some",
        kind: ExerciseKind::FillBlank,
        prompt: "只在 `maybe` 是 `Some` 时打印值，空白处应填什么模式？",
        code: "let maybe = Some(7);\nif let ____ = maybe {\n    println!(\"{value}\");\n}",
        options: &[],
        answer: Answer::Text("Some(value)"),
        explanation: "`if let Some(value) = maybe` 适合只关心一种模式的场景。",
        hint: "模式需要同时匹配 Some 并绑定内部值。",
    },
    Exercise {
        id: "ownership-move",
        lesson_id: "ownership",
        title: "找出 move 后的无效变量",
        kind: ExerciseKind::SingleChoice,
        prompt: "哪一行会导致编译失败？",
        code: "let name = String::from(\"Rust\");\nlet alias = name;\nprintln!(\"{name}\");",
        options: &[
            "`let name = String::from(\"Rust\");`",
            "`let alias = name;`",
            "`println!(\"{name}\");`",
        ],
        answer: Answer::Choice(2),
        explanation: "`String` 不是 Copy，赋值给 alias 后所有权移动，name 不能再被使用。",
        hint: "赋值本身合法，真正报错发生在再次使用旧 owner 时。",
    },
    Exercise {
        id: "ownership-clone",
        lesson_id: "ownership",
        title: "保留两个 String",
        kind: ExerciseKind::FillBlank,
        prompt: "如果要让 `name` 和 `alias` 都能继续使用，空白处应该填什么？",
        code: "let name = String::from(\"Rust\");\nlet alias = name.____();\nprintln!(\"{name} {alias}\");",
        options: &[],
        answer: Answer::Text("clone"),
        explanation: "`clone()` 显式复制堆数据，因此两个 String 都拥有各自资源。",
        hint: "这是一个有成本的深拷贝，Rust 要求你显式写出来。",
    },
    Exercise {
        id: "ownership-copy-trap",
        lesson_id: "ownership",
        title: "Copy 和 move 的分界",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪种赋值后，原变量仍然可以继续使用？",
        code: "let a = 3;\nlet b = a;\n\nlet s1 = String::from(\"hi\");\nlet s2 = s1;",
        options: &["`a` 仍可用，因为 i32 实现 Copy", "`s1` 仍可用，因为 String 自动 Copy", "两者都不可用"],
        answer: Answer::Choice(0),
        explanation: "整数等简单标量实现 Copy，赋值会复制值；String 管理堆内存，赋值会 move 所有权。",
        hint: "Rust Book 用 Copy trait 区分栈上简单值和持有资源的类型。",
    },
    Exercise {
        id: "slice-range",
        lesson_id: "slices",
        title: "取出 hello 切片",
        kind: ExerciseKind::FillBlank,
        prompt: "按照 Rust Book 的字符串切片例子，空白处应填什么范围？",
        code: "let s = String::from(\"hello world\");\nlet hello = &s[____];",
        options: &[],
        answer: Answer::Text("0..5"),
        explanation: "`&s[0..5]` 借用从索引 0 到 5 之前的字节范围，对应 `hello`。",
        hint: "Rust 的范围右边界不包含在内。",
    },
    Exercise {
        id: "first-word-return",
        lesson_id: "slices",
        title: "first_word 应返回什么",
        kind: ExerciseKind::SingleChoice,
        prompt: "Rust Book 为什么把 first_word 的返回值从 usize 改成 &str？",
        code: "fn first_word(s: &String) -> &str { /* ... */ }",
        options: &[
            "&str 会和原字符串借用关系绑定，避免索引失效",
            "&str 会复制第一个单词，速度更快",
            "&str 可以绕过 UTF-8 检查",
        ],
        answer: Answer::Choice(0),
        explanation: "返回切片能让编译器维护返回值和原字符串的借用关系，避免字符串被修改后旧索引仍被误用。",
        hint: "索引只是数字，切片带着对原数据的借用约束。",
    },
    Exercise {
        id: "string-literal-slice",
        lesson_id: "slices",
        title: "字符串字面量的类型",
        kind: ExerciseKind::SingleChoice,
        prompt: "字符串字面量 `\"hello\"` 的类型最准确地说是什么？",
        code: "let s = \"hello\";",
        options: &["&str", "String", "[char; 5]"],
        answer: Answer::Choice(0),
        explanation: "字符串字面量存储在程序二进制中，变量 `s` 是指向它的 `&str`。",
        hint: "这也是为什么字符串字面量不可变。",
    },
    Exercise {
        id: "borrowing-mut-ref",
        lesson_id: "borrowing",
        title: "可变借用修改值",
        kind: ExerciseKind::OrderSteps,
        prompt: "把步骤排成能通过编译并修改 Vec 的顺序。",
        code: "",
        options: &[
            "let mut nums = vec![1, 2];",
            "let view = &mut nums;",
            "view.push(3);",
            "println!(\"{:?}\", nums);",
        ],
        answer: Answer::Ordered(&[
            "let mut nums = vec![1, 2];",
            "let view = &mut nums;",
            "view.push(3);",
            "println!(\"{:?}\", nums);",
        ]),
        explanation: "先创建可变 owner，再创建可变引用，最后使用引用修改。引用最后一次使用后，nums 可以再次读取。",
        hint: "创建 `&mut nums` 之前，`nums` 必须声明为 mut。",
    },
    Exercise {
        id: "borrowing-rule",
        lesson_id: "borrowing",
        title: "引用规则一句话",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪句话最准确？",
        code: "",
        options: &[
            "同一时间可以有多个不可变引用，或一个可变引用",
            "只要变量是 mut，就可以无限创建可变引用",
            "不可变引用会自动 clone 底层数据",
        ],
        answer: Answer::Choice(0),
        explanation: "借用规则限制的是活跃引用组合：多个读，或一个写。",
        hint: "关键是“同一时间”和“活跃引用”。",
    },
    Exercise {
        id: "borrow-scope-release",
        lesson_id: "borrowing",
        title: "缩短不可变借用作用域",
        kind: ExerciseKind::OrderSteps,
        prompt: "把步骤排成能先读取再修改 String 的顺序。",
        code: "",
        options: &[
            "let mut s = String::from(\"hello\");",
            "let len = s.len();",
            "println!(\"{len}\");",
            "s.push_str(\" world\");",
        ],
        answer: Answer::Ordered(&[
            "let mut s = String::from(\"hello\");",
            "let len = s.len();",
            "println!(\"{len}\");",
            "s.push_str(\" world\");",
        ]),
        explanation: "不可变读取在 `println!` 后不再使用，借用结束，之后可以创建可变借用修改 `s`。",
        hint: "NLL 会根据最后一次使用位置结束借用，而不是机械等到作用域末尾。",
    },
    Exercise {
        id: "dangling-reference",
        lesson_id: "borrowing",
        title: "为什么不能返回悬垂引用",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面函数为什么不能通过编译？",
        code: "fn dangle() -> &String {\n    let s = String::from(\"hello\");\n    &s\n}",
        options: &[
            "s 在函数结束时被释放，返回引用会悬垂",
            "String 不能被引用",
            "返回值必须写成 &'static String",
        ],
        answer: Answer::Choice(0),
        explanation: "局部变量 `s` 在函数结束时释放，返回 `&s` 会让调用方拿到无效引用，Rust 直接拒绝。",
        hint: "正确做法通常是返回拥有所有权的 `String`。",
    },
    Exercise {
        id: "struct-update",
        lesson_id: "structs-enums",
        title: "结构体更新语法",
        kind: ExerciseKind::FillBlank,
        prompt: "复用 user1 其他字段创建 user2，空白处应填什么？",
        code: "let user2 = User {\n    email: String::from(\"another@example.com\"),\n    ____\n};",
        options: &[],
        answer: Answer::Text("..user1"),
        explanation: "`..user1` 表示剩余字段从 user1 取得；非 Copy 字段会发生 move。",
        hint: "Rust Book 的结构体更新语法以两个点开头。",
    },
    Exercise {
        id: "method-self",
        lesson_id: "structs-enums",
        title: "只读方法应借用 self",
        kind: ExerciseKind::SingleChoice,
        prompt: "如果方法只读取结构体字段，不拿走所有权，签名里通常写什么？",
        code: "impl Rectangle {\n    fn area(____) -> u32 {\n        self.width * self.height\n    }\n}",
        options: &["&self", "&mut self", "self"],
        answer: Answer::Choice(0),
        explanation: "`&self` 是 `self: &Self` 的简写，适合只读方法。",
        hint: "area 不需要修改 Rectangle，也不应该消费 Rectangle。",
    },
    Exercise {
        id: "enum-match-option",
        lesson_id: "structs-enums",
        title: "Option 是枚举",
        kind: ExerciseKind::FillBlank,
        prompt: "把 Some 中的值取出来加 1，空白处分支应填什么？",
        code: "let maybe = Some(41);\nlet answer = match maybe {\n    ____ => n + 1,\n    None => 0,\n};",
        options: &[],
        answer: Answer::Text("Some(n)"),
        explanation: "Option<T> 是枚举，`Some(n)` 既匹配变体，也绑定内部值。",
        hint: "这一题和 if let 类似，但 match 需要覆盖 None。",
    },
    Exercise {
        id: "option-match",
        lesson_id: "result-option",
        title: "安全处理 None",
        kind: ExerciseKind::FillBlank,
        prompt: "空白处填什么分支才能安全处理 None？",
        code: "let value: Option<i32> = None;\nlet text = match value {\n    Some(n) => n.to_string(),\n    ____ => \"empty\".to_string(),\n};",
        options: &[],
        answer: Answer::Text("None"),
        explanation: "Option 只有 Some 和 None 两种情况，match 需要都处理。",
        hint: "不是 null，Rust 用一个明确的枚举变体表达空值。",
    },
    Exercise {
        id: "result-question-mark",
        lesson_id: "result-option",
        title: "? 的真实含义",
        kind: ExerciseKind::SingleChoice,
        prompt: "`?` 在 Result 上做了什么？",
        code: "let n = raw.parse::<i32>()?;",
        options: &[
            "Ok 时解包值，Err 时从当前函数提前返回 Err",
            "捕获错误并继续执行下一行",
            "把 Err 自动转换成 None",
        ],
        answer: Answer::Choice(0),
        explanation: "`?` 是提前返回语法糖，不是异常捕获。",
        hint: "它要求当前函数的返回类型能承载错误。",
    },
    Exercise {
        id: "vec-mut-push",
        lesson_id: "collections",
        title: "Vec push 需要可变集合",
        kind: ExerciseKind::FillBlank,
        prompt: "要向 Vec 追加元素，空白处应该填什么？",
        code: "let ____ v = vec![1, 2, 3];\nv.push(4);",
        options: &[],
        answer: Answer::Text("mut"),
        explanation: "`push` 会修改 Vec，因此绑定必须是 `mut`。",
        hint: "这和修改普通变量一样，需要显式声明可变。",
    },
    Exercise {
        id: "string-update",
        lesson_id: "collections",
        title: "追加字符串切片",
        kind: ExerciseKind::FillBlank,
        prompt: "把 `bar` 追加到 String 后面，空白处应填哪个方法名？",
        code: "let mut s = String::from(\"foo\");\ns.____(\"bar\");",
        options: &[],
        answer: Answer::Text("push_str"),
        explanation: "`push_str` 接收字符串切片并追加到 String 后面，不会取得参数所有权。",
        hint: "`push` 追加单个 char，`push_str` 追加 &str。",
    },
    Exercise {
        id: "hashmap-entry",
        lesson_id: "collections",
        title: "只在 key 不存在时插入",
        kind: ExerciseKind::FillBlank,
        prompt: "按照 Rust Book 的 entry 写法，空白处应填什么？",
        code: "scores.entry(String::from(\"Blue\")).____(50);",
        options: &[],
        answer: Answer::Text("or_insert"),
        explanation: "`or_insert` 在 key 不存在时插入默认值，存在时返回现有值的可变引用。",
        hint: "词频统计例子会用 `entry(...).or_insert(0)`。",
    },
    Exercise {
        id: "iterator-chain",
        lesson_id: "iterators-traits",
        title: "迭代器链结果",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码打印什么？",
        code: "let values: Vec<_> = vec![1, 2, 3, 4]\n    .into_iter()\n    .filter(|n| n % 2 == 0)\n    .map(|n| n * 10)\n    .collect();\nprintln!(\"{:?}\", values);",
        options: &[],
        answer: Answer::Output("[20, 40]"),
        explanation: "filter 保留偶数 2 和 4，map 再分别乘以 10。",
        hint: "按链式顺序从上到下处理。",
    },
    Exercise {
        id: "lifetime-meaning",
        lesson_id: "iterators-traits",
        title: "生命周期标注的作用",
        kind: ExerciseKind::SingleChoice,
        prompt: "生命周期标注真正表达的是什么？",
        code: "fn longest<'a>(left: &'a str, right: &'a str) -> &'a str { left }",
        options: &[
            "返回引用不会比传入引用活得更久",
            "函数会自动延长 left 和 right 的生命",
            "所有字符串都会被复制到静态区",
        ],
        answer: Answer::Choice(0),
        explanation: "生命周期标注描述引用关系，不负责延长任何值的实际生命周期。",
        hint: "`'a` 是约束，不是内存分配策略。",
    },
    Exercise {
        id: "iter-vs-into-iter",
        lesson_id: "iterators-traits",
        title: "iter 和 into_iter 的所有权差异",
        kind: ExerciseKind::SingleChoice,
        prompt: "对 `Vec<String>` 调用 `into_iter()` 后，原 vector 会怎样？",
        code: "let names = vec![String::from(\"Ferris\")];\nlet owned = names.into_iter();",
        options: &["names 被消费，不能再使用", "names 仍可用，因为只是借用", "into_iter 会返回索引"],
        answer: Answer::Choice(0),
        explanation: "`into_iter()` 消费集合并产生拥有所有权的元素；`iter()` 才是借用迭代。",
        hint: "方法名里的 into 往往暗示所有权转换。",
    },
    Exercise {
        id: "collect-type",
        lesson_id: "iterators-traits",
        title: "collect 需要目标类型",
        kind: ExerciseKind::FillBlank,
        prompt: "把迭代器收集成 Vec<i32>，空白处应填什么？",
        code: "let doubled = vec![1, 2, 3].into_iter().map(|n| n * 2).collect::<____>();",
        options: &[],
        answer: Answer::Text("Vec<i32>"),
        explanation: "`collect` 很泛型，通常需要 turbofish 或变量类型告诉编译器目标集合类型。",
        hint: "也可以写 `let doubled: Vec<i32> = ...collect();`。",
    },
    Exercise {
        id: "generic-largest",
        lesson_id: "generics-traits",
        title: "泛型 largest 需要能力约束",
        kind: ExerciseKind::SingleChoice,
        prompt: "如果 largest 要比较任意 T，T 至少需要实现什么能力？",
        code: "fn largest<T>(list: &[T]) -> &T { /* compare items */ }",
        options: &["PartialOrd", "Iterator", "Default"],
        answer: Answer::Choice(0),
        explanation: "要使用 `>` 或 `<` 比较 T，泛型参数需要 `PartialOrd` 之类的 trait bound。",
        hint: "泛型不是任意魔法；你使用了什么操作，就要声明需要什么能力。",
    },
    Exercise {
        id: "trait-bound-display",
        lesson_id: "generics-traits",
        title: "打印泛型值的 trait bound",
        kind: ExerciseKind::FillBlank,
        prompt: "如果函数内部要用 `{}` 打印 item，空白处应填哪个 trait？",
        code: "fn print_it<T: ____>(item: T) {\n    println!(\"{}\", item);\n}",
        options: &[],
        answer: Answer::Text("Display"),
        explanation: "`{}` 使用 `std::fmt::Display`；`{:?}` 才对应 Debug。",
        hint: "Rust Book 在 trait bound 中用能力描述泛型参数。",
    },
    Exercise {
        id: "lifetime-longest",
        lesson_id: "generics-traits",
        title: "longest 返回值的有效期",
        kind: ExerciseKind::SingleChoice,
        prompt: "`fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` 表示什么？",
        code: "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { x }",
        options: &[
            "返回引用的有效期不超过 x 和 y 中较短的那个",
            "返回引用一定是静态生命周期",
            "函数会复制更长的字符串",
        ],
        answer: Answer::Choice(0),
        explanation: "`'a` 把返回引用和两个输入引用关联起来，编译器会按较短有效期约束调用。",
        hint: "生命周期参数描述关系，不改变所有权和分配。",
    },
    Exercise {
        id: "where-clause",
        lesson_id: "generics-traits",
        title: "where 子句让复杂约束更清晰",
        kind: ExerciseKind::FillBlank,
        prompt: "把泛型约束写到 where 子句中，空白处应填什么 trait？",
        code: "fn notify<T>(item: T)\nwhere\n    T: ____,\n{\n    println!(\"{}\", item);\n}",
        options: &[],
        answer: Answer::Text("Display"),
        explanation: "where 子句适合较复杂的泛型约束；这里 `{}` 仍然要求 T 实现 Display。",
        hint: "这题和 trait bound display 是同一能力，只是换成 where 写法。",
    },
    Exercise {
        id: "static-lifetime-myth",
        lesson_id: "generics-traits",
        title: "'static 不等于永远运行",
        kind: ExerciseKind::SingleChoice,
        prompt: "`&'static str` 最准确表示什么？",
        code: "let s: &'static str = \"I live in the binary\";",
        options: &[
            "引用的数据在整个程序运行期间都有效",
            "变量 s 不能被移动",
            "字符串会在每次调用时重新分配",
        ],
        answer: Answer::Choice(0),
        explanation: "`'static` 描述引用指向的数据可在整个程序期间有效，常见于字符串字面量。",
        hint: "它是生命周期约束，不是变量绑定位置。",
    },
    Exercise {
        id: "thread-move",
        lesson_id: "concurrency",
        title: "线程闭包为什么需要 move",
        kind: ExerciseKind::SingleChoice,
        prompt: "Rust Book 在线程中打印 vector 时，为什么给闭包加 `move`？",
        code: "let v = vec![1, 2, 3];\nthread::spawn(move || println!(\"{:?}\", v));",
        options: &[
            "把 v 的所有权移动进线程，避免线程引用已失效的栈变量",
            "让线程运行得更快",
            "把 Vec 转换成静态数组",
        ],
        answer: Answer::Choice(0),
        explanation: "新线程可能比当前作用域活得更久，`move` 让闭包拥有 v，从根上避免悬垂引用。",
        hint: "并发章节的核心仍然是所有权。",
    },
    Exercise {
        id: "channel-send",
        lesson_id: "concurrency",
        title: "channel 发送会移动值",
        kind: ExerciseKind::SingleChoice,
        prompt: "通过 `tx.send(val)` 发送 String 后，val 会怎样？",
        code: "let val = String::from(\"hi\");\ntx.send(val).unwrap();",
        options: &[
            "所有权移动到接收端，发送端不能再使用 val",
            "send 会自动 clone val",
            "val 会变成空字符串",
        ],
        answer: Answer::Choice(0),
        explanation: "channel 传递所有权，发送后继续使用 val 会被编译器拒绝。",
        hint: "消息传递并发把所有权从一个线程交给另一个线程。",
    },
    Exercise {
        id: "mutex-lock",
        lesson_id: "concurrency",
        title: "共享可变计数器",
        kind: ExerciseKind::FillBlank,
        prompt: "多个线程共享 Mutex 时，通常需要用哪个智能指针包裹？",
        code: "let counter = ____(Mutex::new(0));",
        options: &[],
        answer: Answer::Text("Arc::new"),
        explanation: "`Arc<T>` 提供线程安全的引用计数所有权，`Mutex<T>` 提供内部可变性和互斥访问。",
        hint: "Rc<T> 不能跨线程安全共享；并发章节用 Arc<Mutex<T>>。",
    },
    Exercise {
        id: "arc-clone",
        lesson_id: "concurrency",
        title: "Arc::clone 克隆的是指针",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么在线程循环里常写 `let counter = Arc::clone(&counter);`？",
        code: "let counter = Arc::new(Mutex::new(0));\nlet worker_counter = Arc::clone(&counter);",
        options: &[
            "增加引用计数，让多个线程共享同一个 Mutex",
            "深拷贝 Mutex 里的数字",
            "把 Mutex 变成不可变",
        ],
        answer: Answer::Choice(0),
        explanation: "`Arc::clone` 只增加引用计数，不复制内部数据；多个 Arc 指向同一个 Mutex。",
        hint: "Arc 是 atomic reference counting。",
    },
    Exercise {
        id: "mutex-guard-drop",
        lesson_id: "concurrency",
        title: "MutexGuard 何时释放锁",
        kind: ExerciseKind::SingleChoice,
        prompt: "调用 `counter.lock().unwrap()` 得到的 guard 什么时候释放锁？",
        code: "let mut num = counter.lock().unwrap();\n*num += 1;",
        options: &["guard 离开作用域时自动释放", "调用 println! 时释放", "线程结束前永不释放"],
        answer: Answer::Choice(0),
        explanation: "MutexGuard 实现 Drop，离开作用域时自动释放锁，这是 Rust RAII 模式的一部分。",
        hint: "想想文件句柄、堆内存和锁在 Rust 中如何自动清理。",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_kind_labels_are_human_readable() {
        assert_eq!(ExerciseKind::SingleChoice.label(), "单选题");
        assert_eq!(ExerciseKind::FillBlank.label(), "填空题");
        assert_eq!(ExerciseKind::OrderSteps.label(), "排序题");
        assert_eq!(ExerciseKind::CodeOutput.label(), "输出判断");
    }

    #[test]
    fn finds_exercise_by_id() {
        let exercise = exercise_by_id("ownership-move").expect("exercise");

        assert_eq!(exercise.lesson_id, "ownership");
        assert_eq!(exercise.kind, ExerciseKind::SingleChoice);
        assert!(exercise_by_id("missing").is_none());
    }

    #[test]
    fn lists_exercises_for_lesson_in_curriculum_order() {
        let ids: Vec<&str> = exercises_for_lesson("syntax-basics")
            .into_iter()
            .map(|exercise| exercise.id)
            .collect();

        assert_eq!(ids, vec!["syntax-let-mut", "syntax-output"]);
        assert!(exercises_for_lesson("unknown").is_empty());
    }

    #[test]
    fn checks_choice_answers() {
        let exercise = exercise_by_id("match-exhaustive").expect("exercise");

        assert!(exercise.check(&UserAnswer::Choice(0)).correct);
        assert!(!exercise.check(&UserAnswer::Choice(1)).correct);
    }

    #[test]
    fn checks_text_answers_with_whitespace_normalization() {
        let exercise = exercise_by_id("if-let-some").expect("exercise");

        assert!(
            exercise
                .check(&UserAnswer::Text("Some(value)".to_owned()))
                .correct
        );
        assert!(
            exercise
                .check(&UserAnswer::Text("  Some(value)  ".to_owned()))
                .correct
        );
        assert!(
            !exercise
                .check(&UserAnswer::Text("Some(name)".to_owned()))
                .correct
        );
    }

    #[test]
    fn checks_ordered_answers() {
        let exercise = exercise_by_id("borrowing-mut-ref").expect("exercise");
        let correct = vec![
            "let mut nums = vec![1, 2];".to_owned(),
            "let view = &mut nums;".to_owned(),
            "view.push(3);".to_owned(),
            "println!(\"{:?}\", nums);".to_owned(),
        ];
        let wrong = vec![
            "let view = &mut nums;".to_owned(),
            "let mut nums = vec![1, 2];".to_owned(),
            "view.push(3);".to_owned(),
            "println!(\"{:?}\", nums);".to_owned(),
        ];

        assert!(exercise.check(&UserAnswer::Ordered(correct)).correct);
        assert!(!exercise.check(&UserAnswer::Ordered(wrong)).correct);
    }

    #[test]
    fn checks_output_answers_without_trailing_newline_noise() {
        let exercise = exercise_by_id("syntax-output").expect("exercise");

        assert!(
            exercise
                .check(&UserAnswer::Output("42\n".to_owned()))
                .correct
        );
        assert!(!exercise.check(&UserAnswer::Output("41".to_owned())).correct);
    }

    #[test]
    fn mismatched_answer_kind_is_incorrect() {
        let exercise = exercise_by_id("syntax-output").expect("exercise");

        assert!(!exercise.check(&UserAnswer::Choice(0)).correct);
    }

    #[test]
    fn answer_summary_is_clear() {
        assert_eq!(Answer::Choice(2).summary(), "选择第 3 项");
        assert_eq!(Answer::Text("mut").summary(), "mut");
        assert_eq!(Answer::Ordered(&["a", "b", "c"]).summary(), "a -> b -> c");
        assert_eq!(Answer::Output("hello\n").summary(), "hello");
    }

    #[test]
    fn every_exercise_has_valid_static_data() {
        let mut ids = Vec::new();

        for exercise in exercises() {
            assert!(!exercise.id.is_empty());
            assert!(!ids.contains(&exercise.id));
            assert!(!exercise.lesson_id.is_empty());
            assert!(!exercise.prompt.is_empty());
            assert!(!exercise.explanation.is_empty());
            assert!(!exercise.hint.is_empty());
            ids.push(exercise.id);
        }
    }
}
