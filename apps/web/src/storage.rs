use learning_core::ProgressSnapshot;

use crate::app::Theme;
use crate::i18n::Language;

const STORAGE_KEY: &str = "rust-learning-progress-v1";
const LANGUAGE_KEY: &str = "rust-learning-language";
const THEME_KEY: &str = "rust-learning-theme";

#[cfg(target_arch = "wasm32")]
pub fn load_progress() -> ProgressSnapshot {
    use gloo_storage::{LocalStorage, Storage};

    LocalStorage::get(STORAGE_KEY).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_progress() -> ProgressSnapshot {
    ProgressSnapshot::default()
}

#[cfg(target_arch = "wasm32")]
pub fn save_progress(progress: &ProgressSnapshot) {
    use gloo_storage::{LocalStorage, Storage};

    let _ = LocalStorage::set(STORAGE_KEY, progress);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_progress(_: &ProgressSnapshot) {}

#[cfg(target_arch = "wasm32")]
pub fn load_language() -> Language {
    use gloo_storage::{LocalStorage, Storage};

    LocalStorage::get::<String>(LANGUAGE_KEY)
        .map(|code| Language::from_code(&code))
        .unwrap_or(Language::Zh)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_language() -> Language {
    Language::Zh
}

#[cfg(target_arch = "wasm32")]
pub fn save_language(language: Language) {
    use gloo_storage::{LocalStorage, Storage};

    let _ = LocalStorage::set(LANGUAGE_KEY, language.code());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_language(_: Language) {}

#[cfg(target_arch = "wasm32")]
pub fn load_theme() -> Theme {
    use gloo_storage::{LocalStorage, Storage};

    LocalStorage::get::<String>(THEME_KEY)
        .map(|value| Theme::from_code(&value))
        .unwrap_or(Theme::Dark)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_theme() -> Theme {
    Theme::Dark
}

#[cfg(target_arch = "wasm32")]
pub fn save_theme(theme: Theme) {
    use gloo_storage::{LocalStorage, Storage};

    let _ = LocalStorage::set(THEME_KEY, theme.code());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_theme(_: Theme) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_load_progress_returns_default_snapshot() {
        let progress = load_progress();

        assert_eq!(progress.version, 1);
        assert!(progress.completed_exercises.is_empty());
    }

    #[test]
    fn native_save_progress_is_noop() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("syntax-let-mut", true, 1);

        save_progress(&progress);
    }

    #[test]
    fn native_preferences_use_defaults() {
        assert_eq!(load_language(), Language::Zh);
        assert_eq!(load_theme(), Theme::Dark);
    }

    #[test]
    fn native_save_preferences_are_noop() {
        save_language(Language::En);
        save_theme(Theme::Light);
    }
}
