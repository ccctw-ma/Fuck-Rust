const MONACO_VERSION = "0.52.2";
const MONACO_BASE = `https://cdn.jsdelivr.net/npm/monaco-editor@${MONACO_VERSION}/min/vs`;
const MONACO_LOAD_TIMEOUT_MS = 8000;

let monacoPromise;
let rustProviderRegistered = false;

function loadMonaco() {
  if (window.monaco?.editor) {
    return Promise.resolve(window.monaco);
  }
  if (monacoPromise) {
    return monacoPromise;
  }

  monacoPromise = new Promise((resolve, reject) => {
    const existingLoader = document.querySelector("script[data-monaco-loader]");
    const finish = () => {
      window.require.config({ paths: { vs: MONACO_BASE } });
      window.MonacoEnvironment = {
        getWorkerUrl() {
          return `data:text/javascript;charset=utf-8,${encodeURIComponent(`
            self.MonacoEnvironment = { baseUrl: '${MONACO_BASE}/' };
            importScripts('${MONACO_BASE}/base/worker/workerMain.js');
          `)}`;
        },
      };
      window.require(["vs/editor/editor.main"], () => resolve(window.monaco), reject);
    };

    if (window.require) {
      finish();
      return;
    }

    const loader = existingLoader || document.createElement("script");
    loader.dataset.monacoLoader = "true";
    loader.src = `${MONACO_BASE}/loader.js`;
    loader.async = true;
    loader.onload = finish;
    loader.onerror = () => reject(new Error("Monaco loader failed"));
    if (!existingLoader) {
      document.head.appendChild(loader);
    }
  });

  return monacoPromise;
}

function registerRustLanguage(monaco) {
  if (rustProviderRegistered) {
    return;
  }
  rustProviderRegistered = true;

  monaco.languages.registerCompletionItemProvider("rust", {
    triggerCharacters: [".", ":", "!"],
    provideCompletionItems(model, position) {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };
      const kind = monaco.languages.CompletionItemKind;
      const snippets = [
        ["println!", "println!(\"${1:{} }\");", kind.Function, "print formatted output"],
        ["eprintln!", "eprintln!(\"${1:{} }\");", kind.Function, "print to stderr"],
        ["dbg!", "dbg!(${1:value})", kind.Function, "debug-print expression"],
        ["fn main", "fn main() {\n    ${1}\n}", kind.Snippet, "binary entry point"],
        ["let mut", "let mut ${1:name} = ${2:value};", kind.Keyword, "mutable binding"],
        ["match", "match ${1:value} {\n    ${2:pattern} => ${3:todo!()},\n}", kind.Snippet, "pattern matching"],
        ["if let", "if let Some(${1:value}) = ${2:option} {\n    ${3}\n}", kind.Snippet, "destructure one pattern"],
        ["Result main", "fn main() -> Result<(), Box<dyn std::error::Error>> {\n    ${1}\n    Ok(())\n}", kind.Snippet, "fallible main"],
        ["Option", "Option<${1:T}>", kind.Class, "optional value"],
        ["Vec", "Vec::<${1:T}>::new()", kind.Class, "growable vector"],
        ["String", "String::from(\"${1}\")", kind.Class, "owned UTF-8 string"],
        ["impl", "impl ${1:Type} {\n    ${2}\n}", kind.Keyword, "implementation block"],
        ["struct", "struct ${1:Name} {\n    ${2:field}: ${3:Type},\n}", kind.Keyword, "data structure"],
        ["enum", "enum ${1:Name} {\n    ${2:Variant},\n}", kind.Keyword, "sum type"],
        ["derive", "#[derive(Debug, Clone, PartialEq, Eq)]", kind.Snippet, "common derives"],
        ["use std::", "use std::${1};", kind.Module, "standard library import"],
      ];

      return {
        suggestions: snippets.map(([label, insertText, itemKind, detail]) => ({
          label,
          kind: itemKind,
          detail,
          insertText,
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        })),
      };
    },
  });
}

export function createMonacoRustEditor(container, initialValue, onChange) {
  const handle = {
    editor: null,
    value: initialValue,
    disposed: false,
  };

  container.textContent = "Loading Monaco editor...";
  container.classList.add("is-loading");

  withTimeout(loadMonaco(), MONACO_LOAD_TIMEOUT_MS)
    .then((monaco) => {
      if (handle.disposed) {
        return;
      }
      registerRustLanguage(monaco);
      container.textContent = "";
      container.classList.remove("is-loading");
      handle.editor = monaco.editor.create(container, {
        value: initialValue,
        language: "rust",
        theme: document.documentElement.dataset.theme === "dark" ? "vs-dark" : "vs",
        automaticLayout: true,
        minimap: { enabled: false },
        fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, monospace",
        fontSize: 14,
        lineHeight: 24,
        lineNumbers: "on",
        roundedSelection: true,
        scrollBeyondLastLine: false,
        tabSize: 4,
        insertSpaces: true,
        formatOnPaste: true,
        formatOnType: true,
        suggestOnTriggerCharacters: true,
        quickSuggestions: { other: true, comments: false, strings: false },
        snippetSuggestions: "top",
        wordBasedSuggestions: "currentDocument",
        parameterHints: { enabled: true },
        bracketPairColorization: { enabled: true },
        guides: { bracketPairs: true, indentation: true },
      });
      handle.editor.onDidChangeModelContent(() => {
        handle.value = handle.editor.getValue();
        onChange(handle.value);
      });
      handle.editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => {
        container.closest(".playground-drawer")?.querySelector("[data-playground-run]")?.click();
      });
    })
    .catch(() => {
      container.classList.remove("is-loading");
      container.innerHTML = `<textarea class="monaco-fallback" spellcheck="false"></textarea>`;
      const fallback = container.querySelector("textarea");
      fallback.value = initialValue;
      fallback.addEventListener("input", () => {
        handle.value = fallback.value;
        onChange(handle.value);
      });
    });

  return handle;
}

function withTimeout(promise, timeoutMs) {
  return new Promise((resolve, reject) => {
    const timeout = window.setTimeout(
      () => reject(new Error("Monaco loader timed out")),
      timeoutMs,
    );
    promise.then(
      (value) => {
        window.clearTimeout(timeout);
        resolve(value);
      },
      (error) => {
        window.clearTimeout(timeout);
        reject(error);
      },
    );
  });
}

export function getMonacoEditorValue(handle) {
  return handle?.editor ? handle.editor.getValue() : handle?.value || "";
}

export function layoutMonacoEditor(handle) {
  handle?.editor?.layout();
}

export function disposeMonacoEditor(handle) {
  if (!handle) {
    return;
  }
  handle.disposed = true;
  handle.editor?.dispose();
}
