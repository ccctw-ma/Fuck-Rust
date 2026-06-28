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
            Stage::Foundation => "读懂入口层",
            Stage::Ownership => "读懂数据流",
            Stage::Patterns => "读懂抽象层",
            Stage::Production => "读懂工程层",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Stage::Foundation => "从 ripgrep 入口、参数解析和配置读取建立源码阅读手感。",
            Stage::Ownership => "沿着搜索文件、pattern 和 writer 追踪所有权与借用边界。",
            Stage::Patterns => "用 ripgrep 的 builder、Option、Result 和集合理解真实抽象。",
            Stage::Production => "读并行搜索、trait、泛型和生命周期在工程代码里的组合方式。",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Demo {
    pub title: &'static str,
    pub source_path: &'static str,
    pub source_lines: &'static str,
    pub source_role: &'static str,
    pub book_rule: &'static str,
    pub source_url: &'static str,
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
        title: "从 ripgrep 入口读变量与表达式",
        stage: Stage::Foundation,
        minutes: 18,
        difficulty: 1,
        summary: "从 `crates/core/main.rs` 的入口函数读 let、match、返回值和退出码。",
        goals: &[
            "在 main/run 中定位 let 绑定和表达式返回值",
            "理解 ExitCode 如何由 if/match 表达式决定",
            "读懂 ripgrep 源码里的宏调用和早返回",
        ],
        exercise_ids: &[
            "syntax-let-mut",
            "syntax-output",
            "syntax-shadowing",
            "syntax-println-placeholder",
            "syntax-semicolon-unit",
            "syntax-const-binding",
            "syntax-shadow-mutability",
            "syntax-block-scope",
            "syntax-type-inference",
            "syntax-mut-reassign",
            "syntax-const-uppercase",
            "syntax-expression-parentheses",
            "advanced-syntax-numeric-suffix",
            "advanced-syntax-never-semicolon",
            "advanced-syntax-macro-vs-function",
        ],
        book_url: "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html",
        demo: Demo {
            title: "ripgrep: crates/core/main.rs",
            source_path: "crates/core/main.rs",
            source_lines: "L43-L66",
            source_role: "ripgrep 的程序入口，把解析 flags、运行搜索和错误退出码连接起来。",
            book_rule: "match 是表达式，分支最后一行可以成为函数返回值；宏调用负责副作用输出。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs#L43-L66",
            code: "fn main() -> ExitCode {\n    match run(flags::parse()) {\n        Ok(code) => code,\n        Err(err) => {\n            eprintln_locked!(\"{:#}\", err);\n            ExitCode::from(2)\n        }\n    }\n}",
            output: "Ok(code) returns code; Err returns ExitCode 2",
            takeaway: "这段真实入口代码把 Rust Book 的 match 表达式、尾表达式和宏调用放在一起：分支最后一行就是整个函数要返回的退出码。",
        },
    },
    Lesson {
        id: "control-flow",
        title: "从 ripgrep 错误处理读 match 与 if let",
        stage: Stage::Foundation,
        minutes: 20,
        difficulty: 2,
        summary: "用 ripgrep 对 BrokenPipe 和 CLI mode 的处理理解分支、模式和穷尽。",
        goals: &[
            "读懂 ParseResult 和 Mode 的 match 分支",
            "识别 if let Some(...) 的单模式处理",
            "理解每个分支如何决定搜索流程",
        ],
        exercise_ids: &[
            "match-exhaustive",
            "if-let-some",
            "match-return-type",
            "if-expression-value",
            "match-option-none",
            "match-guard",
            "if-let-else",
            "while-let-pop",
            "match-literal-pattern",
            "match-binding-value",
            "if-else-no-semicolon",
            "loop-break-value",
            "advanced-control-destructure-enum",
            "advanced-control-at-binding",
            "advanced-control-matches-macro",
        ],
        book_url: "https://doc.rust-lang.org/book/ch06-02-match.html",
        demo: Demo {
            title: "ripgrep: run 里的模式分发",
            source_path: "crates/core/main.rs",
            source_lines: "L77-L100",
            source_role: "根据 CLI mode 选择搜索、列文件、列类型或生成补全脚本的执行路径。",
            book_rule: "match 必须覆盖所有模式；guard 可以在模式匹配后继续加条件。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs#L77-L100",
            code: "let matched = match args.mode() {\n    Mode::Search(_) if !args.matches_possible() => false,\n    Mode::Search(mode) if args.threads() == 1 => search(&args, mode)?,\n    Mode::Search(mode) => search_parallel(&args, mode)?,\n    Mode::Files if args.threads() == 1 => files(&args)?,\n    Mode::Files => files_parallel(&args)?,\n    Mode::Types => return types(&args),\n    Mode::Generate(mode) => return generate(mode),\n};",
            output: "matched controls the final ExitCode",
            takeaway: "ripgrep 用 match 把 CLI 模式分派到不同执行路径；guard 负责细分条件，所有非 return 分支都产出 bool。",
        },
    },
    Lesson {
        id: "data-functions",
        title: "从 pattern 读取读函数签名与返回值",
        stage: Stage::Foundation,
        minutes: 24,
        difficulty: 2,
        summary: "通过 `patterns_from_reader` 看参数类型、泛型参数、Vec 返回值和错误传播。",
        goals: &[
            "读懂 `R: io::Read` 这类函数签名",
            "理解 `Vec<String>` 如何作为业务结果返回",
            "区分 push 语句和 Ok(patterns) 尾表达式",
        ],
        exercise_ids: &[
            "tuple-destructure",
            "function-return",
            "array-type",
            "statement-vs-expression",
            "function-param-type",
            "tuple-index",
            "array-repeat-init",
            "function-explicit-return",
            "integer-overflow-debug",
            "float-default-f64",
            "char-four-bytes",
            "function-implicit-unit",
            "advanced-data-diverging-return",
            "advanced-data-array-copy-init",
            "advanced-data-tuple-trailing-comma",
        ],
        book_url: "https://doc.rust-lang.org/book/ch03-02-data-types.html",
        demo: Demo {
            title: "ripgrep: crates/cli/src/pattern.rs",
            source_path: "crates/cli/src/pattern.rs",
            source_lines: "L141-L158",
            source_role: "从 reader 逐行读取 pattern，把可搜索模式收集成 Vec<String>。",
            book_rule: "函数签名说明输入、输出和错误；无分号尾表达式返回最终 Result。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/cli/src/pattern.rs#L141-L158",
            code: "pub fn patterns_from_reader<R: io::Read>(rdr: R) -> io::Result<Vec<String>> {\n    let mut patterns = vec![];\n    let mut line_number = 0;\n    io::BufReader::new(rdr).for_byte_line(|line| {\n        line_number += 1;\n        match pattern_from_bytes(line) {\n            Ok(pattern) => {\n                patterns.push(pattern.to_string());\n                Ok(true)\n            }\n            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),\n        }\n    })?;\n    Ok(patterns)\n}",
            output: "Ok(Vec<String>) or io::Error",
            takeaway: "函数签名已经说明输入是任何 io::Read，输出是 `io::Result<Vec<String>>`；最后的 `Ok(patterns)` 是真实业务返回值。",
        },
    },
    Lesson {
        id: "ownership",
        title: "从 CommandReader 读所有权转移",
        stage: Stage::Ownership,
        minutes: 28,
        difficulty: 3,
        summary: "看 ripgrep 如何用 `Option::take` 移出 stdout、关闭进程资源并避免双重所有权。",
        goals: &[
            "判断 Option 里的资源何时被 move 出来",
            "理解 `take()` 为什么留下 None",
            "知道显式 drop 如何释放外部进程管道",
        ],
        exercise_ids: &[
            "ownership-move",
            "ownership-clone",
            "ownership-copy-trap",
            "ownership-borrow-read",
            "ownership-function-takes",
            "ownership-return-ownership",
            "ownership-borrow-vs-clone",
            "ownership-copy-types",
            "ownership-drop-at-scope-end",
            "ownership-reassign-drops-old",
            "ownership-tuple-move-field",
            "ownership-reference-no-drop",
            "advanced-ownership-partial-move-struct",
            "advanced-ownership-mem-take",
            "advanced-ownership-closure-move",
        ],
        book_url: "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html",
        demo: Demo {
            title: "ripgrep: process stdout take",
            source_path: "crates/cli/src/process.rs",
            source_lines: "L218-L242",
            source_role: "关闭外部解压进程的 stdout 管道，并等待子进程退出。",
            book_rule: "move 会转移资源所有权；Option::take 可以从可变位置移出值并留下 None。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/cli/src/process.rs#L218-L242",
            code: "let stdout = match self.child.stdout.take() {\n    None => return Ok(()),\n    Some(stdout) => stdout,\n};\ndrop(stdout);\nlet status = self.child.wait()?;",
            output: "stdout is moved out, then the child process is waited on",
            takeaway: "`take()` 把 Option 里的 stdout 移出来并留下 None，后续不会再次拥有同一个管道；这就是工程代码里的所有权转移。",
        },
    },
    Lesson {
        id: "slices",
        title: "从 pattern bytes 读切片与 &str",
        stage: Stage::Ownership,
        minutes: 24,
        difficulty: 3,
        summary: "用 `pattern_from_bytes` 理解 `&[u8]` 到 `&str` 的借用视图转换。",
        goals: &[
            "读懂 `&[u8]` 表示借来的字节窗口",
            "知道 `&str` 是通过 UTF-8 校验后的借用视图",
            "理解返回引用为什么必须绑定到输入数据",
        ],
        exercise_ids: &[
            "slice-range",
            "first-word-return",
            "string-literal-slice",
            "slice-borrow-blocks-clear",
            "slice-open-ended-range",
            "slice-full-range",
            "slice-string-param",
            "slice-utf8-boundary",
            "slice-array-slice",
            "slice-len-method",
            "slice-mut-slice",
            "slice-first-word-signature",
            "advanced-slice-split-at-mut",
            "advanced-slice-pattern-match",
            "advanced-slice-as-bytes",
        ],
        book_url: "https://doc.rust-lang.org/book/ch04-03-slices.html",
        demo: Demo {
            title: "ripgrep: bytes to pattern str",
            source_path: "crates/cli/src/pattern.rs",
            source_lines: "L67-L74",
            source_role: "把命令行或配置里的原始字节校验成可搜索的 UTF-8 pattern。",
            book_rule: "切片是不拥有数据的借用视图；返回 &str 必须和输入字节切片的生命周期绑定。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/cli/src/pattern.rs#L67-L74",
            code: "fn pattern_from_bytes(pattern: &[u8]) -> Result<&str, InvalidPatternError> {\n    match std::str::from_utf8(pattern) {\n        Ok(pattern) => Ok(pattern),\n        Err(_) => Err(InvalidPatternError(())),\n    }\n}",
            output: "Ok(&str) when bytes are valid UTF-8",
            takeaway: "`&[u8]` 和 `&str` 都是不拥有数据的切片；ripgrep 只是在校验后把同一段输入看成文本模式。",
        },
    },
    Lesson {
        id: "borrowing",
        title: "从 writer 读借用与可变引用",
        stage: Stage::Ownership,
        minutes: 30,
        difficulty: 4,
        summary: "通过 `impl io::Write for StandardStream` 看 `&mut self` 与 `&[u8]` 如何协作。",
        goals: &[
            "识别 `&mut self` 修改 writer 状态",
            "识别 `&[u8]` 只是读取待写入 buffer",
            "理解 match 分支中可变借用的作用范围",
        ],
        exercise_ids: &[
            "borrowing-mut-ref",
            "borrowing-rule",
            "borrow-scope-release",
            "dangling-reference",
            "borrow-immutable-many",
            "borrow-mut-exclusive-error",
            "borrow-reborrow-shared",
            "borrow-function-mut-param",
            "borrow-last-use-nll",
            "borrow-mut-then-read-owner",
            "borrow-cannot-move-while-borrowed",
            "borrow-mut-argument-call",
            "advanced-borrow-two-phase",
            "advanced-borrow-reborrow-mut",
            "advanced-borrow-interior-mutability",
        ],
        book_url: "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html",
        demo: Demo {
            title: "ripgrep: crates/cli/src/wtr.rs",
            source_path: "crates/cli/src/wtr.rs",
            source_lines: "L67-L87",
            source_role: "把不同输出后端统一成 io::Write，让搜索结果写入 stdout/stderr。",
            book_rule: "&mut self 表示独占修改 writer 状态；&[u8] 只是临时读取待写入 buffer。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/cli/src/wtr.rs#L67-L87",
            code: "impl io::Write for StandardStream {\n    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {\n        match self.0 {\n            termcolor::StandardStream::NoColor(ref mut wtr) => wtr.write(buf),\n            termcolor::StandardStream::NoColorLock(ref mut wtr) => wtr.write(buf),\n            termcolor::StandardStream::Print(ref mut wtr) => wtr.write(buf),\n        }\n    }\n}",
            output: "writer mutates itself; buf is only borrowed",
            takeaway: "`&mut self` 让 writer 能写入内部状态，`buf: &[u8]` 只是借来读取；这正是 Rust Book 借用规则在输出层的应用。",
        },
    },
    Lesson {
        id: "structs-enums",
        title: "从解压 matcher 读 struct、impl 与 enum",
        stage: Stage::Patterns,
        minutes: 30,
        difficulty: 3,
        summary: "ripgrep 用 struct 保存配置，用 impl 暴露构建步骤，用 enum 表达 stderr 读取策略。",
        goals: &[
            "看 builder struct 如何组织字段",
            "区分 `new`、`build(&self)`、`defaults(&mut self)`",
            "用 enum 变体表达异步或同步 stderr 读取",
        ],
        exercise_ids: &[
            "struct-update",
            "method-self",
            "enum-match-option",
            "struct-field-init-shorthand",
            "method-mut-self",
            "tuple-struct-access",
            "enum-variant-data",
            "match-enum-method",
            "struct-associated-function",
            "struct-debug-print",
            "enum-if-let-method",
            "method-takes-self",
            "advanced-struct-enum-size",
            "advanced-struct-option-niche",
            "advanced-struct-match-ref",
        ],
        book_url: "https://doc.rust-lang.org/book/ch05-01-defining-structs.html",
        demo: Demo {
            title: "ripgrep: DecompressionMatcherBuilder",
            source_path: "crates/cli/src/decompress.rs",
            source_lines: "L15-L45",
            source_role: "保存解压命令配置，并用 builder 方法逐步构造 matcher。",
            book_rule: "struct 保存相关字段，impl 放类型行为；&mut self 方法修改自身并可返回自身继续链式调用。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/cli/src/decompress.rs#L15-L45",
            code: "pub struct DecompressionMatcherBuilder {\n    commands: Vec<DecompressionCommand>,\n    defaults: bool,\n}\n\nimpl DecompressionMatcherBuilder {\n    pub fn new() -> DecompressionMatcherBuilder {\n        DecompressionMatcherBuilder { commands: vec![], defaults: true }\n    }\n\n    pub fn defaults(&mut self, yes: bool) -> &mut DecompressionMatcherBuilder {\n        self.defaults = yes;\n        self\n    }\n}",
            output: "builder stores state, then returns &mut self for chaining",
            takeaway: "struct 承载配置数据，impl 定义行为；`&mut self` 方法修改 builder 后再把自己借回去，方便链式配置。",
        },
    },
    Lesson {
        id: "result-option",
        title: "从解压命令读 Option、Result 与错误处理",
        stage: Stage::Patterns,
        minutes: 26,
        difficulty: 3,
        summary: "用 `command` 和 `build` 看 ripgrep 如何区分没有匹配、构建失败和正常返回。",
        goals: &[
            "读懂 `Option<Command>` 表达可能不存在",
            "读懂 `Result<_, CommandError>` 表达可能失败",
            "理解 `?` 和 `let Some(...) else` 如何缩短错误路径",
        ],
        exercise_ids: &[
            "option-match",
            "result-question-mark",
            "result-match-ok-err",
            "option-unwrap-risk",
            "question-mark-return-type",
            "result-unwrap-or",
            "option-map",
            "question-mark-option",
            "result-map-err",
            "result-ok-method",
            "option-and-then",
            "result-expect-message",
            "advanced-result-transpose",
            "advanced-result-ok-or-else",
            "advanced-result-thiserror-idea",
        ],
        book_url: "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
        demo: Demo {
            title: "ripgrep: decompressor lookup",
            source_path: "crates/cli/src/decompress.rs",
            source_lines: "L179-L187",
            source_role: "根据文件路径查找匹配的解压命令；找不到时正常返回 None。",
            book_rule: "Option 表达可能没有值；Result 表达可能失败，二者不要混用成隐藏异常。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/cli/src/decompress.rs#L179-L187",
            code: "pub fn command<P: AsRef<Path>>(&self, path: P) -> Option<Command> {\n    if let Some(i) = self.globs.matches(path).into_iter().next_back() {\n        let decomp_cmd = &self.commands[i];\n        let mut cmd = Command::new(&decomp_cmd.bin);\n        cmd.args(&decomp_cmd.args);\n        return Some(cmd);\n    }\n    None\n}",
            output: "Some(Command) or None",
            takeaway: "没有匹配的解压器不是异常，而是 `None`；真正的 I/O 或进程问题才进入 `Result` 错误路径。",
        },
    },
    Lesson {
        id: "collections",
        title: "从配置解析读 Vec、String 与集合",
        stage: Stage::Patterns,
        minutes: 32,
        difficulty: 3,
        summary: "通过 ripgrep 配置文件解析看 `Vec<OsString>`、错误集合和 push。",
        goals: &[
            "看 `vec![]` 如何收集 CLI 参数",
            "理解 push 把拥有的 OsString 放进集合",
            "知道 HashMap 在 flags/type 索引中承担查找职责",
        ],
        exercise_ids: &[
            "vec-mut-push",
            "string-update",
            "hashmap-entry",
            "vec-index-bounds",
            "hashmap-count-entry",
            "vec-iterate-borrow",
            "string-push-char",
            "hashmap-insert-overwrite",
            "vec-new-type-annotation",
            "vec-macro-init",
            "string-from-vs-literal",
            "hashmap-get-option",
            "advanced-collections-entry-and-modify",
            "advanced-collections-drain",
            "advanced-collections-hashmap-key-move",
        ],
        book_url: "https://doc.rust-lang.org/book/ch08-00-common-collections.html",
        demo: Demo {
            title: "ripgrep: config args parser",
            source_path: "crates/core/flags/config.rs",
            source_lines: "L84-L108",
            source_role: "逐行解析配置文件，把参数和解析错误分别收集到两个 Vec。",
            book_rule: "Vec 是可增长集合；push 会把拥有的数据放入集合，修改集合需要 mut。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/core/flags/config.rs#L84-L108",
            code: "fn parse_reader<R: std::io::Read>(\n    rdr: R,\n) -> anyhow::Result<(Vec<OsString>, Vec<anyhow::Error>)> {\n    let mut bufrdr = std::io::BufReader::new(rdr);\n    let (mut args, mut errs) = (vec![], vec![]);\n    bufrdr.for_byte_line_with_terminator(|line| {\n        match line.to_os_str() {\n            Ok(osstr) => args.push(osstr.to_os_string()),\n            Err(err) => errs.push(anyhow::anyhow!(\"{err}\")),\n        }\n        Ok(true)\n    })?;\n    Ok((args, errs))\n}",
            output: "returns collected args and parse errors",
            takeaway: "ripgrep 把配置文件每一行转成拥有所有权的 OsString 后 push 进 Vec，同时把解析错误收集到另一个 Vec。",
        },
    },
    Lesson {
        id: "iterators-traits",
        title: "从搜索 pipeline 读迭代器和 trait",
        stage: Stage::Production,
        minutes: 35,
        difficulty: 5,
        summary: "从 `walk_builder().build().filter_map(...)` 看 ripgrep 如何组织搜索数据流。",
        goals: &[
            "读懂 filter_map 如何过滤不可搜索路径",
            "理解 Iterator trait 的 next 模型",
            "知道 clone worker 是为了并行遍历复用配置",
        ],
        exercise_ids: &[
            "iterator-chain",
            "lifetime-meaning",
            "iter-vs-into-iter",
            "collect-type",
            "iterator-lazy",
            "iterator-filter-borrow",
            "iterator-sum-consumes",
            "iterator-enumerate",
            "iterator-any",
            "iterator-find",
            "iterator-collect-string",
            "iterator-closure-capture",
            "advanced-iterator-flat-map",
            "advanced-iterator-by-ref",
            "advanced-iterator-partition",
        ],
        book_url: "https://doc.rust-lang.org/book/ch13-02-iterators.html",
        demo: Demo {
            title: "ripgrep: search iterator pipeline",
            source_path: "crates/core/main.rs",
            source_lines: "L107-L151",
            source_role: "把目录遍历结果转换成可搜索 haystack，再逐个执行搜索。",
            book_rule: "迭代器适配器是惰性的；filter_map 描述转换，for 循环才真正消费数据流。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs#L107-L151",
            code: "let unsorted = args\n    .walk_builder()?\n    .build()\n    .filter_map(|result| haystack_builder.build_from_result(result));\nlet haystacks = args.sort(unsorted);\n\nfor haystack in haystacks {\n    let search_result = searcher.search(&haystack)?;\n    matched = matched || search_result.has_match();\n}",
            output: "walk entries become searchable haystacks",
            takeaway: "目录遍历先产生迭代器，`filter_map` 丢掉不可搜索项并提取 Haystack，最后 for 循环才真正消费它。",
        },
    },
    Lesson {
        id: "generics-traits",
        title: "从 ripgrep 泛型接口读 trait bound 与生命周期",
        stage: Stage::Production,
        minutes: 36,
        difficulty: 5,
        summary: "用 globset Candidate 和 searcher SinkError 看 AsRef、Display、Debug、生命周期与泛型约束。",
        goals: &[
            "读懂 `AsRef<Path>` 如何接收 path-like 输入",
            "理解 `Display`、`Debug` 这类 trait bound 来自函数体真实需要",
            "解释生命周期如何把借用数据和输入参数关联",
        ],
        exercise_ids: &[
            "generic-largest",
            "trait-bound-display",
            "lifetime-longest",
            "where-clause",
            "static-lifetime-myth",
            "impl-trait-param",
            "derive-debug-bound",
            "lifetime-elision",
            "trait-default-method",
            "trait-impl-for-type",
            "generic-multiple-bounds",
            "lifetime-struct-ref",
            "advanced-generics-associated-type",
            "advanced-generics-blanket-impl",
            "advanced-generics-phantom-lifetime",
        ],
        book_url: "https://doc.rust-lang.org/book/ch10-00-generics.html",
        demo: Demo {
            title: "ripgrep: globset Candidate<'a>",
            source_path: "crates/globset/src/lib.rs",
            source_lines: "L599-L619",
            source_role: "把待匹配路径封装成 Candidate，供 globset 后续判断是否命中。",
            book_rule: "泛型约束说明可接受的参数能力；生命周期把 Candidate 内部借用和输入 path 关联起来。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/globset/src/lib.rs#L599-L619",
            code: "pub struct Candidate<'a> {\n    path: Cow<'a, [u8]>,\n    basename: Option<usize>,\n}\n\nimpl<'a> Candidate<'a> {\n    pub fn new<P: AsRef<Path> + ?Sized>(path: &'a P) -> Candidate<'a> {\n        Candidate::new_candidate(path.as_ref(), false)\n    }\n}",
            output: "Candidate borrows path data for lifetime 'a",
            takeaway: "泛型让 new 接收多种 path-like 输入；生命周期 `'a` 说明 Candidate 里借用的数据不能比传入 path 活得更久。",
        },
    },
    Lesson {
        id: "concurrency",
        title: "从并行搜索读线程、channel 与 Mutex",
        stage: Stage::Production,
        minutes: 38,
        difficulty: 5,
        summary: "通过 ripgrep 的 parallel search 看 AtomicBool、Mutex、channel 和打印线程如何协同。",
        goals: &[
            "知道 channel 如何把 Haystack 交给打印线程",
            "理解 Mutex 如何保护统计信息",
            "读懂 AtomicBool 在 worker 间共享状态",
        ],
        exercise_ids: &[
            "thread-move",
            "channel-send",
            "mutex-lock",
            "arc-clone",
            "mutex-guard-drop",
            "thread-join",
            "channel-recv-block",
            "mutex-poison-unwrap",
            "thread-spawn-return",
            "channel-multiple-send",
            "arc-needed-not-rc",
            "mutex-scope-release-early",
            "advanced-concurrency-send-sync",
            "advanced-concurrency-mpsc-clone-sender",
            "advanced-concurrency-deadlock-order",
        ],
        book_url: "https://doc.rust-lang.org/book/ch16-00-concurrency.html",
        demo: Demo {
            title: "ripgrep: files_parallel",
            source_path: "crates/core/main.rs",
            source_lines: "L271-L326",
            source_role: "多个 worker 并行遍历文件，把要打印的路径交给单独打印线程。",
            book_rule: "channel 通过发送值转移所有权；thread::spawn 常用 move 让线程拥有捕获数据。",
            source_url: "https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs#L271-L326",
            code: "let (tx, rx) = mpsc::channel::<crate::haystack::Haystack>();\nlet print_thread = thread::spawn(move || -> std::io::Result<()> {\n    for haystack in rx.iter() {\n        path_printer.write(haystack.path())?;\n    }\n    Ok(())\n});\n\nlet tx = tx.clone();\nmatch tx.send(haystack) {\n    Ok(_) => WalkState::Continue,\n    Err(_) => WalkState::Quit,\n}",
            output: "workers send haystacks; one thread prints paths",
            takeaway: "ripgrep 用 channel 转移 Haystack 所有权到打印线程，避免多个 worker 同时写 stdout。",
        },
    },
];

