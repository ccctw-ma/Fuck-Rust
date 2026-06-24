use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseKind {
    SingleChoice,
    FillBlank,
    OrderSteps,
    CodeOutput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExerciseDifficulty {
    Basic,
    Practice,
    Challenge,
}

impl ExerciseDifficulty {
    pub const fn level(self) -> u8 {
        match self {
            ExerciseDifficulty::Basic => 1,
            ExerciseDifficulty::Practice => 2,
            ExerciseDifficulty::Challenge => 3,
        }
    }
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
    pub fn difficulty(&self) -> ExerciseDifficulty {
        if self.id.contains("-challenge-") || self.id.starts_with("advanced-") {
            return ExerciseDifficulty::Challenge;
        }

        if self.id.contains("-practice-") {
            return ExerciseDifficulty::Practice;
        }

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
            | "channel-send" => ExerciseDifficulty::Practice,
            "dangling-reference"
            | "lifetime-meaning"
            | "iter-vs-into-iter"
            | "collect-type"
            | "lifetime-longest"
            | "where-clause"
            | "static-lifetime-myth"
            | "mutex-lock"
            | "arc-clone"
            | "mutex-guard-drop" => ExerciseDifficulty::Challenge,
            _ => ExerciseDifficulty::Basic,
        }
    }

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
        self.difficulty().level()
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
    exercises().iter().find(|exercise| exercise.id == id)
}

pub fn exercises_for_lesson(lesson_id: &str) -> Vec<&'static Exercise> {
    let mut lesson_exercises: Vec<(usize, &Exercise)> = exercises()
        .iter()
        .enumerate()
        .filter(|(_, exercise)| exercise.lesson_id == lesson_id)
        .collect();
    lesson_exercises.sort_by_key(|(index, exercise)| (exercise.level(), *index));

    let mut concept_keys = Vec::new();
    lesson_exercises
        .into_iter()
        .filter_map(|(_, exercise)| {
            let concept_key = exercise_concept_key(exercise);
            if concept_keys.iter().any(|seen| seen == &concept_key) {
                None
            } else {
                concept_keys.push(concept_key);
                Some(exercise)
            }
        })
        .collect()
}

pub fn exercises() -> &'static [Exercise] {
    EXERCISES.get_or_init(build_exercises)
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

static EXERCISES: OnceLock<Box<[Exercise]>> = OnceLock::new();

fn build_exercises() -> Box<[Exercise]> {
    let mut exercises = Vec::with_capacity(CORE_EXERCISES.len() + generated_drill_count());
    exercises.extend(CORE_EXERCISES.iter().copied());
    exercises.extend(build_generated_drills());
    exercises.into_boxed_slice()
}

fn generated_drill_count() -> usize {
    DRILL_PLANS
        .iter()
        .map(|plan| plan.basic.len() + plan.practice.len() + plan.challenge.len())
        .sum()
}

fn build_generated_drills() -> Vec<Exercise> {
    let mut drills = Vec::with_capacity(generated_drill_count());
    for plan in DRILL_PLANS {
        push_drill_group(
            &mut drills,
            plan.lesson_id,
            ExerciseDifficulty::Basic,
            plan.basic,
        );
        push_drill_group(
            &mut drills,
            plan.lesson_id,
            ExerciseDifficulty::Practice,
            plan.practice,
        );
        push_drill_group(
            &mut drills,
            plan.lesson_id,
            ExerciseDifficulty::Challenge,
            plan.challenge,
        );
    }
    drills
}

fn push_drill_group(
    drills: &mut Vec<Exercise>,
    lesson_id: &'static str,
    difficulty: ExerciseDifficulty,
    specs: &'static [DrillSpec],
) {
    for (index, spec) in specs.iter().enumerate() {
        let difficulty_slug = match difficulty {
            ExerciseDifficulty::Basic => "basic",
            ExerciseDifficulty::Practice => "practice",
            ExerciseDifficulty::Challenge => "challenge",
        };
        let difficulty_name = match difficulty {
            ExerciseDifficulty::Basic => "基础",
            ExerciseDifficulty::Practice => "进阶",
            ExerciseDifficulty::Challenge => "挑战",
        };
        let ordinal = index + 1;
        let id = leak(format!("drill-{lesson_id}-{difficulty_slug}-{ordinal:02}"));
        let title = leak(format!("{difficulty_name}训练：{}", spec.concept));
        let prompt = leak(format!("围绕“{}”选择最准确的 Rust 判断。", spec.concept));
        let code = leak(spec.code.to_owned());
        let options = Box::leak(
            vec![
                spec.correct,
                spec.misconception,
                "这部分完全由运行时决定，编译器无法提供帮助",
            ]
            .into_boxed_slice(),
        );
        let explanation = leak(format!(
            "{}。这是本知识点的{}题，用来把概念从识别推进到可应用。",
            spec.explanation, difficulty_name
        ));
        let hint = leak(format!(
            "先判断代码里的所有权、类型或模式关系，再排除“自动/运行时魔法”的说法。关键词：{}。",
            spec.concept
        ));

        drills.push(Exercise {
            id,
            lesson_id,
            title,
            kind: ExerciseKind::SingleChoice,
            prompt,
            code,
            options,
            answer: Answer::Choice(0),
            explanation,
            hint,
        });
    }
}

fn exercise_concept_key(exercise: &Exercise) -> String {
    if let Some(key) = known_concept_key(exercise.id) {
        return key.to_owned();
    }

    let title = exercise
        .title
        .strip_prefix("基础训练：")
        .or_else(|| exercise.title.strip_prefix("进阶训练："))
        .or_else(|| exercise.title.strip_prefix("挑战训练："))
        .or_else(|| exercise.title.strip_prefix("高阶："))
        .unwrap_or(exercise.title);

    normalize_concept(title)
}

fn known_concept_key(id: &str) -> Option<&'static str> {
    match id {
        "syntax-shadowing" | "syntax-shadow-mutability" => Some("syntax-shadowing"),
        "array-type" => Some("array-type-length"),
        "advanced-data-tuple-trailing-comma" => Some("single-element-tuple"),
        _ => None,
    }
}

fn normalize_concept(input: &str) -> String {
    input
        .chars()
        .filter(|character| character.is_alphanumeric() || is_cjk(*character))
        .flat_map(char::to_lowercase)
        .collect()
}

fn is_cjk(character: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&character)
}

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

#[derive(Clone, Copy)]
struct DrillSpec {
    concept: &'static str,
    code: &'static str,
    correct: &'static str,
    misconception: &'static str,
    explanation: &'static str,
}

struct DrillPlan {
    lesson_id: &'static str,
    basic: &'static [DrillSpec],
    practice: &'static [DrillSpec],
    challenge: &'static [DrillSpec],
}

macro_rules! drill {
    ($concept:expr, $code:expr, $correct:expr, $misconception:expr, $explanation:expr $(,)?) => {
        DrillSpec {
            concept: $concept,
            code: $code,
            correct: $correct,
            misconception: $misconception,
            explanation: $explanation,
        }
    };
}

const SYNTAX_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "不可变绑定",
        "let score = 41;",
        "不写 mut 的绑定默认不可变",
        "let 绑定默认可以重新赋值",
        "Rust 默认不可变，修改同一绑定必须显式 mut"
    ),
    drill!(
        "mut 重新赋值",
        "let mut score = 41;\nscore += 1;",
        "mut 允许同类型的新值重新赋给同一绑定",
        "mut 会允许变量变成任意类型",
        "mut 只放开可变性，不放开静态类型约束"
    ),
    drill!(
        "shadowing",
        "let spaces = \"   \";\nlet spaces = spaces.len();",
        "再次 let 会创建新绑定，因此可以改变类型",
        "shadowing 会修改原绑定的类型",
        "shadowing 是新绑定覆盖旧名字，不是原地修改"
    ),
    drill!(
        "block 表达式",
        "let value = { let n = 1; n + 1 };",
        "代码块最后无分号表达式会成为块的值",
        "代码块永远返回 unit",
        "Rust 中块可以是表达式"
    ),
];
const SYNTAX_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "const 类型标注",
        "const MAX: u32 = 100;",
        "const 必须显式写类型",
        "const 可以完全依赖类型推断",
        "常量是编译期值，签名需要清晰类型"
    ),
    drill!(
        "分号丢弃值",
        "let value = { 42; };",
        "分号把表达式变成语句，块值是 ()",
        "分号会让块返回 42",
        "语句没有可继续传递的表达式值"
    ),
    drill!(
        "println! 捕获变量",
        "let name = \"Ferris\";\nprintln!(\"{name}\");",
        "Rust 2021 格式串可以按名字捕获变量",
        "必须总是把 name 作为额外参数传入",
        "格式宏支持捕获同名变量"
    ),
    drill!(
        "类型推断",
        "let mut x = 1;\nx = 2;",
        "推断出的类型固定，后续赋值仍需同类型",
        "推断意味着变量是动态类型",
        "Rust 是静态类型语言"
    ),
];
const SYNTAX_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "never 类型",
        "let value: i32 = panic!(\"boom\");",
        "panic! 的 never 类型可强制转换到需要的类型",
        "panic! 会返回默认 i32",
        "never 表示永不正常返回"
    ),
    drill!(
        "宏调用",
        "println!(\"hi\");",
        "println! 是宏，调用名后需要感叹号",
        "感叹号表示函数会 panic",
        "宏和函数调用语法不同"
    ),
    drill!(
        "数字后缀",
        "let n = 10u8;",
        "u8 后缀直接指定数字字面量类型",
        "u8 后缀表示变量可变",
        "字面量后缀参与类型确定"
    ),
];

const CONTROL_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "match 穷尽",
        "match n { 0 => \"zero\", _ => \"many\" }",
        "match 必须覆盖输入类型的所有可能",
        "match 可以只写运行时遇到的分支",
        "穷尽性是 Rust 模式匹配的核心安全网"
    ),
    drill!(
        "通配模式",
        "_ => \"fallback\"",
        "_ 会匹配前面没有覆盖的剩余情况",
        "_ 只匹配 None",
        "通配模式常用于兜底"
    ),
    drill!(
        "if let",
        "if let Some(n) = maybe { println!(\"{n}\"); }",
        "if let 适合只关心一种模式",
        "if let 必须覆盖所有模式",
        "只关心成功路径时 if let 更轻量"
    ),
    drill!(
        "if 表达式",
        "let label = if ok { \"yes\" } else { \"no\" };",
        "if/else 可以产生一个值",
        "if 只能用于控制语句不能赋值",
        "Rust 的 if 是表达式"
    ),
];
const CONTROL_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "分支类型一致",
        "let x = if flag { 1 } else { 2 };",
        "if 两个分支需要产出兼容类型",
        "分支类型可以完全不同并自动转换",
        "表达式最终必须有一个明确类型"
    ),
    drill!(
        "match guard",
        "match n { x if x > 0 => \"positive\", _ => \"other\" }",
        "guard 会在模式匹配后再检查条件",
        "guard 会让分支忽略模式",
        "guard 用来细化匹配条件"
    ),
    drill!(
        "while let",
        "while let Some(x) = stack.pop() { println!(\"{x}\"); }",
        "while let 会在模式持续匹配时循环",
        "while let 只会执行一次",
        "它把循环和模式匹配合在一起"
    ),
    drill!(
        "loop break 值",
        "let n = loop { break 7; };",
        "loop 可以通过 break value 产生值",
        "loop 永远不能返回值",
        "break 后的值成为 loop 表达式结果"
    ),
];
const CONTROL_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "@ 绑定",
        "id @ 3..=7 => id",
        "@ 可以在检查范围时保留实际匹配值",
        "@ 会创建闭区间",
        "@ 同时完成子模式匹配和绑定"
    ),
    drill!(
        "枚举解构",
        "Message::Move { x, y } => (x, y)",
        "match 可以直接解构枚举变体携带的数据",
        "枚举数据必须先手动取字段",
        "模式能把结构拆开并绑定字段"
    ),
    drill!(
        "matches! 宏",
        "matches!(value, Some(n) if n > 2)",
        "matches! 返回表达式是否匹配模式的 bool",
        "matches! 会解包并移动所有权给调用方",
        "它适合把模式判断写成布尔表达式"
    ),
];

const DATA_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "元组解构",
        "let (x, y) = (1, 2);",
        "元组可以用模式按位置解构",
        "元组只能通过索引访问",
        "模式绑定能一次取出多个位置"
    ),
    drill!(
        "数组类型",
        "let xs: [i32; 3] = [1, 2, 3];",
        "数组类型包含元素类型和固定长度",
        "数组类型只写元素类型即可",
        "长度是数组类型的一部分"
    ),
    drill!(
        "函数参数类型",
        "fn double(x: i32) -> i32 { x * 2 }",
        "函数参数必须写明类型",
        "Rust 会从函数体自动推断参数类型",
        "函数签名需要显式参数类型"
    ),
    drill!(
        "尾表达式返回",
        "fn answer() -> i32 { 42 }",
        "函数体最后无分号表达式可作为返回值",
        "Rust 函数必须使用 return",
        "尾表达式是 Rust 常见返回写法"
    ),
];
const DATA_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "unit 返回",
        "fn log() { println!(\"hi\"); }",
        "没有返回类型时默认返回 ()",
        "println! 会让函数返回字符串",
        "副作用函数通常返回 unit"
    ),
    drill!(
        "数组重复初始化",
        "let xs = [3; 5];",
        "[value; count] 会创建固定长度数组",
        "[3; 5] 表示两个元素 3 和 5",
        "分号语法代表重复初始化"
    ),
    drill!(
        "char 类型",
        "let c = '🦀';",
        "char 表示 Unicode 标量值",
        "char 等同于一个 UTF-8 字节",
        "Rust char 不是 C 风格 byte"
    ),
    drill!(
        "整数溢出",
        "let x: u8 = 255;",
        "debug 构建通常会检查整数溢出",
        "整数溢出总是自动变大类型",
        "溢出策略和构建模式及显式方法有关"
    ),
];
const DATA_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "发散函数",
        "fn fail() -> ! { panic!(\"nope\") }",
        "返回 ! 表示函数永不正常返回",
        "! 是空元组的别名",
        "never type 表达控制流不会回来"
    ),
    drill!(
        "单元素元组",
        "let one = (5,);",
        "单元素元组必须有尾随逗号",
        "(5) 就是单元素元组",
        "没有逗号只是括号表达式"
    ),
    drill!(
        "非 Copy 重复数组",
        "let xs = [String::from(\"x\"); 3];",
        "重复初始化通常要求元素可 Copy",
        "String 会自动深拷贝三份",
        "非 Copy 值不能这样隐式复制"
    ),
];

const OWNERSHIP_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "String move",
        "let b = a;",
        "String 赋值通常移动所有权",
        "String 赋值默认深拷贝",
        "拥有堆资源的值默认 move"
    ),
    drill!(
        "Copy 类型",
        "let b = a; // a: i32",
        "i32 这类标量赋值后原变量仍可用",
        "所有类型赋值都会让原变量失效",
        "Copy 类型按位复制且保留原绑定可用"
    ),
    drill!(
        "clone",
        "let b = a.clone();",
        "clone 显式复制需要的资源",
        "clone 是零成本借用",
        "Rust 要求深拷贝显式发生"
    ),
    drill!(
        "drop 时机",
        "{ let s = String::from(\"hi\"); }",
        "owner 离开作用域时资源自动 drop",
        "Rust 需要手动 free String",
        "所有权负责确定释放时机"
    ),
];
const OWNERSHIP_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "函数取得所有权",
        "fn take(s: String) {}",
        "按值传 String 会把所有权移动进函数",
        "函数参数默认只借用",
        "参数也是新的 owner"
    ),
    drill!(
        "借用避免 move",
        "fn len(s: &String) -> usize { s.len() }",
        "传引用可读取而不取得所有权",
        "传引用会复制整段字符串",
        "引用是借用视图"
    ),
    drill!(
        "返回所有权",
        "fn id(s: String) -> String { s }",
        "返回值可以把所有权交还调用方",
        "函数结束一定销毁所有参数",
        "move 可以沿返回值继续传递"
    ),
    drill!(
        "重新赋值",
        "s = String::from(\"new\");",
        "重新赋值会 drop 旧值并让变量拥有新值",
        "旧值会自动泄漏",
        "Rust 会清理被替换的资源"
    ),
];
const OWNERSHIP_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "部分 move",
        "let name = person.name;",
        "移出非 Copy 字段会让整体不再完整可用",
        "结构体字段移动会复制整个结构体",
        "部分 move 会影响剩余可用性"
    ),
    drill!(
        "mem::take",
        "let old = std::mem::take(&mut s);",
        "take 用默认值替换原位置并移出旧值",
        "take 会 clone 原值",
        "它常用于从可变引用中拿出 owned 值"
    ),
    drill!(
        "move 闭包",
        "let f = move || s.len();",
        "move 闭包会捕获并拥有用到的变量",
        "move 只影响闭包执行顺序",
        "闭包环境也参与所有权移动"
    ),
];

