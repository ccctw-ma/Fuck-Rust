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