pub const CARDS: &[KnowledgeCard] = &[
    KnowledgeCard {
        id: "move-after-use",
        title: "ripgrep 里 move 后不能再用原变量",
        tag: "ownership",
        summary: "`tx.send(haystack)` 会把 Haystack 所有权交给打印线程，worker 不能再使用它。",
        wrong: "tx.send(haystack)?; println!(\"{:?}\", haystack.path());",
        fix: "像 ripgrep 的 `files_parallel` 一样，发送前完成本线程需要的判断，或只借用 `haystack.path()`。",
    },
    KnowledgeCard {
        id: "mut-ref-exclusive",
        title: "ripgrep writer 的可变借用必须独占",
        tag: "borrow",
        summary: "`searcher.printer().get_mut().clear()` 要独占修改 buffer，不能同时保留共享借用。",
        wrong: "let p = searcher.printer(); searcher.printer().get_mut().clear();",
        fix: "把读取和修改拆成不重叠的作用域，让 `get_mut()` 独占访问 writer。",
    },
    KnowledgeCard {
        id: "question-mark",
        title: "ripgrep 里的 ? 是错误传播",
        tag: "error",
        summary: "`walk_builder()?`、`args.matcher()?` 失败时会直接返回错误，不会继续搜索。",
        wrong: "fn search(args: &HiArgs) { let walk = args.walk_builder()?; }",
        fix: "让函数返回 `anyhow::Result<_>`，或像 BrokenPipe 分支那样手写 match 做特殊处理。",
    },
    KnowledgeCard {
        id: "lifetime-not-extend",
        title: "Candidate<'a> 的生命周期不延长引用",
        tag: "lifetime",
        summary: "ripgrep globset 的 `Candidate<'a>` 只是说明 path 借自外部，不能让临时 Path 活更久。",
        wrong: "fn candidate<'a>() -> Candidate<'a> { Candidate::new(&PathBuf::from(\"src\")) }",
        fix: "让调用者传入 path，或返回拥有所有权的数据；生命周期标注只描述借用关系。",
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
        assert_eq!(Stage::Foundation.label(), "读懂入口层");
        assert!(Stage::Production.description().contains("并行搜索"));
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