const SLICES_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "字符串切片",
        "let part = &s[0..5];",
        "&str 是对字符串一段内容的借用",
        "&str 会复制出新 String",
        "切片不拥有底层数据"
    ),
    drill!(
        "右开区间",
        "&s[0..5]",
        "范围右边界不包含在切片内",
        "0..5 会包含索引 5",
        "Rust 范围通常是左闭右开"
    ),
    drill!(
        "完整切片",
        "let all = &s[..];",
        ".. 可以表示从开始到结束",
        ".. 会清空字符串",
        "省略边界代表完整范围"
    ),
    drill!(
        "字符串字面量",
        "let s = \"hello\";",
        "字符串字面量的类型是 &str",
        "字符串字面量默认是 String",
        "字面量借用程序中的静态数据"
    ),
];
const SLICES_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "first_word 返回 &str",
        "fn first_word(s: &str) -> &str",
        "返回切片能把结果和输入借用关系绑定",
        "返回 usize 总是更安全",
        "切片让编译器维护有效性"
    ),
    drill!(
        "参数接收 &str",
        "fn print(s: &str) {}",
        "&str 比 &String 更通用",
        "&String 可以接收所有字符串形式",
        "API 应优先接收切片"
    ),
    drill!(
        "数组切片",
        "let part = &nums[1..3];",
        "数组切片类型是 &[T]",
        "切片类型固定包含长度",
        "切片是对连续片段的借用"
    ),
    drill!(
        "UTF-8 边界",
        "let part = &s[0..1];",
        "字符串切片必须落在 UTF-8 字符边界",
        "字符串可以按任意字节切",
        "Rust 字符串索引是字节边界约束"
    ),
];
const SLICES_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "split_at_mut",
        "let (a, b) = xs.split_at_mut(2);",
        "它保证两个可变切片不重叠",
        "它关闭了借用检查",
        "安全 API 通过不重叠满足独占规则"
    ),
    drill!(
        "切片模式",
        "[first, .., last]",
        "该模式匹配至少两个元素并绑定首尾",
        "该模式只匹配正好两个元素",
        ".. 能吸收中间任意元素"
    ),
    drill!(
        "UTF-8 字节",
        "\"é\".as_bytes().len()",
        "é 的 UTF-8 字节长度是 2",
        "所有 char 都占 1 字节",
        "字节长度和字符个数不同"
    ),
];

const BORROW_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "不可变引用",
        "let r = &s;",
        "不可变引用允许读取但不能修改",
        "不可变引用拥有 s",
        "&T 是共享借用"
    ),
    drill!(
        "可变引用",
        "let r = &mut s;",
        "可变引用允许通过借用修改值",
        "&mut 会复制一份值",
        "&mut T 是独占借用"
    ),
    drill!(
        "多个读",
        "let a = &s; let b = &s;",
        "多个不可变引用可以同时存在",
        "任何引用都只能有一个",
        "只读共享不会破坏数据"
    ),
    drill!(
        "一个写",
        "let r = &mut s;",
        "同一时间只能有一个活跃可变引用",
        "mut 变量可以有无限多个 &mut",
        "独占写避免数据竞争和别名写入"
    ),
];
const BORROW_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "NLL 最后使用",
        "println!(\"{r}\");\ns.push('!');",
        "引用最后一次使用后借用即可结束",
        "借用一定持续到花括号结尾",
        "非词法生命周期能缩短借用"
    ),
    drill!(
        "函数可变参数",
        "fn change(s: &mut String) {}",
        "调用方需要传入 &mut value",
        "函数能自动把不可变引用升级",
        "可变借用在签名和调用处都显式"
    ),
    drill!(
        "被借用时不能 move",
        "let r = &s; let moved = s;",
        "活跃引用存在时不能移动 owner",
        "move 会自动更新所有引用",
        "引用不能指向已移动的值"
    ),
    drill!(
        "悬垂引用",
        "fn dangle() -> &String { let s = String::new(); &s }",
        "不能返回指向局部变量的引用",
        "编译器会把局部变量提升成 static",
        "Rust 拒绝悬垂引用"
    ),
];
const BORROW_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "两阶段借用",
        "v.push(v.len());",
        "方法调用可先计算参数再激活可变借用",
        "push 不需要可变借用",
        "两阶段借用支持常见方法调用模式"
    ),
    drill!(
        "可变重借用",
        "touch(r); r.push('!');",
        "传 &mut 参数常发生临时重借用",
        "&mut 引用传参后必然被移动",
        "重借用结束后原引用可继续使用"
    ),
    drill!(
        "RefCell",
        "RefCell::new(1)",
        "RefCell 把借用规则推迟到运行时检查",
        "RefCell 取消所有借用规则",
        "违反规则会 panic 而非编译错误"
    ),
];

const STRUCT_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "结构体字段",
        "User { email, active: true }",
        "结构体用字段名组织相关数据",
        "结构体字段只能按位置访问",
        "命名字段提升可读性"
    ),
    drill!(
        "字段初始化简写",
        "User { email, username }",
        "变量名和字段名相同可简写",
        "简写会忽略该字段",
        "field init shorthand 是常见语法"
    ),
    drill!(
        "方法 &self",
        "fn area(&self) -> u32",
        "只读方法通常接收 &self",
        "所有方法都必须消费 self",
        "&self 是 self: &Self 的简写"
    ),
    drill!(
        "枚举变体",
        "enum Message { Quit, Write(String) }",
        "枚举不同变体可以携带不同数据",
        "枚举所有变体必须字段相同",
        "Rust 枚举是代数数据类型"
    ),
];
const STRUCT_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "结构体更新",
        "User { email, ..user1 }",
        "..user1 会从 user1 取得剩余字段",
        "..user1 会借用所有字段",
        "非 Copy 字段可能被移动"
    ),
    drill!(
        "&mut self",
        "fn inc(&mut self)",
        "修改字段的方法需要 &mut self",
        "&self 可以直接修改字段",
        "接收者可变性决定能否修改"
    ),
    drill!(
        "调试打印",
        "#[derive(Debug)]",
        "{:?} 打印自定义类型通常需要 Debug",
        "{} 和 {:?} 都使用 Display",
        "Debug 面向开发调试输出"
    ),
    drill!(
        "if let 枚举",
        "if let Message::Quit = msg",
        "if let 可只处理一个枚举变体",
        "if let 必须列出所有变体",
        "单分支模式处理用 if let 更简洁"
    ),
];
const STRUCT_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "枚举大小",
        "enum E { A(u8), B([u8; 1024]) }",
        "同一枚举类型的值需要统一大小",
        "每个变体总是单独堆分配",
        "最大变体会影响枚举大小"
    ),
    drill!(
        "Option niche",
        "Option<&T>",
        "None 通常可用空指针 niche 表示",
        "Option<&T> 一定比 &T 大两倍",
        "非空引用给优化留下编码空间"
    ),
    drill!(
        "模式 ref",
        "User { name: ref name, age }",
        "模式中的 ref 会借用字段避免移动",
        "ref 会把字段克隆一份",
        "模式 ref 和表达式 & 位置不同"
    ),
];

const RESULT_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "Option Some",
        "Some(3)",
        "Some 表示存在一个值",
        "Some 表示错误",
        "Option 用枚举显式表达有无"
    ),
    drill!(
        "Option None",
        "None",
        "None 表示没有值",
        "None 等同于空字符串",
        "Rust 没有 null，使用 None"
    ),
    drill!(
        "Result Ok",
        "Ok(value)",
        "Ok 表示成功结果",
        "Ok 表示默认值",
        "Result 区分成功和失败"
    ),
    drill!(
        "Result Err",
        "Err(error)",
        "Err 携带失败信息",
        "Err 会自动被忽略",
        "错误是类型系统的一部分"
    ),
];
const RESULT_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "问号传播 Result",
        "let n = raw.parse::<i32>()?;",
        "? 在 Err 时提前返回错误",
        "? 会捕获错误并继续执行",
        "? 是错误传播语法糖"
    ),
    drill!(
        "unwrap 风险",
        "value.unwrap()",
        "unwrap 在 None/Err 时会 panic",
        "unwrap 会自动修复错误",
        "它适合示例或确定成功的场景"
    ),
    drill!(
        "map 转换",
        "maybe.map(|n| n + 1)",
        "map 只转换 Some/Ok 中的值",
        "map 会同时处理错误分支",
        "组合子让成功路径更简洁"
    ),
    drill!(
        "and_then",
        "maybe.and_then(parse)",
        "and_then 的闭包返回 Option/Result 并自动扁平化",
        "and_then 会制造嵌套 Option",
        "它适合连续可能失败的步骤"
    ),
];
const RESULT_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "transpose",
        "Option<Result<T, E>>::transpose()",
        "transpose 可翻转成 Result<Option<T>, E>",
        "transpose 会丢弃错误",
        "它常用于可选字段解析"
    ),
    drill!(
        "ok_or_else",
        "maybe.ok_or_else(|| make_error())",
        "ok_or_else 只在 None 时构造错误",
        "ok_or_else 总是立即构造错误",
        "闭包让错误构造延迟发生"
    ),
    drill!(
        "库错误设计",
        "fn load() -> Result<Config, Error>",
        "可恢复失败应返回 Result",
        "库函数应优先 panic",
        "Result 让调用方决定恢复策略"
    ),
];

const COLLECTION_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "Vec 可增长",
        "let mut v = vec![1]; v.push(2);",
        "Vec 是可增长集合，push 需要 mut",
        "Vec 长度写入类型后不可增长",
        "Vec 管理堆上连续元素"
    ),
    drill!(
        "String 可增长",
        "let mut s = String::from(\"hi\");",
        "String 拥有可增长 UTF-8 字符串",
        "String 等同于 &'static str",
        "String 和 &str 所有权不同"
    ),
    drill!(
        "HashMap key",
        "map.insert(key, value);",
        "HashMap 通过 key 查找 value",
        "HashMap 保持插入顺序",
        "哈希表关注 key 到 value 的映射"
    ),
    drill!(
        "get 返回 Option",
        "values.get(index)",
        "get 可能越界所以返回 Option",
        "get 越界会 panic",
        "Option 强制处理缺失"
    ),
];
const COLLECTION_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "entry or_insert",
        "map.entry(k).or_insert(0)",
        "or_insert 只在 key 缺失时插入默认值",
        "or_insert 总会覆盖旧值",
        "entry API 合并查找和插入"
    ),
    drill!(
        "遍历借用",
        "for item in &values",
        "遍历 &values 不会消费 Vec",
        "for 总会 move 集合",
        "选择 iter/into_iter 决定所有权"
    ),
    drill!(
        "insert 覆盖",
        "map.insert(\"Blue\", 25);",
        "相同 key 再 insert 会覆盖旧值",
        "HashMap 会保存两个相同 key",
        "key 在 map 中唯一"
    ),
    drill!(
        "String push",
        "s.push('!'); s.push_str(\"hi\");",
        "push 加 char，push_str 加 &str",
        "push 可以追加任意字符串切片",
        "两种方法参数类型不同"
    ),
];
const COLLECTION_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "and_modify",
        "entry(k).and_modify(|v| *v += 1).or_insert(1)",
        "and_modify 只在 key 已存在时运行",
        "and_modify 会无条件插入新 key",
        "它适合更新或插入的链式逻辑"
    ),
    drill!(
        "drain",
        "v.drain(1..3)",
        "drain 会移除范围并产生被移除元素",
        "drain 只是不可变查看范围",
        "drain 会修改原 Vec"
    ),
    drill!(
        "key move",
        "map.insert(key, 1);",
        "String key 会移动进 HashMap",
        "insert 会自动 clone key",
        "按值插入会转移所有权"
    ),
];

const ITER_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "iter 借用",
        "values.iter()",
        "iter 通常产生元素引用",
        "iter 会消费集合",
        "借用迭代后集合仍可使用"
    ),
    drill!(
        "into_iter 所有权",
        "values.into_iter()",
        "into_iter 通常消费集合并产出 owned 元素",
        "into_iter 只是借用",
        "into 表示所有权转换"
    ),
    drill!(
        "map",
        "iter.map(|n| n * 2)",
        "map 描述逐项转换",
        "map 会立刻执行所有闭包",
        "适配器通常是惰性的"
    ),
    drill!(
        "filter",
        "iter.filter(|n| **n > 0)",
        "filter 保留满足谓词的元素",
        "filter 会改变原集合",
        "迭代器链通常不修改源集合"
    ),
];
const ITER_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "惰性",
        "values.iter().map(|n| n + 1);",
        "没有消费适配器时 map 不会真正运行",
        "创建 map 后闭包立即全部执行",
        "迭代器按需拉取元素"
    ),
    drill!(
        "collect 目标类型",
        "iter.collect::<Vec<_>>()",
        "collect 通常需要目标集合类型",
        "collect 总是返回 Vec<String>",
        "FromIterator 目标由类型决定"
    ),
    drill!(
        "sum 消费",
        "let total: i32 = iter.sum();",
        "sum 会消费迭代器",
        "sum 后 iter 可从头再来",
        "消费适配器按值接收 self"
    ),
    drill!(
        "闭包捕获",
        "map(|n| n * factor)",
        "闭包可以捕获外部变量",
        "闭包不能访问外部作用域",
        "闭包比函数指针更灵活"
    ),
];
const ITER_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "flat_map",
        "iter.flat_map(|v| v)",
        "flat_map 先映射成迭代器再展平",
        "flat_map 只返回第一个元素",
        "它等价于 map 后 flatten 的常见组合"
    ),
    drill!(
        "by_ref",
        "iter.by_ref().take(2)",
        "by_ref 让适配器暂时借用迭代器",
        "by_ref 会克隆整个迭代器",
        "适合分段消费同一个迭代器"
    ),
    drill!(
        "partition",
        "iter.partition(|n| n % 2 == 0)",
        "partition 按谓词分成两个集合",
        "partition 只返回 bool",
        "它会消费迭代器并收集两组结果"
    ),
];

