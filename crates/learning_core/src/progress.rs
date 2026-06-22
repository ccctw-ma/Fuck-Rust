use serde::{Deserialize, Serialize};

use crate::curriculum::Lesson;
use crate::exercises::{exercises_for_lesson, Exercise};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptRecord {
    pub exercise_id: String,
    pub correct: bool,
    pub timestamp: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressSnapshot {
    pub version: u8,
    pub completed_exercises: Vec<String>,
    pub attempts: Vec<AttemptRecord>,
}

impl Default for ProgressSnapshot {
    fn default() -> Self {
        Self {
            version: 1,
            completed_exercises: Vec::new(),
            attempts: Vec::new(),
        }
    }
}

impl ProgressSnapshot {
    pub fn record_attempt(&mut self, exercise_id: &str, correct: bool, timestamp: u64) {
        self.attempts.push(AttemptRecord {
            exercise_id: exercise_id.to_owned(),
            correct,
            timestamp,
        });

        if correct && !self.is_completed(exercise_id) {
            self.completed_exercises.push(exercise_id.to_owned());
        }
    }

    pub fn is_completed(&self, exercise_id: &str) -> bool {
        self.completed_exercises
            .iter()
            .any(|completed| completed == exercise_id)
    }

    pub fn completion_rate(&self, total_exercises: usize) -> f32 {
        if total_exercises == 0 {
            0.0
        } else {
            self.completed_exercises.len() as f32 / total_exercises as f32
        }
    }

    pub fn correct_rate(&self) -> f32 {
        if self.attempts.is_empty() {
            0.0
        } else {
            let correct_count = self
                .attempts
                .iter()
                .filter(|attempt| attempt.correct)
                .count();
            correct_count as f32 / self.attempts.len() as f32
        }
    }

    pub fn streak_days(&self) -> usize {
        let mut days: Vec<u64> = self
            .attempts
            .iter()
            .map(|attempt| attempt.timestamp / 86_400)
            .collect();
        days.sort_unstable();
        days.dedup();

        if days.is_empty() {
            return 0;
        }

        let mut streak = 1;
        for pair in days.windows(2).rev() {
            if pair[1] == pair[0] + 1 {
                streak += 1;
            } else {
                break;
            }
        }
        streak
    }

    pub fn weak_lessons(&self, lessons: &'static [Lesson]) -> Vec<WeakLesson> {
        let mut weak = Vec::new();

        for lesson in lessons {
            let lesson_exercises = exercises_for_lesson(lesson.id);
            let attempts = attempts_for_exercises(&self.attempts, &lesson_exercises);
            if attempts.len() < 2 {
                continue;
            }

            let correct = attempts.iter().filter(|attempt| attempt.correct).count();
            let rate = correct as f32 / attempts.len() as f32;
            if rate < 0.7 {
                weak.push(WeakLesson {
                    lesson_id: lesson.id,
                    title: lesson.title,
                    correct_rate: rate,
                    attempts: attempts.len(),
                });
            }
        }

        weak.sort_by(|left, right| {
            left.correct_rate
                .partial_cmp(&right.correct_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        weak
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeakLesson {
    pub lesson_id: &'static str,
    pub title: &'static str,
    pub correct_rate: f32,
    pub attempts: usize,
}

fn attempts_for_exercises<'a>(
    attempts: &'a [AttemptRecord],
    exercises: &[&Exercise],
) -> Vec<&'a AttemptRecord> {
    attempts
        .iter()
        .filter(|attempt| {
            exercises
                .iter()
                .any(|exercise| exercise.id == attempt.exercise_id)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::lessons;

    #[test]
    fn default_progress_is_versioned_and_empty() {
        let progress = ProgressSnapshot::default();

        assert_eq!(progress.version, 1);
        assert!(progress.completed_exercises.is_empty());
        assert!(progress.attempts.is_empty());
    }

    #[test]
    fn correct_attempt_completes_exercise_once() {
        let mut progress = ProgressSnapshot::default();

        progress.record_attempt("syntax-let-mut", true, 100);
        progress.record_attempt("syntax-let-mut", true, 101);

        assert_eq!(progress.attempts.len(), 2);
        assert_eq!(progress.completed_exercises, vec!["syntax-let-mut"]);
        assert!(progress.is_completed("syntax-let-mut"));
    }

    #[test]
    fn wrong_attempt_does_not_complete_exercise() {
        let mut progress = ProgressSnapshot::default();

        progress.record_attempt("syntax-let-mut", false, 100);

        assert_eq!(progress.attempts.len(), 1);
        assert!(progress.completed_exercises.is_empty());
        assert!(!progress.is_completed("syntax-let-mut"));
    }

    #[test]
    fn completion_rate_handles_empty_and_non_empty_totals() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("syntax-let-mut", true, 100);

        assert_eq!(progress.completion_rate(0), 0.0);
        assert_eq!(progress.completion_rate(4), 0.25);
    }

    #[test]
    fn correct_rate_counts_all_attempts() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("a", true, 100);
        progress.record_attempt("b", false, 101);
        progress.record_attempt("b", true, 102);

        assert_eq!(progress.correct_rate(), 2.0 / 3.0);
    }

    #[test]
    fn empty_correct_rate_is_zero() {
        assert_eq!(ProgressSnapshot::default().correct_rate(), 0.0);
    }

    #[test]
    fn streak_days_counts_contiguous_recent_days() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("a", true, 86_400);
        progress.record_attempt("b", true, 2 * 86_400 + 10);
        progress.record_attempt("c", true, 3 * 86_400 + 20);

        assert_eq!(progress.streak_days(), 3);
    }

    #[test]
    fn streak_days_stops_at_gap() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("a", true, 86_400);
        progress.record_attempt("b", true, 3 * 86_400);
        progress.record_attempt("c", true, 4 * 86_400);

        assert_eq!(progress.streak_days(), 2);
    }

    #[test]
    fn empty_streak_is_zero() {
        assert_eq!(ProgressSnapshot::default().streak_days(), 0);
    }

    #[test]
    fn weak_lessons_returns_low_accuracy_lessons() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("ownership-move", false, 1);
        progress.record_attempt("ownership-clone", false, 2);
        progress.record_attempt("syntax-let-mut", true, 3);
        progress.record_attempt("syntax-output", true, 4);

        let weak = progress.weak_lessons(lessons());

        assert_eq!(weak.len(), 1);
        assert_eq!(weak[0].lesson_id, "ownership");
        assert_eq!(weak[0].attempts, 2);
        assert_eq!(weak[0].correct_rate, 0.0);
    }

    #[test]
    fn weak_lessons_ignores_small_samples() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("ownership-move", false, 1);

        assert!(progress.weak_lessons(lessons()).is_empty());
    }

    #[test]
    fn progress_roundtrips_as_json() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("syntax-let-mut", true, 123);

        let encoded = serde_json::to_string(&progress).expect("serialize progress");
        let decoded: ProgressSnapshot = serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, progress);
    }
}