const GENERICS_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "泛型参数",
        "fn id<T>(value: T) -> T",
        "T 代表由调用处确定的类型参数",
        "T 表示运行时任意类型",
        "泛型在编译期单态化"
    ),
    drill!(
        "trait bound",
        "T: Display",
        "trait bound 声明泛型必须具备的能力",
        "bound 会自动实现 trait",
        "使用能力前要约束能力"
    ),
    drill!(
        "impl Trait",
        "fn notify(item: impl Display)",
        "impl Trait 可简化参数位置的 trait 约束",
        "impl Trait 表示动态类型变量",
        "它仍是静态分发语法"
    ),
    drill!(
        "Debug vs Display",
        "println!(\"{:?}\", value)",
        "{:?} 需要 Debug",
        "{:?} 需要 Display",
        "格式化占位符对应不同 trait"
    ),
];
const GENERICS_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "where 子句",
        "where T: Display + Clone",
        "where 让复杂约束更清晰",
        "where 会改变函数运行时性能",
        "它只是约束书写位置不同"
    ),
    drill!(
        "生命周期关系",
        "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str",
        "'a 描述输入和输出引用的有效期关系",
        "'a 会延长两个字符串生命",
        "生命周期标注不改变实际存活时间"
    ),
    drill!(
        "生命周期省略",
        "fn first(s: &str) -> &str",
        "简单引用关系可按省略规则补全",
        "省略代表没有生命周期检查",
        "编译器按规则推断而不是取消约束"
    ),
    drill!(
        "结构体引用字段",
        "struct Excerpt<'a> { part: &'a str }",
        "保存引用的结构体需要生命周期参数",
        "所有结构体都必须写生命周期",
        "引用字段需要表达有效期关系"
    ),
];
const GENERICS_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "关联类型",
        "Iterator<Item = i32>",
        "Item 是 Iterator trait 的关联类型",
        "Item 是生命周期名",
        "关联类型描述实现者产出的类型"
    ),
    drill!(
        "blanket impl",
        "impl<T: Display> ToString for T",
        "为所有满足约束的 T 统一实现叫 blanket impl",
        "这是 orphan rule",
        "标准库大量使用 blanket implementation"
    ),
    drill!(
        "PhantomData",
        "PhantomData<&'a T>",
        "PhantomData 表达逻辑上的类型/生命周期关系",
        "PhantomData 会分配 T",
        "零大小标记可影响所有权和变型分析"
    ),
];

const CONCURRENCY_BASIC_DRILLS: &[DrillSpec] = &[
    drill!(
        "spawn",
        "thread::spawn(|| println!(\"hi\"))",
        "spawn 会创建新线程执行闭包",
        "spawn 会在当前线程同步执行",
        "新线程可能与当前线程并发运行"
    ),
    drill!(
        "join",
        "handle.join().unwrap()",
        "join 等待子线程结束",
        "join 会强制杀死线程",
        "join 用于同步线程完成"
    ),
    drill!(
        "channel send",
        "tx.send(value)",
        "send 会把消息所有权发送给接收端",
        "send 总是 clone 消息",
        "消息传递也遵守所有权"
    ),
    drill!(
        "recv",
        "rx.recv()",
        "recv 会阻塞等待消息或通道关闭",
        "recv 永远立即返回默认值",
        "接收端显式处理消息到达"
    ),
];
const CONCURRENCY_PRACTICE_DRILLS: &[DrillSpec] = &[
    drill!(
        "move 线程闭包",
        "thread::spawn(move || println!(\"{:?}\", v))",
        "move 让线程闭包拥有捕获值",
        "move 会复制所有捕获值",
        "避免线程引用已结束的栈数据"
    ),
    drill!(
        "Arc",
        "Arc::clone(&counter)",
        "Arc 提供线程安全引用计数共享所有权",
        "Arc 会深拷贝内部数据",
        "跨线程共享通常需要原子引用计数"
    ),
    drill!(
        "Mutex",
        "counter.lock().unwrap()",
        "Mutex 通过锁保护内部可变数据",
        "Mutex 让数据不需要同步也安全",
        "互斥访问避免同时写"
    ),
    drill!(
        "MutexGuard",
        "let guard = mutex.lock().unwrap();",
        "guard 离开作用域时释放锁",
        "锁必须手动 unlock",
        "RAII 管理锁释放"
    ),
];
const CONCURRENCY_CHALLENGE_DRILLS: &[DrillSpec] = &[
    drill!(
        "Send",
        "T: Send",
        "Send 表示所有权可安全转移到其他线程",
        "Send 表示 &T 可多线程共享",
        "Send 和 Sync 表达不同并发能力"
    ),
    drill!(
        "mpsc 多生产者",
        "let tx2 = tx.clone();",
        "克隆 Sender 可创建多个生产者",
        "clone Sender 会复制通道缓冲区",
        "mpsc 的 m 就是 multiple producer"
    ),
    drill!(
        "死锁",
        "A 锁 left 再 right，B 锁 right 再 left",
        "锁顺序不一致可能造成死锁",
        "Rust 编译器能消除所有死锁",
        "Rust 防数据竞争但不防所有逻辑并发 bug"
    ),
];

const DRILL_PLANS: &[DrillPlan] = &[
    DrillPlan {
        lesson_id: "syntax-basics",
        basic: SYNTAX_BASIC_DRILLS,
        practice: SYNTAX_PRACTICE_DRILLS,
        challenge: SYNTAX_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "control-flow",
        basic: CONTROL_BASIC_DRILLS,
        practice: CONTROL_PRACTICE_DRILLS,
        challenge: CONTROL_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "data-functions",
        basic: DATA_BASIC_DRILLS,
        practice: DATA_PRACTICE_DRILLS,
        challenge: DATA_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "ownership",
        basic: OWNERSHIP_BASIC_DRILLS,
        practice: OWNERSHIP_PRACTICE_DRILLS,
        challenge: OWNERSHIP_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "slices",
        basic: SLICES_BASIC_DRILLS,
        practice: SLICES_PRACTICE_DRILLS,
        challenge: SLICES_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "borrowing",
        basic: BORROW_BASIC_DRILLS,
        practice: BORROW_PRACTICE_DRILLS,
        challenge: BORROW_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "structs-enums",
        basic: STRUCT_BASIC_DRILLS,
        practice: STRUCT_PRACTICE_DRILLS,
        challenge: STRUCT_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "result-option",
        basic: RESULT_BASIC_DRILLS,
        practice: RESULT_PRACTICE_DRILLS,
        challenge: RESULT_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "collections",
        basic: COLLECTION_BASIC_DRILLS,
        practice: COLLECTION_PRACTICE_DRILLS,
        challenge: COLLECTION_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "iterators-traits",
        basic: ITER_BASIC_DRILLS,
        practice: ITER_PRACTICE_DRILLS,
        challenge: ITER_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "generics-traits",
        basic: GENERICS_BASIC_DRILLS,
        practice: GENERICS_PRACTICE_DRILLS,
        challenge: GENERICS_CHALLENGE_DRILLS,
    },
    DrillPlan {
        lesson_id: "concurrency",
        basic: CONCURRENCY_BASIC_DRILLS,
        practice: CONCURRENCY_PRACTICE_DRILLS,
        challenge: CONCURRENCY_CHALLENGE_DRILLS,
    },
];

const CORE_EXERCISES: &[Exercise] = &[
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
        id: "syntax-shadowing",
        lesson_id: "syntax-basics",
        title: "shadowing 会创建新绑定",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段 shadowing 代码会输出什么？",
        code: "let spaces = \"   \";\nlet spaces = spaces.len();\nprintln!(\"{spaces}\");",
        options: &[],
        answer: Answer::Output("3"),
        explanation: "第二个 `let spaces` 创建了一个新绑定，因此类型可以从 `&str` 变成 `usize`。",
        hint: "注意这里不是修改同一个变量，而是重新 `let` 了同名变量。",
    },
    Exercise {
        id: "syntax-println-placeholder",
        lesson_id: "syntax-basics",
        title: "println! 捕获变量名",
        kind: ExerciseKind::FillBlank,
        prompt: "按 Rust 2021 的格式化写法，空白处填什么才能打印 name？",
        code: "let name = \"Ferris\";\nprintln!(\"Hello, {____}!\");",
        options: &[],
        answer: Answer::Text("name"),
        explanation: "格式字符串里的 `{name}` 会捕获同名变量并调用格式化输出。",
        hint: "花括号里写变量名，不需要再传第二个参数。",
    },
    Exercise {
        id: "syntax-semicolon-unit",
        lesson_id: "syntax-basics",
        title: "分号会让表达式变成语句",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪个 block 的值是 `()`？",
        code: "let a = { 40 + 2 };\nlet b = { 40 + 2; };",
        options: &["`a` 的 block", "`b` 的 block", "两个 block 都是 42"],
        answer: Answer::Choice(1),
        explanation: "`40 + 2;` 带分号，是语句，block 没有尾表达式时值为 `()`。",
        hint: "Rust 中分号会丢弃表达式的值。",
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
        id: "statement-vs-expression",
        lesson_id: "data-functions",
        title: "语句没有返回值",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么这段函数不能按 `i32` 返回？",
        code: "fn answer() -> i32 {\n    42;\n}",
        options: &["`42;` 是语句，函数实际返回 `()`", "i32 不能作为返回类型", "函数必须写 return"],
        answer: Answer::Choice(0),
        explanation: "加了分号后 `42` 的值被丢弃，函数体最后没有尾表达式，所以返回 `()`。",
        hint: "把分号去掉，`42` 才是尾表达式。",
    },
    Exercise {
        id: "function-param-type",
        lesson_id: "data-functions",
        title: "函数参数必须写类型",
        kind: ExerciseKind::FillBlank,
        prompt: "Rust 函数参数需要显式类型，空白处应填什么？",
        code: "fn double(x: ____) -> i32 {\n    x * 2\n}",
        options: &[],
        answer: Answer::Text("i32"),
        explanation: "Rust 不会从函数体反推公开签名里的参数类型，参数类型必须写出来。",
        hint: "返回类型已经暗示这里用整数练习。",
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
        id: "match-return-type",
        lesson_id: "control-flow",
        title: "match 分支类型要一致",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面代码为什么不能通过编译？",
        code: "let flag = true;\nlet value = match flag {\n    true => 1,\n    false => \"zero\",\n};",
        options: &["两个分支返回类型不同", "match 不能匹配 bool", "false 分支永远不会执行"],
        answer: Answer::Choice(0),
        explanation: "`match` 是表达式，所有分支必须产出兼容类型；这里一个是整数，一个是字符串切片。",
        hint: "想想 value 最终应该是什么类型。",
    },
    Exercise {
        id: "if-expression-value",
        lesson_id: "control-flow",
        title: "if 也可以产生值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段 if 表达式会输出什么？",
        code: "let score = 80;\nlet level = if score >= 60 { \"pass\" } else { \"retry\" };\nprintln!(\"{level}\");",
        options: &[],
        answer: Answer::Output("pass"),
        explanation: "Rust 的 `if` 可以作为表达式赋值给变量，条件为 true 时产出 `\"pass\"`。",
        hint: "两个分支都是 `&str`，所以可以赋给 level。",
    },
    Exercise {
        id: "match-option-none",
        lesson_id: "control-flow",
        title: "match 处理 None",
        kind: ExerciseKind::FillBlank,
        prompt: "空白处分支应该填什么才能覆盖没有值的情况？",
        code: "let maybe: Option<i32> = None;\nlet n = match maybe {\n    Some(value) => value,\n    ____ => 0,\n};",
        options: &[],
        answer: Answer::Text("None"),
        explanation: "`Option` 只有 `Some` 和 `None` 两种变体，match 必须都覆盖。",
        hint: "这不是 `_` 练习，而是明确写出空值变体。",
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
        id: "ownership-borrow-read",
        lesson_id: "ownership",
        title: "借用读取避免 move",
        kind: ExerciseKind::FillBlank,
        prompt: "只想读取 `name` 而不移动所有权，调用函数时空白处应填什么？",
        code: "fn len(s: &String) -> usize { s.len() }\nlet name = String::from(\"Rust\");\nlet n = len(____name);\nprintln!(\"{name} {n}\");",
        options: &[],
        answer: Answer::Text("&"),
        explanation: "传 `&name` 只是借用，函数读完后调用方仍然拥有 `name`。",
        hint: "函数参数类型已经写成 `&String`。",
    },
    Exercise {
        id: "ownership-function-takes",
        lesson_id: "ownership",
        title: "函数参数也会取得所有权",
        kind: ExerciseKind::SingleChoice,
        prompt: "调用 `takes(s)` 后，为什么不能再打印 `s`？",
        code: "fn takes(value: String) { println!(\"{value}\"); }\nlet s = String::from(\"hi\");\ntakes(s);\nprintln!(\"{s}\");",
        options: &["s 的所有权移动进函数参数 value", "println! 只能打印一次 String", "函数调用会自动 clone s"],
        answer: Answer::Choice(0),
        explanation: "按值传入 `String` 会 move 到函数参数里，调用后原绑定 `s` 失效。",
        hint: "函数参数也是一个新的 owner。",
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
        id: "slice-borrow-blocks-clear",
        lesson_id: "slices",
        title: "切片借用会限制修改",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `word` 还会被使用时，不能先 `clear` 原 String？",
        code: "let mut s = String::from(\"hello world\");\nlet word = &s[0..5];\ns.clear();\nprintln!(\"{word}\");",
        options: &["word 仍借用 s，clear 需要可变借用", "clear 会删除变量名", "切片会复制字符串所以冲突"],
        answer: Answer::Choice(0),
        explanation: "`word` 是对 `s` 的不可变借用；在它最后一次使用前，不能对 `s` 做可变借用。",
        hint: "切片不是复制，它仍然指向原字符串。",
    },
    Exercise {
        id: "slice-open-ended-range",
        lesson_id: "slices",
        title: "省略范围边界",
        kind: ExerciseKind::FillBlank,
        prompt: "从索引 6 一直切到字符串末尾，空白处应填什么？",
        code: "let s = String::from(\"hello world\");\nlet world = &s[____];",
        options: &[],
        answer: Answer::Text("6.."),
        explanation: "`6..` 省略右边界，表示从索引 6 到结尾。",
        hint: "Rust 范围可以省略起点或终点。",
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
        id: "borrow-immutable-many",
        lesson_id: "borrowing",
        title: "多个不可变引用可以共存",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面代码为什么可以通过编译？",
        code: "let s = String::from(\"hello\");\nlet r1 = &s;\nlet r2 = &s;\nprintln!(\"{r1} {r2}\");",
        options: &[
            "多个不可变引用只读，不会互相破坏数据",
            "String 被自动复制了两份",
            "println! 会释放所有引用",
        ],
        answer: Answer::Choice(0),
        explanation: "同时存在多个不可变引用是允许的，因为它们只能读取，不能修改被引用的数据。",
        hint: "借用规则允许“多个读”或“一个写”。",
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
        id: "struct-field-init-shorthand",
        lesson_id: "structs-enums",
        title: "字段初始化简写",
        kind: ExerciseKind::FillBlank,
        prompt: "变量名和字段名相同时，空白处可以怎样简写？",
        code: "let email = String::from(\"a@example.com\");\nlet username = String::from(\"ferris\");\nlet user = User {\n    ____,\n    username,\n    active: true,\n};",
        options: &[],
        answer: Answer::Text("email"),
        explanation: "当局部变量名和结构体字段名相同时，可以直接写字段名 `email`，等价于 `email: email`。",
        hint: "Rust Book 里这叫 field init shorthand。",
    },
    Exercise {
        id: "method-mut-self",
        lesson_id: "structs-enums",
        title: "修改结构体的方法要借用 mut self",
        kind: ExerciseKind::SingleChoice,
        prompt: "方法内部要修改字段时，self 参数通常应该写什么？",
        code: "impl Counter {\n    fn inc(____) {\n        self.value += 1;\n    }\n}",
        options: &["&mut self", "&self", "self: &Self"],
        answer: Answer::Choice(0),
        explanation: "修改字段需要可变借用接收者，因此方法签名应使用 `&mut self`。",
        hint: "只读用 `&self`，要改字段就需要可变借用。",
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
        id: "result-match-ok-err",
        lesson_id: "result-option",
        title: "Result 必须处理成功和失败",
        kind: ExerciseKind::FillBlank,
        prompt: "空白处分支应该填什么，才能在解析失败时返回 0？",
        code: "let parsed = \"42\".parse::<i32>();\nlet n = match parsed {\n    Ok(value) => value,\n    ____ => 0,\n};",
        options: &[],
        answer: Answer::Text("Err(_)"),
        explanation: "`Result` 有 `Ok` 和 `Err` 两类结果；`Err(_)` 匹配任意错误并忽略错误值。",
        hint: "下划线可以忽略你暂时不关心的错误对象。",
    },
    Exercise {
        id: "option-unwrap-risk",
        lesson_id: "result-option",
        title: "unwrap 的风险",
        kind: ExerciseKind::SingleChoice,
        prompt: "对 `None` 调用 `unwrap()` 会发生什么？",
        code: "let value: Option<i32> = None;\nlet n = value.unwrap();",
        options: &["运行时 panic", "返回 0", "编译器自动补一个默认值"],
        answer: Answer::Choice(0),
        explanation: "`unwrap()` 只适合你确信有值的场景；对 `None` 调用会直接 panic。",
        hint: "生产代码通常优先用 match、if let、unwrap_or 或 ?。",
    },
    Exercise {
        id: "question-mark-return-type",
        lesson_id: "result-option",
        title: "? 要求函数返回 Result",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么这个函数里不能直接使用 `?`？",
        code: "fn parse_num(raw: &str) -> i32 {\n    let n = raw.parse::<i32>()?;\n    n\n}",
        options: &[
            "函数返回 i32，不能承载提前返回的 Err",
            "parse 不能和 ? 一起使用",
            "? 只能出现在 main 函数里",
        ],
        answer: Answer::Choice(0),
        explanation: "`?` 在 `Err` 时会从当前函数提前返回错误，所以当前函数返回类型必须是 `Result` 等兼容类型。",
        hint: "把返回类型改成 `Result<i32, _>` 方向就对了。",
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
        id: "vec-index-bounds",
        lesson_id: "collections",
        title: "Vec get 比索引更安全",
        kind: ExerciseKind::SingleChoice,
        prompt: "访问可能越界的 Vec 元素时，哪种写法更安全？",
        code: "let values = vec![10, 20, 30];\nlet maybe = values.get(99);",
        options: &[
            "get 返回 Option，不会因为越界而 panic",
            "values[99] 会自动返回最后一个元素",
            "Vec 不允许按索引访问",
        ],
        answer: Answer::Choice(0),
        explanation: "`get` 返回 `Option<&T>`，越界时得到 `None`；直接索引越界会 panic。",
        hint: "当索引来自用户输入或外部数据时，优先考虑 `get`。",
    },
    Exercise {
        id: "hashmap-count-entry",
        lesson_id: "collections",
        title: "entry 统计词频",
        kind: ExerciseKind::FillBlank,
        prompt: "按照词频统计写法，空白处应填什么让计数加 1？",
        code: "let count = map.entry(word).or_insert(0);\n____ += 1;",
        options: &[],
        answer: Answer::Text("*count"),
        explanation: "`or_insert` 返回的是值的可变引用，想修改里面的整数需要先用 `*count` 解引用。",
        hint: "count 的类型是 `&mut i32`，不是 i32 本身。",
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
        id: "iterator-lazy",
        lesson_id: "iterators-traits",
        title: "迭代器适配器是惰性的",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么只调用 `map` 时，闭包里的打印不会执行？",
        code: "let iter = vec![1, 2, 3].iter().map(|n| {\n    println!(\"{n}\");\n    n * 2\n});",
        options: &[
            "map 是惰性适配器，必须被 collect、sum、for 等消费才会运行",
            "println! 不能写在 map 里",
            "iter 变量必须声明为 mut 才会执行",
        ],
        answer: Answer::Choice(0),
        explanation: "迭代器适配器本身只描述转换流程；只有消费适配器或 for 循环拉取元素时，链条才真正执行。",
        hint: "Rust Book 强调 iterators are lazy。",
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
    Exercise {
        id: "syntax-const-binding",
        lesson_id: "syntax-basics",
        title: "const 必须标注类型",
        kind: ExerciseKind::FillBlank,
        prompt: "定义编译期常量时，空白处应填什么类型标注？",
        code: "const MAX_POINTS: ____ = 100_000;",
        options: &[],
        answer: Answer::Text("u32"),
        explanation: "`const` 常量必须显式写出类型，不能像普通 `let` 那样省略类型标注。",
        hint: "Rust Book 的例子是 `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`。",
    },
    Exercise {
        id: "syntax-shadow-mutability",
        lesson_id: "syntax-basics",
        title: "shadowing 不要求 mut",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么这段代码不需要把 spaces 声明成 mut？",
        code: "let spaces = \"   \";\nlet spaces = spaces.len();",
        options: &["第二行创建了新的同名绑定", "字符串字面量默认可变", "usize 会自动修改原字符串"],
        answer: Answer::Choice(0),
        explanation: "shadowing 是重新 `let` 一个新绑定，不是修改原绑定，所以不需要 `mut`，并且可以改变类型。",
        hint: "看第二行是不是又写了一次 `let`。",
    },
    Exercise {
        id: "syntax-block-scope",
        lesson_id: "syntax-basics",
        title: "代码块有自己的作用域",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let x = 5;\n{\n    let x = x + 1;\n    println!(\"{x}\");\n}\nprintln!(\"{x}\");",
        options: &[],
        answer: Answer::Output("6\n5"),
        explanation: "内部代码块中的 `let x` 只在块内 shadow 外层变量；离开块后外层 `x` 仍然是 5。",
        hint: "分别判断内层作用域和外层作用域中的 x。",
    },
    Exercise {
        id: "match-guard",
        lesson_id: "control-flow",
        title: "match guard 增加条件",
        kind: ExerciseKind::FillBlank,
        prompt: "空白处填什么 guard，才能只匹配正数？",
        code: "match n {\n    value ____ => \"positive\",\n    _ => \"other\",\n}",
        options: &[],
        answer: Answer::Text("if value > 0"),
        explanation: "match arm 可以在模式后加 `if` guard，只有模式和条件都满足时才匹配该分支。",
        hint: "guard 写在模式后面，箭头 `=>` 前面。",
    },
    Exercise {
        id: "if-let-else",
        lesson_id: "control-flow",
        title: "if let 也能搭配 else",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let maybe: Option<i32> = None;\nif let Some(n) = maybe {\n    println!(\"{n}\");\n} else {\n    println!(\"missing\");\n}",
        options: &[],
        answer: Answer::Output("missing"),
        explanation: "`maybe` 是 `None`，不满足 `Some(n)` 模式，因此进入 `else` 分支。",
        hint: "if let 只在模式匹配时进入第一个分支。",
    },
    Exercise {
        id: "while-let-pop",
        lesson_id: "control-flow",
        title: "while let 循环解包",
        kind: ExerciseKind::SingleChoice,
        prompt: "`while let Some(top) = stack.pop()` 会在什么时候停止？",
        code: "let mut stack = vec![1, 2, 3];\nwhile let Some(top) = stack.pop() {\n    println!(\"{top}\");\n}",
        options: &["pop 返回 None 时", "top 等于 0 时", "循环体第一次执行后"],
        answer: Answer::Choice(0),
        explanation: "`while let` 会在模式持续匹配时循环；当 `pop()` 返回 `None`，模式不再匹配，循环结束。",
        hint: "Vec 为空时 pop 的返回值是什么？",
    },
    Exercise {
        id: "tuple-index",
        lesson_id: "data-functions",
        title: "用点号访问元组元素",
        kind: ExerciseKind::FillBlank,
        prompt: "访问元组第二个元素，空白处应填什么？",
        code: "let tup = (500, 6.4, 1);\nlet y = tup.____;",
        options: &[],
        answer: Answer::Text("1"),
        explanation: "元组索引从 0 开始，使用点号加数字访问位置元素，因此第二个元素是 `tup.1`。",
        hint: "不是数组的 `[1]`，元组使用 `.1`。",
    },
    Exercise {
        id: "array-repeat-init",
        lesson_id: "data-functions",
        title: "数组重复初始化语法",
        kind: ExerciseKind::FillBlank,
        prompt: "创建 5 个值都为 3 的数组，空白处应填什么？",
        code: "let a = [____];",
        options: &[],
        answer: Answer::Text("3; 5"),
        explanation: "`[value; count]` 会创建包含 `count` 个相同值的数组，`[3; 5]` 等价于 `[3, 3, 3, 3, 3]`。",
        hint: "分号左边是值，右边是长度。",
    },
    Exercise {
        id: "function-explicit-return",
        lesson_id: "data-functions",
        title: "return 会提前返回",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段函数调用会输出什么？",
        code: "fn pick() -> i32 {\n    return 7;\n    9\n}\nprintln!(\"{}\", pick());",
        options: &[],
        answer: Answer::Output("7"),
        explanation: "`return 7;` 会立即从函数返回，后面的尾表达式不会再决定返回值。",
        hint: "Rust 更常用尾表达式，但 `return` 仍然会提前退出。",
    },
    Exercise {
        id: "ownership-return-ownership",
        lesson_id: "ownership",
        title: "返回值可以交还所有权",
        kind: ExerciseKind::FillBlank,
        prompt: "函数拿走 String 后想交还给调用方，空白处应填什么？",
        code: "fn take_and_give_back(s: String) -> String {\n    ____\n}",
        options: &[],
        answer: Answer::Text("s"),
        explanation: "参数 `s` 进入函数后由函数拥有；把 `s` 作为返回值返回，会把所有权再移动给调用方。",
        hint: "尾表达式可以直接移动返回局部变量。",
    },
    Exercise {
        id: "ownership-borrow-vs-clone",
        lesson_id: "ownership",
        title: "能借用就不要 clone",
        kind: ExerciseKind::SingleChoice,
        prompt: "只需要读取字符串长度时，哪种调用通常更合适？",
        code: "fn len(s: &String) -> usize { s.len() }\nlet name = String::from(\"Ferris\");",
        options: &["len(&name)", "len(&name.clone())", "len(name)"],
        answer: Answer::Choice(0),
        explanation: "只读场景传引用即可，既不移动所有权，也不产生不必要的堆数据 clone。",
        hint: "clone 有成本，借用更轻量。",
    },
    Exercise {
        id: "ownership-copy-types",
        lesson_id: "ownership",
        title: "哪些类型通常是 Copy",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪组值赋值后原变量通常仍可使用？",
        code: "let a = true;\nlet b = 'R';\nlet c = 42;",
        options: &["bool、char 和整数", "String 和 Vec", "所有权类型都一样"],
        answer: Answer::Choice(0),
        explanation: "布尔、字符、整数等简单标量通常实现 `Copy`；`String`、`Vec` 这类拥有堆资源的类型通常会 move。",
        hint: "问自己这个类型是否管理堆资源。",
    },
    Exercise {
        id: "slice-full-range",
        lesson_id: "slices",
        title: "完整范围切片",
        kind: ExerciseKind::FillBlank,
        prompt: "借用整个字符串切片，空白处应填什么范围？",
        code: "let s = String::from(\"hello\");\nlet all = &s[____];",
        options: &[],
        answer: Answer::Text(".."),
        explanation: "`..` 表示从开始到结束的完整范围，`&s[..]` 借用整个字符串。",
        hint: "同时省略左右边界。",
    },
    Exercise {
        id: "slice-string-param",
        lesson_id: "slices",
        title: "参数优先接收 &str",
        kind: ExerciseKind::SingleChoice,
        prompt: "只读字符串内容的函数，参数写成哪种通常更灵活？",
        code: "fn first_word(s: ____) -> &str { /* ... */ }",
        options: &["&str", "&String", "String"],
        answer: Answer::Choice(0),
        explanation: "`&str` 可以接收字符串字面量，也可以接收 `String` 的切片，比 `&String` 更通用。",
        hint: "Rust Book 后续会把参数从 `&String` 改进为 `&str`。",
    },
    Exercise {
        id: "slice-utf8-boundary",
        lesson_id: "slices",
        title: "字符串切片必须在 UTF-8 边界",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么随意按字节范围切中文字符串可能 panic？",
        code: "let s = String::from(\"你好\");\nlet part = &s[0..1];",
        options: &["范围没有落在 UTF-8 字符边界上", "String 不能切片", "中文字符串不能存入 String"],
        answer: Answer::Choice(0),
        explanation: "Rust 字符串是 UTF-8，字符串切片范围必须位于合法字符边界；切到字符中间会 panic。",
        hint: "字符串索引是字节位置，不是第几个字符。",
    },
    Exercise {
        id: "borrow-mut-exclusive-error",
        lesson_id: "borrowing",
        title: "可变借用必须独占",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面代码为什么不能通过编译？",
        code: "let mut s = String::from(\"hi\");\nlet r1 = &mut s;\nlet r2 = &mut s;\nprintln!(\"{r1} {r2}\");",
        options: &["同一时间不能有两个活跃的可变引用", "String 不能可变借用", "println! 不能打印引用"],
        answer: Answer::Choice(0),
        explanation: "可变引用提供独占写权限，同时存在两个活跃 `&mut` 会破坏 Rust 的别名规则。",
        hint: "一个写，或多个读；不能多个写。",
    },
    Exercise {
        id: "borrow-reborrow-shared",
        lesson_id: "borrowing",
        title: "可变引用可以短暂重借用为只读",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let mut s = String::from(\"hi\");\nlet r = &mut s;\nlet len = r.len();\nr.push('!');\nprintln!(\"{len} {r}\");",
        options: &[],
        answer: Answer::Output("2 hi!"),
        explanation: "通过可变引用可以先只读调用 `len`，随后继续用同一个可变引用修改字符串。",
        hint: "这里没有同时创建另一个会活到 println 的引用。",
    },
    Exercise {
        id: "borrow-function-mut-param",
        lesson_id: "borrowing",
        title: "函数修改调用方的值",
        kind: ExerciseKind::FillBlank,
        prompt: "函数要修改调用方传入的 String，参数类型空白处应填什么？",
        code: "fn add_world(s: ____) {\n    s.push_str(\" world\");\n}",
        options: &[],
        answer: Answer::Text("&mut String"),
        explanation: "函数若要修改调用方拥有的 `String`，需要接收 `&mut String` 可变引用。",
        hint: "只读是 `&String`，修改需要加 mut。",
    },
    Exercise {
        id: "tuple-struct-access",
        lesson_id: "structs-enums",
        title: "访问元组结构体字段",
        kind: ExerciseKind::FillBlank,
        prompt: "访问 Color 的第一个字段，空白处应填什么？",
        code: "struct Color(i32, i32, i32);\nlet black = Color(0, 0, 0);\nlet red = black.____;",
        options: &[],
        answer: Answer::Text("0"),
        explanation: "元组结构体字段没有名字，和元组一样用 `.0`、`.1`、`.2` 访问。",
        hint: "字段位置从 0 开始。",
    },
    Exercise {
        id: "enum-variant-data",
        lesson_id: "structs-enums",
        title: "枚举变体可以携带数据",
        kind: ExerciseKind::SingleChoice,
        prompt: "`IpAddr::V4(127, 0, 0, 1)` 说明了什么？",
        code: "enum IpAddr {\n    V4(u8, u8, u8, u8),\n    V6(String),\n}",
        options: &["枚举变体可以携带不同类型和数量的数据", "枚举只能保存数字", "V4 和 V6 必须字段完全相同"],
        answer: Answer::Choice(0),
        explanation: "Rust 枚举的每个变体都可以定义自己携带的数据形状，这是它比很多语言枚举更强的地方。",
        hint: "Rust Book 的 IP 地址例子正是为了展示这一点。",
    },
    Exercise {
        id: "match-enum-method",
        lesson_id: "structs-enums",
        title: "方法里 match self",
        kind: ExerciseKind::FillBlank,
        prompt: "方法中只读取枚举自身并匹配变体，空白处通常填什么？",
        code: "impl Message {\n    fn call(____) {\n        match self { /* ... */ }\n    }\n}",
        options: &[],
        answer: Answer::Text("&self"),
        explanation: "方法只读取枚举值时接收 `&self` 即可，然后可以在方法体中 `match self` 判断具体变体。",
        hint: "不修改、不消费，就优先考虑借用 self。",
    },
    Exercise {
        id: "result-unwrap-or",
        lesson_id: "result-option",
        title: "unwrap_or 提供默认值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let n = \"abc\".parse::<i32>().unwrap_or(0);\nprintln!(\"{n}\");",
        options: &[],
        answer: Answer::Output("0"),
        explanation: "解析失败得到 `Err`，`unwrap_or(0)` 会使用给定默认值 0，而不是 panic。",
        hint: "和 unwrap 不同，unwrap_or 有兜底值。",
    },
    Exercise {
        id: "option-map",
        lesson_id: "result-option",
        title: "Option::map 转换内部值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let maybe = Some(2);\nlet doubled = maybe.map(|n| n * 2);\nprintln!(\"{:?}\", doubled);",
        options: &[],
        answer: Answer::Output("Some(4)"),
        explanation: "`Option::map` 只在 `Some` 时转换内部值，并把结果重新包回 `Some`。",
        hint: "如果 maybe 是 None，map 的结果仍会是 None。",
    },
    Exercise {
        id: "question-mark-option",
        lesson_id: "result-option",
        title: "? 也能传播 None",
        kind: ExerciseKind::SingleChoice,
        prompt: "在返回 Option 的函数里，对 Option 使用 `?` 遇到 None 会怎样？",
        code: "fn first(values: Vec<i32>) -> Option<i32> {\n    let n = values.get(0)?;\n    Some(*n)\n}",
        options: &["从当前函数提前返回 None", "把 None 转成 0", "触发 panic"],
        answer: Answer::Choice(0),
        explanation: "`?` 不只用于 Result；在返回 Option 的函数中，遇到 `None` 会提前返回 `None`。",
        hint: "关键仍然是当前函数返回类型要兼容。",
    },
    Exercise {
        id: "vec-iterate-borrow",
        lesson_id: "collections",
        title: "遍历 Vec 时只借用",
        kind: ExerciseKind::FillBlank,
        prompt: "遍历后还想继续使用 values，for 循环空白处应填什么？",
        code: "let values = vec![1, 2, 3];\nfor n in ____values {\n    println!(\"{n}\");\n}\nprintln!(\"{}\", values.len());",
        options: &[],
        answer: Answer::Text("&"),
        explanation: "`for n in &values` 只迭代不可变引用，不会消费 Vec，因此循环后还能继续使用 `values`。",
        hint: "不想 move 集合，就遍历它的引用。",
    },
    Exercise {
        id: "string-push-char",
        lesson_id: "collections",
        title: "push 追加单个字符",
        kind: ExerciseKind::FillBlank,
        prompt: "向 String 追加单个感叹号字符，空白处应填什么方法？",
        code: "let mut s = String::from(\"hi\");\ns.____('!');",
        options: &[],
        answer: Answer::Text("push"),
        explanation: "`String::push` 追加单个 `char`，而 `push_str` 追加字符串切片 `&str`。",
        hint: "单引号里的 `'!'` 是 char。",
    },
    Exercise {
        id: "hashmap-insert-overwrite",
        lesson_id: "collections",
        title: "insert 会覆盖旧值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码最后会打印什么？",
        code: "let mut scores = std::collections::HashMap::new();\nscores.insert(\"Blue\", 10);\nscores.insert(\"Blue\", 25);\nprintln!(\"{}\", scores[\"Blue\"]);",
        options: &[],
        answer: Answer::Output("25"),
        explanation: "对已存在的 key 再次 `insert` 会覆盖旧值；如果只想缺失时插入，应使用 `entry(...).or_insert(...)`。",
        hint: "区分 insert 覆盖和 entry 只在缺失时插入。",
    },
    Exercise {
        id: "iterator-filter-borrow",
        lesson_id: "iterators-traits",
        title: "filter 闭包收到引用",
        kind: ExerciseKind::FillBlank,
        prompt: "对 `iter()` 的结果 filter，空白处常见写法是什么？",
        code: "let evens: Vec<_> = [1, 2, 3, 4].iter().filter(|n| ____ % 2 == 0).collect();",
        options: &[],
        answer: Answer::Text("**n"),
        explanation: "`iter()` 产生引用，`filter` 闭包参数又接收对 item 的引用，所以这里的 `n` 形如 `&&i32`，需要解引用到整数比较。",
        hint: "如果觉得难读，也可以用模式 `|&&n| n % 2 == 0`。",
    },
    Exercise {
        id: "iterator-sum-consumes",
        lesson_id: "iterators-traits",
        title: "sum 会消费迭代器",
        kind: ExerciseKind::SingleChoice,
        prompt: "调用 `iter.sum()` 后，为什么不能再使用 iter？",
        code: "let iter = vec![1, 2, 3].into_iter();\nlet total: i32 = iter.sum();",
        options: &["sum 获取迭代器所有权并消费它", "sum 会清空原来的 Vec", "i32 不能求和"],
        answer: Answer::Choice(0),
        explanation: "`sum` 是消费适配器，调用时会拿走迭代器并不断取值，因此原迭代器不能再次使用。",
        hint: "消费适配器通常按值接收 self。",
    },
    Exercise {
        id: "iterator-enumerate",
        lesson_id: "iterators-traits",
        title: "enumerate 产生索引和值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "for (index, value) in [\"a\", \"b\"].iter().enumerate() {\n    println!(\"{index}:{value}\");\n}",
        options: &[],
        answer: Answer::Output("0:a\n1:b"),
        explanation: "`enumerate` 把迭代器元素变成 `(index, value)`，索引从 0 开始。",
        hint: "先产生第 0 个元素，再产生第 1 个元素。",
    },
    Exercise {
        id: "impl-trait-param",
        lesson_id: "generics-traits",
        title: "impl Trait 简化参数约束",
        kind: ExerciseKind::FillBlank,
        prompt: "函数参数想接收任何可 Display 的值，空白处应填什么？",
        code: "fn notify(item: impl ____) {\n    println!(\"{}\", item);\n}",
        options: &[],
        answer: Answer::Text("Display"),
        explanation: "`impl Display` 是常见的参数写法，表示该参数可以是任意实现了 `Display` 的类型。",
        hint: "这和 `T: Display` 表达的是相同能力。",
    },
    Exercise {
        id: "derive-debug-bound",
        lesson_id: "generics-traits",
        title: "调试打印需要 Debug",
        kind: ExerciseKind::SingleChoice,
        prompt: "泛型函数中使用 `{:?}` 打印 T，T 需要什么 trait bound？",
        code: "fn dump<T: ____>(value: T) {\n    println!(\"{:?}\", value);\n}",
        options: &["Debug", "Display", "Clone"],
        answer: Answer::Choice(0),
        explanation: "`{:?}` 使用调试格式化，需要 `std::fmt::Debug`；`{}` 才需要 `Display`。",
        hint: "Debug 对应问号格式化。",
    },
    Exercise {
        id: "lifetime-elision",
        lesson_id: "generics-traits",
        title: "生命周期省略规则",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `fn first_word(s: &str) -> &str` 通常不需要手写生命周期？",
        code: "fn first_word(s: &str) -> &str { s }",
        options: &["只有一个输入引用，编译器能推断输出来自它", "生命周期不存在了", "&str 永远是 static"],
        answer: Answer::Choice(0),
        explanation: "生命周期省略规则允许在简单模式下省略标注；这里唯一输入引用的生命周期会被赋给输出引用。",
        hint: "省略不是取消约束，而是编译器按规则补上。",
    },
    Exercise {
        id: "thread-join",
        lesson_id: "concurrency",
        title: "join 等待线程结束",
        kind: ExerciseKind::SingleChoice,
        prompt: "调用 `handle.join().unwrap()` 的主要作用是什么？",
        code: "let handle = std::thread::spawn(|| println!(\"hi\"));\nhandle.join().unwrap();",
        options: &["等待子线程执行完成", "强制杀死子线程", "把线程变成异步任务"],
        answer: Answer::Choice(0),
        explanation: "`join` 会阻塞当前线程，直到被 join 的线程执行结束，并返回线程闭包的结果。",
        hint: "不 join 时，主线程可能先结束。",
    },
    Exercise {
        id: "channel-recv-block",
        lesson_id: "concurrency",
        title: "recv 会阻塞等待消息",
        kind: ExerciseKind::SingleChoice,
        prompt: "`rx.recv()` 在还没有消息但发送端仍存在时会怎样？",
        code: "let received = rx.recv().unwrap();",
        options: &["阻塞等待消息", "立即返回空字符串", "自动创建一条默认消息"],
        answer: Answer::Choice(0),
        explanation: "`recv` 会阻塞当前线程直到收到消息；如果所有发送端都断开，则会返回错误。",
        hint: "还有一个非阻塞版本叫 try_recv。",
    },
    Exercise {
        id: "mutex-poison-unwrap",
        lesson_id: "concurrency",
        title: "lock 返回 Result",
        kind: ExerciseKind::FillBlank,
        prompt: "Mutex::lock 返回 Result，示例中通常用什么方法取出 guard？",
        code: "let mut num = counter.lock().____();\n*num += 1;",
        options: &[],
        answer: Answer::Text("unwrap"),
        explanation: "`lock()` 返回 `LockResult<MutexGuard<_>>`，因为持锁线程 panic 可能导致 mutex poisoned；教学示例常用 `unwrap()` 简化处理。",
        hint: "Rust Book 的并发计数器示例就是 `lock().unwrap()`。",
    },    Exercise {
        id: "syntax-type-inference",
        lesson_id: "syntax-basics",
        title: "类型推断不等于动态类型",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `let x = 5;` 之后不能把字符串赋给 x？",
        code: "let mut x = 5;\nx = \"five\";",
        options: &["x 的类型已被推断为整数，不能再变成 &str", "mut 变量不能重新赋值", "字符串不能赋给任何变量"],
        answer: Answer::Choice(0),
        explanation: "Rust 会在编译期为变量确定一个具体类型；`mut` 只允许修改值，不允许改变绑定类型。",
        hint: "mut 不是动态类型开关。",
    },
    Exercise {
        id: "syntax-mut-reassign",
        lesson_id: "syntax-basics",
        title: "mut 允许重新赋值同类型值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let mut count = 1;\ncount = count + 2;\nprintln!(\"{count}\");",
        options: &[],
        answer: Answer::Output("3"),
        explanation: "`count` 被声明为 `mut`，因此可以用同类型的新整数值重新赋值。",
        hint: "确认这里没有改变类型，只是改了值。",
    },
    Exercise {
        id: "syntax-const-uppercase",
        lesson_id: "syntax-basics",
        title: "常量命名习惯",
        kind: ExerciseKind::SingleChoice,
        prompt: "Rust 常量通常采用哪种命名风格？",
        code: "const MAX_RETRIES: u8 = 3;",
        options: &["SCREAMING_SNAKE_CASE", "camelCase", "kebab-case"],
        answer: Answer::Choice(0),
        explanation: "Rust 社区通常用全大写蛇形命名常量，例如 `MAX_RETRIES`。",
        hint: "看 Rust Book 里的常量例子命名。",
    },
    Exercise {
        id: "syntax-expression-parentheses",
        lesson_id: "syntax-basics",
        title: "括号表达式的值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let value = (1 + 2) * 3;\nprintln!(\"{value}\");",
        options: &[],
        answer: Answer::Output("9"),
        explanation: "括号先计算 `1 + 2` 得到 3，再乘以 3，输出 9。",
        hint: "这题练的是普通表达式求值顺序。",
    },
    Exercise {
        id: "match-literal-pattern",
        lesson_id: "control-flow",
        title: "字面量模式",
        kind: ExerciseKind::SingleChoice,
        prompt: "哪个分支会匹配 `x = 2`？",
        code: "let x = 2;\nmatch x {\n    1 => \"one\",\n    2 => \"two\",\n    _ => \"other\",\n};",
        options: &["`2 => \"two\"`", "`1 => \"one\"`", "`_ => \"other\"`"],
        answer: Answer::Choice(0),
        explanation: "match 从上到下尝试模式，字面量 `2` 与值 2 匹配。",
        hint: "`_` 只会匹配前面没匹配到的剩余情况。",
    },
    Exercise {
        id: "match-binding-value",
        lesson_id: "control-flow",
        title: "模式绑定值",
        kind: ExerciseKind::FillBlank,
        prompt: "把匹配到的数字绑定出来，空白处应填什么？",
        code: "let msg = match x {\n    ____ => format!(\"number {n}\"),\n};",
        options: &[],
        answer: Answer::Text("n"),
        explanation: "单个标识符模式会匹配任意值并把它绑定到该名字；这里绑定为 `n`。",
        hint: "这是一个会匹配所有值的绑定模式。",
    },
    Exercise {
        id: "if-else-no-semicolon",
        lesson_id: "control-flow",
        title: "if 表达式赋值不要多余分号",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么这个写法能把结果赋给 label？",
        code: "let label = if ready { \"go\" } else { \"wait\" };",
        options: &["if/else 两个分支都是尾表达式并产生 &str", "if 只能返回 bool", "分号让 label 变成字符串"],
        answer: Answer::Choice(0),
        explanation: "`if` 在 Rust 中可以是表达式；两个分支的尾表达式都是 `&str`，所以整体产生一个 `&str`。",
        hint: "关注每个分支最后有没有分号。",
    },
    Exercise {
        id: "loop-break-value",
        lesson_id: "control-flow",
        title: "loop break 可以带值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let mut n = 0;\nlet result = loop {\n    n += 1;\n    if n == 3 { break n * 10; }\n};\nprintln!(\"{result}\");",
        options: &[],
        answer: Answer::Output("30"),
        explanation: "`loop` 可以通过 `break value` 产生整个循环表达式的值；n 等于 3 时返回 30。",
        hint: "Rust 的 loop 也可以是表达式。",
    },
    Exercise {
        id: "integer-overflow-debug",
        lesson_id: "data-functions",
        title: "整数溢出的调试行为",
        kind: ExerciseKind::SingleChoice,
        prompt: "调试构建中 `u8` 超过 255 通常会怎样？",
        code: "let x: u8 = 255;\nlet y = x + 1;",
        options: &["panic", "自动变成 u16", "得到字符串 \"256\""],
        answer: Answer::Choice(0),
        explanation: "Rust 在 debug 构建中会检查整数溢出并 panic；release 构建会按补码回绕，除非使用显式方法。",
        hint: "Rust Book 在整数类型章节提到过溢出行为。",
    },
    Exercise {
        id: "float-default-f64",
        lesson_id: "data-functions",
        title: "浮点默认类型",
        kind: ExerciseKind::SingleChoice,
        prompt: "`let x = 2.0;` 默认推断成什么浮点类型？",
        code: "let x = 2.0;",
        options: &["f64", "f32", "i32"],
        answer: Answer::Choice(0),
        explanation: "Rust 的浮点默认类型是 `f64`，因为现代 CPU 上通常速度接近且精度更高。",
        hint: "Rust Book：floating-point types。",
    },
    Exercise {
        id: "char-four-bytes",
        lesson_id: "data-functions",
        title: "char 是 Unicode 标量值",
        kind: ExerciseKind::SingleChoice,
        prompt: "Rust 的 `char` 最准确表示什么？",
        code: "let heart = '❤';",
        options: &["Unicode 标量值，大小为 4 字节", "ASCII 字节", "字符串切片"],
        answer: Answer::Choice(0),
        explanation: "Rust `char` 表示 Unicode 标量值，占 4 字节，不等同于一个 UTF-8 字节。",
        hint: "单引号是 char，双引号是字符串切片。",
    },
    Exercise {
        id: "function-implicit-unit",
        lesson_id: "data-functions",
        title: "没有返回类型默认返回 unit",
        kind: ExerciseKind::SingleChoice,
        prompt: "函数不写返回类型且没有尾表达式时返回什么？",
        code: "fn log() {\n    println!(\"hi\");\n}",
        options: &["()", "String", "i32"],
        answer: Answer::Choice(0),
        explanation: "没有显式返回类型的函数默认返回 unit 类型 `()`。",
        hint: "println! 本身也是为了副作用。",
    },
    Exercise {
        id: "ownership-drop-at-scope-end",
        lesson_id: "ownership",
        title: "作用域结束时 drop",
        kind: ExerciseKind::SingleChoice,
        prompt: "`s` 持有的内存什么时候释放？",
        code: "{\n    let s = String::from(\"hi\");\n}",
        options: &["离开作用域时自动 drop", "创建后立即释放", "必须手动 free"],
        answer: Answer::Choice(0),
        explanation: "拥有资源的值在离开作用域时自动调用 `drop` 释放资源。",
        hint: "这是 Rust 无 GC 仍能自动管理内存的关键。",
    },
    Exercise {
        id: "ownership-reassign-drops-old",
        lesson_id: "ownership",
        title: "重新赋值会释放旧值",
        kind: ExerciseKind::SingleChoice,
        prompt: "给已有 String 变量重新赋值时，旧字符串会怎样？",
        code: "let mut s = String::from(\"old\");\ns = String::from(\"new\");",
        options: &["旧值被 drop，新值归 s 所有", "old 和 new 同时归 s 所有", "旧值泄漏到堆上"],
        answer: Answer::Choice(0),
        explanation: "重新赋值会先结束旧值的生命周期并释放其资源，然后让变量拥有新值。",
        hint: "所有权系统也管理重新赋值场景。",
    },
    Exercise {
        id: "ownership-tuple-move-field",
        lesson_id: "ownership",
        title: "元组中的 String 会 move",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面哪项说法正确？",
        code: "let pair = (String::from(\"a\"), 1);\nlet name = pair.0;",
        options: &["pair.0 的 String 被移动出来，pair 不再完整可用", "pair 被完整复制", "i32 也一定被移动导致不可用"],
        answer: Answer::Choice(0),
        explanation: "从元组中移动非 Copy 字段会让该字段失效，整个元组不再能作为完整值使用。",
        hint: "非 Copy 字段移动会影响包含它的复合值。",
    },
    Exercise {
        id: "ownership-reference-no-drop",
        lesson_id: "ownership",
        title: "引用不拥有值",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `r` 离开作用域不会释放 String？",
        code: "let s = String::from(\"hi\");\n{ let r = &s; }\nprintln!(\"{s}\");",
        options: &["r 只是引用，不拥有 String", "引用会 clone String", "println! 会重新创建 s"],
        answer: Answer::Choice(0),
        explanation: "引用只是借用视图，不拥有被引用的数据，因此引用结束不会 drop 原值。",
        hint: "谁拥有 String，谁负责释放它。",
    },
    Exercise {
        id: "slice-array-slice",
        lesson_id: "slices",
        title: "数组切片类型",
        kind: ExerciseKind::SingleChoice,
        prompt: "`&numbers[1..3]` 的类型最准确是什么？",
        code: "let numbers = [10, 20, 30, 40];\nlet part = &numbers[1..3];",
        options: &["&[i32]", "Vec<i32>", "[i32; 2]"],
        answer: Answer::Choice(0),
        explanation: "数组切片是对数组连续片段的借用，类型是 `&[T]`。",
        hint: "切片没有固定长度写在类型里。",
    },
    Exercise {
        id: "slice-len-method",
        lesson_id: "slices",
        title: "切片也有 len",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let nums = [1, 2, 3, 4];\nlet part = &nums[1..];\nprintln!(\"{}\", part.len());",
        options: &[],
        answer: Answer::Output("3"),
        explanation: "`&nums[1..]` 包含索引 1、2、3 三个元素，因此长度为 3。",
        hint: "右边界省略表示一直到结尾。",
    },
    Exercise {
        id: "slice-mut-slice",
        lesson_id: "slices",
        title: "可变切片",
        kind: ExerciseKind::FillBlank,
        prompt: "想通过切片修改数组片段，空白处应填什么？",
        code: "let mut nums = [1, 2, 3];\nlet part = ____ nums[0..2];\npart[0] = 9;",
        options: &[],
        answer: Answer::Text("&mut"),
        explanation: "可变切片写作 `&mut nums[0..2]`，允许通过切片修改底层数组元素。",
        hint: "数组本身也必须是 mut。",
    },
    Exercise {
        id: "slice-first-word-signature",
        lesson_id: "slices",
        title: "first_word 更通用的签名",
        kind: ExerciseKind::FillBlank,
        prompt: "让 first_word 同时接受 String 切片和字符串字面量，参数类型应写什么？",
        code: "fn first_word(s: ____) -> &str {\n    s\n}",
        options: &[],
        answer: Answer::Text("&str"),
        explanation: "参数写 `&str` 比 `&String` 更通用，调用方可以传 `&my_string[..]` 或字符串字面量。",
        hint: "API 参数尽量接收更抽象的切片。",
    },
    Exercise {
        id: "borrow-last-use-nll",
        lesson_id: "borrowing",
        title: "最后一次使用后借用结束",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么这段代码能编译？",
        code: "let mut s = String::from(\"hi\");\nlet r = &s;\nprintln!(\"{r}\");\ns.push_str(\"!\");",
        options: &["r 最后一次使用后不可变借用结束", "push_str 不需要可变借用", "println! 会 clone s"],
        answer: Answer::Choice(0),
        explanation: "非词法生命周期会在引用最后一次使用后结束借用，因此之后可以可变借用 `s`。",
        hint: "看 r 最后在哪里被使用。",
    },
    Exercise {
        id: "borrow-mut-then-read-owner",
        lesson_id: "borrowing",
        title: "可变借用结束后再读 owner",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let mut s = String::from(\"a\");\nlet r = &mut s;\nr.push('b');\nprintln!(\"{s}\");",
        options: &[],
        answer: Answer::Output("ab"),
        explanation: "可变引用 `r` 最后一次使用在 `push`，之后借用结束，`s` 可以再次被读取。",
        hint: "NLL 会缩短借用到最后一次使用。",
    },
    Exercise {
        id: "borrow-cannot-move-while-borrowed",
        lesson_id: "borrowing",
        title: "被借用时不能 move",
        kind: ExerciseKind::SingleChoice,
        prompt: "下面代码为什么不能编译？",
        code: "let s = String::from(\"hi\");\nlet r = &s;\nlet moved = s;\nprintln!(\"{r}\");",
        options: &["s 仍被 r 借用，不能移动所有权", "String 永远不能 move", "引用必须是 mut"],
        answer: Answer::Choice(0),
        explanation: "只要 `r` 之后还会使用，`s` 的不可变借用就仍活跃，此时不能把 `s` move 走。",
        hint: "引用不能指向已经被移动的值。",
    },
    Exercise {
        id: "borrow-mut-argument-call",
        lesson_id: "borrowing",
        title: "调用可变引用参数",
        kind: ExerciseKind::FillBlank,
        prompt: "调用需要 `&mut String` 的函数，空白处应填什么？",
        code: "fn change(s: &mut String) { s.push_str(\"!\"); }\nlet mut text = String::from(\"hi\");\nchange(____text);",
        options: &[],
        answer: Answer::Text("&mut"),
        explanation: "实参处也要显式写 `&mut text`，表示把可变引用借给函数。",
        hint: "函数签名已经提示参数类型。",
    },
    Exercise {
        id: "struct-associated-function",
        lesson_id: "structs-enums",
        title: "关联函数不用 self",
        kind: ExerciseKind::SingleChoice,
        prompt: "`String::from(\"hi\")` 这类函数为什么用 `::` 调用？",
        code: "String::from(\"hi\");",
        options: &["它是关联函数，不接收 self", "它会修改已有 String", "它只能在 match 里使用"],
        answer: Answer::Choice(0),
        explanation: "不接收 `self` 的关联函数属于类型本身，调用时使用 `Type::function(...)`。",
        hint: "构造器常写成关联函数。",
    },
    Exercise {
        id: "struct-debug-print",
        lesson_id: "structs-enums",
        title: "结构体调试打印",
        kind: ExerciseKind::SingleChoice,
        prompt: "想用 `{:?}` 打印自定义结构体，通常需要做什么？",
        code: "#[derive(Debug)]\nstruct User { name: String }",
        options: &["派生或实现 Debug", "派生 Display", "把结构体改成元组"],
        answer: Answer::Choice(0),
        explanation: "`{:?}` 使用 `Debug`，自定义类型通常通过 `#[derive(Debug)]` 快速获得调试打印能力。",
        hint: "问号格式化对应 Debug。",
    },
    Exercise {
        id: "enum-if-let-method",
        lesson_id: "structs-enums",
        title: "if let 匹配枚举变体",
        kind: ExerciseKind::FillBlank,
        prompt: "已知 `msg` 的类型是 `Message`，只关心 Quit 变体时，空白处应填什么？",
        code: "enum Message { Quit, Move { x: i32, y: i32 } }\nlet msg = Message::Quit;\n\nif let ____ = msg {\n    println!(\"quit\");\n}",
        options: &[],
        answer: Answer::Text("Message::Quit"),
        explanation: "题干先定义了 `enum Message { Quit, ... }`，所以枚举变体需要写成 `Message::Quit`；`if let Message::Quit = msg` 只在 msg 是 Quit 变体时执行分支。",
        hint: "先看 `msg` 的类型定义：变体 Quit 属于 Message 这个枚举。",
    },
    Exercise {
        id: "method-takes-self",
        lesson_id: "structs-enums",
        title: "消费 self 的方法",
        kind: ExerciseKind::SingleChoice,
        prompt: "方法签名 `fn into_inner(self)` 表示什么？",
        code: "impl Wrapper {\n    fn into_inner(self) -> String { self.value }\n}",
        options: &["调用方法会消费 Wrapper", "方法只读 Wrapper", "方法借用并修改 Wrapper"],
        answer: Answer::Choice(0),
        explanation: "接收 `self` 表示方法取得调用者所有权，通常用于把内部值拆出来或转换成别的类型。",
        hint: "into_ 前缀经常暗示所有权转换。",
    },
    Exercise {
        id: "result-map-err",
        lesson_id: "result-option",
        title: "map_err 转换错误",
        kind: ExerciseKind::SingleChoice,
        prompt: "`map_err` 的作用是什么？",
        code: "let parsed = raw.parse::<i32>().map_err(|e| e.to_string());",
        options: &["只转换 Err 中的错误值", "只转换 Ok 中的成功值", "忽略所有错误"],
        answer: Answer::Choice(0),
        explanation: "`map_err` 只在 `Err` 时转换错误值，`Ok` 会原样保留。",
        hint: "和 Option::map 对 Some 类似，但作用在 Err 上。",
    },
    Exercise {
        id: "result-ok-method",
        lesson_id: "result-option",
        title: "Result::ok 转成 Option",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let value = \"42\".parse::<i32>().ok();\nprintln!(\"{:?}\", value);",
        options: &[],
        answer: Answer::Output("Some(42)"),
        explanation: "`ok()` 会把 `Result<T, E>` 转成 `Option<T>`：Ok 变 Some，Err 变 None。",
        hint: "方法名 ok 表示只保留成功值。",
    },
    Exercise {
        id: "option-and-then",
        lesson_id: "result-option",
        title: "and_then 链式处理 Option",
        kind: ExerciseKind::SingleChoice,
        prompt: "`and_then` 和 `map` 的关键区别是什么？",
        code: "maybe.and_then(|s| s.parse::<i32>().ok())",
        options: &["闭包本身返回 Option，可避免嵌套 Option", "and_then 会 panic", "and_then 只能用于 Result"],
        answer: Answer::Choice(0),
        explanation: "`and_then` 的闭包返回 `Option<U>`，适合链式可能失败的步骤，并自动扁平化结果。",
        hint: "其他语言里常叫 flatMap。",
    },
    Exercise {
        id: "result-expect-message",
        lesson_id: "result-option",
        title: "expect 提供 panic 信息",
        kind: ExerciseKind::SingleChoice,
        prompt: "`expect(\"config missing\")` 相比 `unwrap()` 多了什么？",
        code: "file_result.expect(\"config missing\");",
        options: &["panic 时带自定义错误信息", "自动修复错误", "把 Err 变成 None"],
        answer: Answer::Choice(0),
        explanation: "`expect` 和 `unwrap` 一样会在失败时 panic，但可以提供更清楚的上下文信息。",
        hint: "它不是错误恢复，只是更好的崩溃信息。",
    },
    Exercise {
        id: "vec-new-type-annotation",
        lesson_id: "collections",
        title: "Vec::new 可能需要类型",
        kind: ExerciseKind::FillBlank,
        prompt: "空 Vec 没有元素可推断类型时，空白处应填什么？",
        code: "let values: Vec<____> = Vec::new();",
        options: &[],
        answer: Answer::Text("i32"),
        explanation: "`Vec::new()` 本身没有元素信息，通常需要通过变量类型标注或后续 push 推断元素类型。",
        hint: "空集合经常需要类型提示。",
    },
    Exercise {
        id: "vec-macro-init",
        lesson_id: "collections",
        title: "vec! 宏初始化",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let values = vec![1, 2, 3];\nprintln!(\"{}\", values.len());",
        options: &[],
        answer: Answer::Output("3"),
        explanation: "`vec![1, 2, 3]` 创建包含三个元素的 Vec，因此长度为 3。",
        hint: "vec! 是宏，不是普通函数。",
    },
    Exercise {
        id: "string-from-vs-literal",
        lesson_id: "collections",
        title: "String::from 与字符串字面量",
        kind: ExerciseKind::SingleChoice,
        prompt: "`String::from(\"hi\")` 相比 `\"hi\"` 的主要区别是什么？",
        code: "let owned = String::from(\"hi\");\nlet borrowed = \"hi\";",
        options: &["String 拥有可增长的堆分配字符串", "String 一定是 static", "字符串字面量可以 push_str"],
        answer: Answer::Choice(0),
        explanation: "`String` 拥有堆上可增长的 UTF-8 字符串；字符串字面量通常是 `&str`。",
        hint: "集合章节的 String 是可增长集合。",
    },
    Exercise {
        id: "hashmap-get-option",
        lesson_id: "collections",
        title: "HashMap::get 返回 Option",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `scores.get(\"Blue\")` 返回 Option？",
        code: "let score = scores.get(\"Blue\");",
        options: &["key 可能不存在", "HashMap 只能存 Option", "get 会删除 key"],
        answer: Answer::Choice(0),
        explanation: "查找 map 时 key 可能不存在，所以 `get` 返回 `Option<&V>` 迫使调用方处理缺失情况。",
        hint: "这和 Vec::get 的安全思想类似。",
    },
    Exercise {
        id: "iterator-any",
        lesson_id: "iterators-traits",
        title: "any 短路判断",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let has_even = [1, 3, 4].iter().any(|n| n % 2 == 0);\nprintln!(\"{has_even}\");",
        options: &[],
        answer: Answer::Output("true"),
        explanation: "`any` 在找到满足条件的元素时返回 true；4 是偶数。",
        hint: "any 是消费适配器，并且会短路。",
    },
    Exercise {
        id: "iterator-find",
        lesson_id: "iterators-traits",
        title: "find 返回 Option",
        kind: ExerciseKind::SingleChoice,
        prompt: "`find` 为什么返回 Option？",
        code: "let found = [1, 2, 3].iter().find(|n| **n > 10);",
        options: &["可能找不到满足条件的元素", "find 会返回 Vec", "find 一定 panic"],
        answer: Answer::Choice(0),
        explanation: "`find` 可能找不到元素，因此返回 `Option<Item>`。",
        hint: "没有匹配项时就是 None。",
    },
    Exercise {
        id: "iterator-collect-string",
        lesson_id: "iterators-traits",
        title: "collect 成 String",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let text: String = ['R', 's', 't'].into_iter().collect();\nprintln!(\"{text}\");",
        options: &[],
        answer: Answer::Output("Rst"),
        explanation: "`collect` 可以把 `char` 迭代器收集成 `String`，目标类型由变量标注给出。",
        hint: "collect 的目标类型很关键。",
    },
    Exercise {
        id: "iterator-closure-capture",
        lesson_id: "iterators-traits",
        title: "迭代器闭包捕获环境",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let factor = 3;\nlet values: Vec<_> = [1, 2].iter().map(|n| n * factor).collect();\nprintln!(\"{:?}\", values);",
        options: &[],
        answer: Answer::Output("[3, 6]"),
        explanation: "闭包可以捕获外部变量 `factor`；map 对每个元素乘以 3。",
        hint: "闭包不只是函数指针，也能捕获环境。",
    },
    Exercise {
        id: "trait-default-method",
        lesson_id: "generics-traits",
        title: "trait 默认方法",
        kind: ExerciseKind::SingleChoice,
        prompt: "trait 中带方法体的方法表示什么？",
        code: "trait Summary {\n    fn summarize(&self) -> String { String::from(\"read more\") }\n}",
        options: &["默认实现，实现类型可以直接使用或覆盖", "必须在每个 impl 里重写", "trait 不能有方法体"],
        answer: Answer::Choice(0),
        explanation: "trait 方法可以提供默认实现；实现该 trait 的类型可以使用默认版本，也可以覆盖它。",
        hint: "Rust Book 的 Summary trait 有默认实现示例。",
    },
    Exercise {
        id: "trait-impl-for-type",
        lesson_id: "generics-traits",
        title: "为类型实现 trait",
        kind: ExerciseKind::FillBlank,
        prompt: "为 NewsArticle 实现 Summary，空白处应填什么关键字？",
        code: "____ Summary for NewsArticle {\n    fn summarize(&self) -> String { String::from(\"...\") }\n}",
        options: &[],
        answer: Answer::Text("impl"),
        explanation: "为类型实现 trait 使用 `impl TraitName for TypeName { ... }` 语法。",
        hint: "impl 既用于固有实现，也用于 trait 实现。",
    },
    Exercise {
        id: "generic-multiple-bounds",
        lesson_id: "generics-traits",
        title: "多个 trait bound",
        kind: ExerciseKind::FillBlank,
        prompt: "T 同时需要 Display 和 Clone，空白处应填什么连接符？",
        code: "fn duplicate<T: Display ____ Clone>(item: T) { }",
        options: &[],
        answer: Answer::Text("+"),
        explanation: "多个 trait bound 用 `+` 连接，例如 `T: Display + Clone`。",
        hint: "impl Trait 参数里也可以写 `impl Display + Clone`。",
    },
    Exercise {
        id: "lifetime-struct-ref",
        lesson_id: "generics-traits",
        title: "结构体保存引用需要生命周期",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么这个结构体需要生命周期参数？",
        code: "struct ImportantExcerpt<'a> {\n    part: &'a str,\n}",
        options: &["结构体字段保存引用，需要说明引用有效期关系", "所有结构体都必须写生命周期", "&str 不能作为字段"],
        answer: Answer::Choice(0),
        explanation: "结构体如果存放引用，必须让编译器知道该引用至少和结构体实例一样有效。",
        hint: "生命周期参数属于引用字段，不属于 owned String 字段。",
    },
    Exercise {
        id: "thread-spawn-return",
        lesson_id: "concurrency",
        title: "线程闭包返回值",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let handle = std::thread::spawn(|| 21 * 2);\nlet value = handle.join().unwrap();\nprintln!(\"{value}\");",
        options: &[],
        answer: Answer::Output("42"),
        explanation: "线程闭包可以返回值，`join()` 成功后拿到该返回值。",
        hint: "join 返回的是 Result。",
    },
    Exercise {
        id: "channel-multiple-send",
        lesson_id: "concurrency",
        title: "发送多条消息",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let (tx, rx) = std::sync::mpsc::channel();\ntx.send(1).unwrap();\ntx.send(2).unwrap();\nprintln!(\"{} {}\", rx.recv().unwrap(), rx.recv().unwrap());",
        options: &[],
        answer: Answer::Output("1 2"),
        explanation: "同一个发送端可以发送多条消息；接收端按发送顺序接收。",
        hint: "mpsc 是多生产者、单消费者。",
    },
    Exercise {
        id: "arc-needed-not-rc",
        lesson_id: "concurrency",
        title: "跨线程共享不能用 Rc",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么多线程共享计数器不用 `Rc<Mutex<T>>`？",
        code: "// Rc<Mutex<i32>> sent into thread",
        options: &["Rc 不是线程安全的引用计数", "Mutex 只能和 Rc 搭配", "Arc 只能单线程使用"],
        answer: Answer::Choice(0),
        explanation: "`Rc<T>` 的引用计数不是原子操作，不能安全跨线程；跨线程共享要用 `Arc<T>`。",
        hint: "Arc 的 A 是 Atomic。",
    },
    Exercise {
        id: "mutex-scope-release-early",
        lesson_id: "concurrency",
        title: "缩小锁作用域",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么常用额外代码块包住 MutexGuard？",
        code: "{\n    let mut n = counter.lock().unwrap();\n    *n += 1;\n}\nprintln!(\"lock released\");",
        options: &["让 guard 更早离开作用域释放锁", "让 Mutex 变成 Copy", "避免编译器检查"],
        answer: Answer::Choice(0),
        explanation: "MutexGuard 离开作用域时释放锁；额外代码块可以让锁在后续耗时操作前更早释放。",
        hint: "锁的释放跟 Drop 相关。",
    },
    Exercise {
        id: "advanced-syntax-numeric-suffix",
        lesson_id: "syntax-basics",
        title: "高阶：数字字面量后缀决定类型",
        kind: ExerciseKind::SingleChoice,
        prompt: "`let x = 10u8;` 中的 `u8` 起什么作用？",
        code: "let x = 10u8;",
        options: &["指定数字字面量的具体整数类型", "把数字转换成字符串", "表示变量可变"],
        answer: Answer::Choice(0),
        explanation: "数字字面量可以带类型后缀，`10u8` 直接把该字面量指定为 `u8`。",
        hint: "后缀属于字面量本身，不是变量名的一部分。",
    },
    Exercise {
        id: "advanced-syntax-never-semicolon",
        lesson_id: "syntax-basics",
        title: "高阶：panic! 的 never 类型",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `let x: i32 = panic!(\"boom\");` 在类型上可以成立？",
        code: "let x: i32 = panic!(\"boom\");",
        options: &["panic! 的返回类型是 never，可以强制转换成任意类型", "panic! 返回 0", "i32 会忽略右侧表达式"],
        answer: Answer::Choice(0),
        explanation: "`panic!` 这类永不正常返回的表达式类型是 `!`，它可以在类型检查中强制转换成任意需要的类型。",
        hint: "这题考察 never type 的直觉。",
    },
    Exercise {
        id: "advanced-syntax-macro-vs-function",
        lesson_id: "syntax-basics",
        title: "高阶：宏调用需要感叹号",
        kind: ExerciseKind::FillBlank,
        prompt: "调用 println 宏时，函数名后面必须带什么符号？",
        code: "println____(\"hi\");",
        options: &[],
        answer: Answer::Text("!"),
        explanation: "`println!` 是宏调用，不是普通函数调用，因此名字后面必须有 `!`。",
        hint: "看到 `!` 优先想到宏。",
    },
    Exercise {
        id: "advanced-control-destructure-enum",
        lesson_id: "control-flow",
        title: "高阶：match 解构枚举携带数据",
        kind: ExerciseKind::FillBlank,
        prompt: "匹配并取出 x、y 两个字段，空白处应填什么模式？",
        code: "match msg {\n    Message::Move { ____, ____ } => println!(\"{x},{y}\"),\n    _ => {}\n}",
        options: &[],
        answer: Answer::Text("x, y"),
        explanation: "结构体样式的枚举变体可以在 match 中直接用字段名解构，`{ x, y }` 会绑定两个字段。",
        hint: "如果字段名和变量名相同，可以使用简写。",
    },
    Exercise {
        id: "advanced-control-at-binding",
        lesson_id: "control-flow",
        title: "高阶：@ 绑定同时检查并保存值",
        kind: ExerciseKind::SingleChoice,
        prompt: "`id @ 3..=7` 的含义是什么？",
        code: "match value {\n    id @ 3..=7 => println!(\"{id}\"),\n    _ => {}\n}",
        options: &["匹配 3 到 7，并把实际值绑定到 id", "只匹配数字 3", "创建一个名为 3..=7 的变量"],
        answer: Answer::Choice(0),
        explanation: "`@` 模式允许在检查子模式的同时，把匹配到的完整值绑定到一个变量名。",
        hint: "这是 Rust 模式匹配里很实用但容易忽略的语法。",
    },
    Exercise {
        id: "advanced-control-matches-macro",
        lesson_id: "control-flow",
        title: "高阶：matches! 宏判断模式",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let value = Some(3);\nprintln!(\"{}\", matches!(value, Some(n) if n > 2));",
        options: &[],
        answer: Answer::Output("true"),
        explanation: "`matches!` 会判断表达式是否匹配给定模式；这里 `Some(3)` 满足 `n > 2` 的 guard。",
        hint: "matches! 常用于布尔判断场景。",
    },
    Exercise {
        id: "advanced-data-diverging-return",
        lesson_id: "data-functions",
        title: "高阶：发散函数返回 never",
        kind: ExerciseKind::SingleChoice,
        prompt: "`fn fail() -> !` 表示什么？",
        code: "fn fail() -> ! { panic!(\"nope\") }",
        options: &["函数永不正常返回", "函数返回空元组", "函数返回任意整数"],
        answer: Answer::Choice(0),
        explanation: "返回类型 `!` 表示函数不会正常返回，例如 panic、无限循环或进程退出。",
        hint: "`!` 不是 unit `()`。",
    },
    Exercise {
        id: "advanced-data-array-copy-init",
        lesson_id: "data-functions",
        title: "高阶：数组重复初始化要求 Copy",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `[String::from(\"x\"); 3]` 不能这样初始化？",
        code: "let values = [String::from(\"x\"); 3];",
        options: &["重复初始化语法需要元素可 Copy", "String 不能放进数组", "数组长度不能是 3"],
        answer: Answer::Choice(0),
        explanation: "`[expr; N]` 会重复同一个表达式的值，通常要求元素类型实现 `Copy`；`String` 不是 Copy。",
        hint: "可以用 `std::array::from_fn` 为每个位置创建独立 String。",
    },
    Exercise {
        id: "advanced-data-tuple-trailing-comma",
        lesson_id: "data-functions",
        title: "高阶：单元素元组需要逗号",
        kind: ExerciseKind::SingleChoice,
        prompt: "`(5,)` 和 `(5)` 的区别是什么？",
        code: "let a = (5,);\nlet b = (5);",
        options: &["`(5,)` 是单元素元组，`(5)` 只是括号表达式", "两者都是 i32 数组", "两者完全一样"],
        answer: Answer::Choice(0),
        explanation: "单元素元组必须带尾随逗号；没有逗号时只是普通括号表达式。",
        hint: "这是 Rust 初学者很容易踩的语法点。",
    },
    Exercise {
        id: "advanced-ownership-partial-move-struct",
        lesson_id: "ownership",
        title: "高阶：结构体字段部分 move",
        kind: ExerciseKind::SingleChoice,
        prompt: "把 `person.name` 移出后，下面哪项仍然通常可用？",
        code: "let person = Person { name: String::from(\"Ada\"), age: 42 };\nlet name = person.name;",
        options: &["person.age", "person 整体", "person.name"],
        answer: Answer::Choice(0),
        explanation: "移动非 Copy 字段后，整个结构体不能再作为完整值使用，但未移动的 Copy 字段仍可单独访问。",
        hint: "这叫 partial move。",
    },
    Exercise {
        id: "advanced-ownership-mem-take",
        lesson_id: "ownership",
        title: "高阶：mem::take 用默认值换出所有权",
        kind: ExerciseKind::SingleChoice,
        prompt: "`std::mem::take(&mut s)` 的核心效果是什么？",
        code: "let mut s = String::from(\"hello\");\nlet old = std::mem::take(&mut s);",
        options: &["把 s 的旧值移出，并留下默认空 String", "克隆 s 的堆数据", "让 s 变成 &'static str"],
        answer: Answer::Choice(0),
        explanation: "`mem::take` 用类型的默认值替换原位置，把旧值作为拥有所有权的值返回；`String::default()` 是空字符串。",
        hint: "这常用于从可变引用里安全拿出所有权。",
    },
    Exercise {
        id: "advanced-ownership-closure-move",
        lesson_id: "ownership",
        title: "高阶：move 闭包捕获所有权",
        kind: ExerciseKind::SingleChoice,
        prompt: "`move || s.len()` 对 String `s` 做了什么？",
        code: "let s = String::from(\"hi\");\nlet f = move || s.len();",
        options: &["把 s 移入闭包环境", "只借用 s 一次", "把 s 转成 &str"],
        answer: Answer::Choice(0),
        explanation: "`move` 闭包会按所有权捕获使用到的变量；对于 `String`，闭包环境会拥有它。",
        hint: "这和线程闭包里的 move 是同一个所有权思想。",
    },
    Exercise {
        id: "advanced-slice-split-at-mut",
        lesson_id: "slices",
        title: "高阶：split_at_mut 安全拆分可变切片",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `split_at_mut` 能返回两个可变切片？",
        code: "let (left, right) = nums.split_at_mut(2);",
        options: &["它保证两个切片不重叠", "它关闭了借用检查", "它会复制整个数组"],
        answer: Answer::Choice(0),
        explanation: "`split_at_mut` 的安全 API 保证返回的两个 `&mut` 切片不重叠，因此满足可变借用独占规则。",
        hint: "多个可变引用可以存在，前提是它们指向不重叠的数据。",
    },
    Exercise {
        id: "advanced-slice-pattern-match",
        lesson_id: "slices",
        title: "高阶：切片模式匹配",
        kind: ExerciseKind::SingleChoice,
        prompt: "`[first, .., last]` 这个切片模式表示什么？",
        code: "match nums {\n    [first, .., last] => println!(\"{first} {last}\"),\n    _ => {}\n}",
        options: &["匹配至少两个元素并绑定首尾", "只匹配两个元素", "匹配空切片"],
        answer: Answer::Choice(0),
        explanation: "切片模式中 `..` 可以匹配中间任意数量元素，`first` 和 `last` 分别绑定首尾。",
        hint: "这是比索引更声明式的写法。",
    },
    Exercise {
        id: "advanced-slice-as-bytes",
        lesson_id: "slices",
        title: "高阶：as_bytes 查看 UTF-8 字节",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let s = \"é\";\nprintln!(\"{}\", s.as_bytes().len());",
        options: &[],
        answer: Answer::Output("2"),
        explanation: "字符串 `é` 在 UTF-8 中占 2 个字节，`as_bytes().len()` 返回字节长度。",
        hint: "字符串长度经常是字节数，不是字符数。",
    },
    Exercise {
        id: "advanced-borrow-two-phase",
        lesson_id: "borrowing",
        title: "高阶：两阶段借用让 push(len) 可行",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `v.push(v.len())` 通常可以编译？",
        code: "let mut v = vec![1, 2];\nv.push(v.len());",
        options: &["方法调用的可变借用会延迟激活，先计算参数", "push 不需要可变借用", "len 会消耗 v"],
        answer: Answer::Choice(0),
        explanation: "Rust 的两阶段借用允许某些方法调用先保留可变借用，再计算参数，真正调用时才激活可变借用。",
        hint: "这是借用检查器为了常见方法调用做的精细化处理。",
    },
    Exercise {
        id: "advanced-borrow-reborrow-mut",
        lesson_id: "borrowing",
        title: "高阶：可变引用重借用",
        kind: ExerciseKind::SingleChoice,
        prompt: "把 `&mut T` 传给函数时，为什么之后还能继续用原可变引用？",
        code: "fn touch(_: &mut String) {}\nlet r = &mut s;\ntouch(r);\nr.push('!');",
        options: &["调用会临时重借用 r，而不是永久移动 r", "&mut String 实现 Copy", "touch 会返回新的 String"],
        answer: Answer::Choice(0),
        explanation: "函数调用会对可变引用做临时重借用；重借用结束后，原来的可变引用可以继续使用。",
        hint: "这解释了很多 `&mut` 参数调用为什么不“消耗”引用。",
    },
    Exercise {
        id: "advanced-borrow-interior-mutability",
        lesson_id: "borrowing",
        title: "高阶：RefCell 把借用检查推迟到运行时",
        kind: ExerciseKind::SingleChoice,
        prompt: "`RefCell<T>` 和普通借用检查的主要区别是什么？",
        code: "let cell = std::cell::RefCell::new(1);",
        options: &["借用规则在运行时检查，违反时 panic", "完全取消借用规则", "只能用于多线程"],
        answer: Answer::Choice(0),
        explanation: "`RefCell<T>` 仍然遵守“多个读或一个写”的规则，只是把检查从编译期移到运行时。",
        hint: "RefCell 适合单线程内部可变性。",
    },
    Exercise {
        id: "advanced-struct-enum-size",
        lesson_id: "structs-enums",
        title: "高阶：枚举大小由最大变体决定",
        kind: ExerciseKind::SingleChoice,
        prompt: "Rust 枚举值为什么需要能容纳最大变体？",
        code: "enum E { A(u8), B([u8; 1024]) }",
        options: &["同一个枚举类型的值必须有统一大小", "每个变体都单独分配到堆上", "小变体会自动压缩成 0 字节"],
        answer: Answer::Choice(0),
        explanation: "一个枚举类型的所有值在编译期需要统一大小，因此通常要能容纳最大变体及判别信息。",
        hint: "这会影响大枚举的内存布局设计。",
    },
    Exercise {
        id: "advanced-struct-option-niche",
        lesson_id: "structs-enums",
        title: "高阶：Option 引用的 niche 优化",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `Option<&T>` 通常和 `&T` 一样大？",
        code: "let maybe: Option<&i32> = None;",
        options: &["None 可以用空指针 niche 表示", "Option 总是零大小", "引用会被复制两份"],
        answer: Answer::Choice(0),
        explanation: "引用不能为 null，因此编译器可以用空指针作为 `None`，让 `Option<&T>` 不需要额外标签空间。",
        hint: "这叫 niche optimization。",
    },
    Exercise {
        id: "advanced-struct-match-ref",
        lesson_id: "structs-enums",
        title: "高阶：匹配引用避免移动字段",
        kind: ExerciseKind::FillBlank,
        prompt: "match 中只想借用 name，空白处应填什么模式关键字？",
        code: "match user {\n    User { name: ____ name, age } => println!(\"{name}\"),\n}",
        options: &[],
        answer: Answer::Text("ref"),
        explanation: "在模式中使用 `ref name` 会绑定字段的引用，避免把 `String` 字段移动出来。",
        hint: "模式里的 ref 和表达式里的 & 位置不同。",
    },
    Exercise {
        id: "advanced-result-transpose",
        lesson_id: "result-option",
        title: "高阶：transpose 翻转 Option<Result>",
        kind: ExerciseKind::SingleChoice,
        prompt: "`Option<Result<T,E>>::transpose()` 会变成什么？",
        code: "let x: Option<Result<i32, &str>> = Some(Ok(1));",
        options: &["Result<Option<T>, E>", "Option<Option<T>>", "Result<T, Option<E>>"],
        answer: Answer::Choice(0),
        explanation: "`transpose` 会把 `Option<Result<T,E>>` 翻转成 `Result<Option<T>,E>`，便于组合可选值和错误传播。",
        hint: "这在解析可选字段时很常见。",
    },
    Exercise {
        id: "advanced-result-ok-or-else",
        lesson_id: "result-option",
        title: "高阶：ok_or_else 延迟构造错误",
        kind: ExerciseKind::SingleChoice,
        prompt: "相比 `ok_or`，`ok_or_else` 的优势是什么？",
        code: "maybe.ok_or_else(|| make_error())?",
        options: &["只有 None 时才执行闭包构造错误", "永远忽略错误", "把 Result 转回 Option"],
        answer: Answer::Choice(0),
        explanation: "`ok_or_else` 接收闭包，只有在 `Option` 是 `None` 时才构造错误，避免不必要的开销。",
        hint: "名字里的 else 暗示兜底逻辑延迟执行。",
    },
    Exercise {
        id: "advanced-result-thiserror-idea",
        lesson_id: "result-option",
        title: "高阶：库函数优先返回错误而非 panic",
        kind: ExerciseKind::SingleChoice,
        prompt: "库函数遇到可恢复失败时，更推荐哪种设计？",
        code: "fn load_config(path: &str) -> ____ { /* ... */ }",
        options: &["返回 Result，让调用方决定如何处理", "直接 panic", "返回 bool 丢掉错误原因"],
        answer: Answer::Choice(0),
        explanation: "可恢复错误应通过 `Result` 暴露给调用方，让上层决定重试、降级、提示还是退出。",
        hint: "panic 更适合不可恢复的 bug 或违反不变量。",
    },
    Exercise {
        id: "advanced-collections-entry-and-modify",
        lesson_id: "collections",
        title: "高阶：entry().and_modify 链式更新",
        kind: ExerciseKind::FillBlank,
        prompt: "已有 key 时先加 1，不存在时插入 1，空白处应填什么？",
        code: "map.entry(word).and_modify(|count| ____).or_insert(1);",
        options: &[],
        answer: Answer::Text("*count += 1"),
        explanation: "`and_modify` 只在 key 已存在时运行闭包，闭包拿到的是 `&mut V`，所以要解引用后修改。",
        hint: "这是 entry API 的高阶组合用法。",
    },
    Exercise {
        id: "advanced-collections-drain",
        lesson_id: "collections",
        title: "高阶：drain 移出范围元素",
        kind: ExerciseKind::SingleChoice,
        prompt: "`v.drain(1..3)` 的核心效果是什么？",
        code: "let removed: Vec<_> = v.drain(1..3).collect();",
        options: &["从 Vec 中移除该范围并产生被移除元素", "只借用该范围不改变 Vec", "复制该范围但保留原元素"],
        answer: Answer::Choice(0),
        explanation: "`drain` 会从集合中移除指定范围，并返回产生被移除元素的迭代器。",
        hint: "它是会修改原集合的消费式操作。",
    },
    Exercise {
        id: "advanced-collections-hashmap-key-move",
        lesson_id: "collections",
        title: "高阶：HashMap insert 会移动 key",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 `key` 在 `map.insert(key, 1)` 后通常不能再使用？",
        code: "let key = String::from(\"blue\");\nmap.insert(key, 1);",
        options: &["String key 的所有权移动进 HashMap", "insert 会自动 clone key", "HashMap 不保存 key"],
        answer: Answer::Choice(0),
        explanation: "`insert` 按值接收 key 和 value，`String` 不是 Copy，因此 key 的所有权会移动进 HashMap。",
        hint: "需要继续使用 key 时，考虑传引用查询或显式 clone。",
    },
    Exercise {
        id: "advanced-iterator-flat-map",
        lesson_id: "iterators-traits",
        title: "高阶：flat_map 展平迭代器",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let values: Vec<_> = [vec![1, 2], vec![3]].into_iter().flat_map(|v| v).collect();\nprintln!(\"{:?}\", values);",
        options: &[],
        answer: Answer::Output("[1, 2, 3]"),
        explanation: "`flat_map` 会把每个元素映射成迭代器，再把这些内部迭代器展平成一个连续序列。",
        hint: "它相当于 map 后再 flatten。",
    },
    Exercise {
        id: "advanced-iterator-by-ref",
        lesson_id: "iterators-traits",
        title: "高阶：by_ref 暂借迭代器",
        kind: ExerciseKind::CodeOutput,
        prompt: "这段代码会输出什么？",
        code: "let mut iter = 1..5;\nlet first: Vec<_> = iter.by_ref().take(2).collect();\nlet rest: Vec<_> = iter.collect();\nprintln!(\"{:?} {:?}\", first, rest);",
        options: &[],
        answer: Answer::Output("[1, 2] [3, 4]"),
        explanation: "`by_ref` 让适配器暂时借用迭代器，因此 take 消费两个元素后，原迭代器还能继续使用剩余元素。",
        hint: "适合分段消费同一个迭代器。",
    },
    Exercise {
        id: "advanced-iterator-partition",
        lesson_id: "iterators-traits",
        title: "高阶：partition 分组成两个集合",
        kind: ExerciseKind::SingleChoice,
        prompt: "`partition` 的返回值是什么？",
        code: "let (even, odd): (Vec<_>, Vec<_>) = nums.into_iter().partition(|n| n % 2 == 0);",
        options: &["满足条件和不满足条件的两个集合", "只返回满足条件的第一个元素", "返回 bool"],
        answer: Answer::Choice(0),
        explanation: "`partition` 会消费迭代器，把元素按谓词结果分到两个集合中。",
        hint: "目标集合类型通常需要标注。",
    },
    Exercise {
        id: "advanced-generics-associated-type",
        lesson_id: "generics-traits",
        title: "高阶：关联类型指定 Iterator::Item",
        kind: ExerciseKind::SingleChoice,
        prompt: "`Iterator<Item = i32>` 中的 `Item` 是什么？",
        code: "fn sum_i32(iter: impl Iterator<Item = i32>) -> i32 { iter.sum() }",
        options: &["Iterator trait 的关联类型", "泛型函数名", "生命周期参数"],
        answer: Answer::Choice(0),
        explanation: "`Item` 是 `Iterator` trait 定义的关联类型，用来表示迭代器每次产出的元素类型。",
        hint: "关联类型让 trait 能声明“实现者会给出一个类型”。",
    },
    Exercise {
        id: "advanced-generics-blanket-impl",
        lesson_id: "generics-traits",
        title: "高阶：blanket impl",
        kind: ExerciseKind::SingleChoice,
        prompt: "`impl<T: Display> ToString for T` 这类实现叫什么？",
        code: "impl<T: Display> ToString for T { /* ... */ }",
        options: &["blanket implementation", "orphan rule", "lifetime elision"],
        answer: Answer::Choice(0),
        explanation: "对所有满足某个约束的类型统一实现 trait，通常称为 blanket implementation。",
        hint: "标准库里有很多这类泛型实现。",
    },
    Exercise {
        id: "advanced-generics-phantom-lifetime",
        lesson_id: "generics-traits",
        title: "高阶：PhantomData 表达类型关系",
        kind: ExerciseKind::SingleChoice,
        prompt: "`PhantomData<&'a T>` 常用于表达什么？",
        code: "struct Holder<'a, T> { marker: std::marker::PhantomData<&'a T> }",
        options: &["类型在逻辑上关联了某个生命周期/类型，即使不直接存字段", "强制分配堆内存", "让 T 自动实现 Copy"],
        answer: Answer::Choice(0),
        explanation: "`PhantomData` 用来告诉编译器某个类型在所有权、生命周期或变型上逻辑关联了 T，即使结构体没有真实字段保存它。",
        hint: "这是写 unsafe 抽象和零大小标记类型时常见的工具。",
    },
    Exercise {
        id: "advanced-concurrency-send-sync",
        lesson_id: "concurrency",
        title: "高阶：Send 与 Sync 的区别",
        kind: ExerciseKind::SingleChoice,
        prompt: "`T: Send` 最准确表示什么？",
        code: "fn spawn_value<T: Send + 'static>(value: T) { /* ... */ }",
        options: &["T 的所有权可以安全转移到另一个线程", "T 的引用一定可以多线程共享", "T 一定不会 panic"],
        answer: Answer::Choice(0),
        explanation: "`Send` 表示值的所有权可以在线程间转移；`Sync` 才表示 `&T` 可以在线程间共享。",
        hint: "很多并发类型错误都和 Send/Sync 边界有关。",
    },
    Exercise {
        id: "advanced-concurrency-mpsc-clone-sender",
        lesson_id: "concurrency",
        title: "高阶：克隆 Sender 实现多生产者",
        kind: ExerciseKind::SingleChoice,
        prompt: "为什么 mpsc 例子里会 clone `tx`？",
        code: "let tx2 = tx.clone();",
        options: &["创建另一个发送端，让多个生产者发消息", "深拷贝整个 channel 缓冲区", "关闭原发送端"],
        answer: Answer::Choice(0),
        explanation: "mpsc 是 multiple producer, single consumer；克隆 Sender 可以让多个线程各自持有发送端。",
        hint: "所有 Sender 都 drop 后，Receiver 才能知道通道关闭。",
    },
    Exercise {
        id: "advanced-concurrency-deadlock-order",
        lesson_id: "concurrency",
        title: "高阶：锁顺序不一致可能死锁",
        kind: ExerciseKind::SingleChoice,
        prompt: "两个线程以相反顺序获取两把锁，主要风险是什么？",
        code: "// thread A locks left then right; thread B locks right then left",
        options: &["死锁", "数据自动 clone", "编译器一定拒绝所有这类代码"],
        answer: Answer::Choice(0),
        explanation: "如果线程 A 持有 left 等 right，同时线程 B 持有 right 等 left，就可能互相等待形成死锁。",
        hint: "Rust 防止数据竞争，但不自动消除所有逻辑死锁。",
    },

];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::lessons;

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

        assert_eq!(
            ids,
            vec![
                "syntax-let-mut",
                "syntax-output",
                "syntax-shadowing",
                "syntax-println-placeholder",
                "syntax-semicolon-unit",
                "syntax-const-binding",
                "syntax-block-scope",
                "syntax-type-inference",
                "syntax-mut-reassign",
                "syntax-const-uppercase",
                "syntax-expression-parentheses",
                "drill-syntax-basics-basic-01",
                "drill-syntax-basics-basic-02",
                "drill-syntax-basics-basic-03",
                "drill-syntax-basics-basic-04",
                "drill-syntax-basics-practice-01",
                "drill-syntax-basics-practice-02",
                "drill-syntax-basics-practice-03",
                "drill-syntax-basics-practice-04",
                "advanced-syntax-numeric-suffix",
                "advanced-syntax-never-semicolon",
                "advanced-syntax-macro-vs-function",
                "drill-syntax-basics-challenge-01",
                "drill-syntax-basics-challenge-02",
                "drill-syntax-basics-challenge-03",
            ]
        );
        assert!(exercises_for_lesson("unknown").is_empty());
    }

    #[test]
    fn generated_drills_expand_bank_above_three_hundred() {
        assert_eq!(exercises().len(), 312);
        assert!(exercises().len() > 300);
    }

    #[test]
    fn every_lesson_has_progressive_difficulty_drills() {
        for lesson in lessons() {
            let lesson_exercises = exercises_for_lesson(lesson.id);
            assert!(
                lesson_exercises.len() >= 25,
                "{} has too few exercises",
                lesson.id
            );

            let mut levels: Vec<u8> = lesson_exercises
                .iter()
                .map(|exercise| exercise.level())
                .collect();
            levels.sort_unstable();
            levels.dedup();
            assert_eq!(
                levels,
                vec![1, 2, 3],
                "{} misses difficulty levels",
                lesson.id
            );

            let drill_count = lesson_exercises
                .iter()
                .filter(|exercise| exercise.id.starts_with("drill-"))
                .count();
            assert!(
                drill_count >= 10,
                "{} has too few visible drills after dedupe",
                lesson.id
            );
        }
    }

    #[test]
    fn lesson_exercises_are_deduped_and_progressive() {
        for lesson in lessons() {
            let mut seen = Vec::new();
            let mut last_level = 0;
            for exercise in exercises_for_lesson(lesson.id) {
                let concept = exercise_concept_key(exercise);
                assert!(
                    !seen.contains(&concept),
                    "{} repeats concept {}",
                    lesson.id,
                    concept
                );
                seen.push(concept);

                assert!(
                    exercise.level() >= last_level,
                    "{} difficulty regressed at {}",
                    lesson.id,
                    exercise.id
                );
                last_level = exercise.level();
            }
        }
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
            assert!((1..=3).contains(&exercise.level()));
            ids.push(exercise.id);
        }
    }
}
