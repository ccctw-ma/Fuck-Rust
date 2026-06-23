pub mod curriculum;
pub mod exercises;
pub mod progress;

pub use curriculum::{
    cards, lessons, Demo, KnowledgeCard, Lesson, LessonProgress, Stage, StageSummary,
};
pub use exercises::{
    exercise_by_id, exercises, exercises_for_lesson, Answer, CheckOutcome, Exercise,
    ExerciseDifficulty, ExerciseKind, UserAnswer,
};
pub use progress::{AttemptRecord, ProgressSnapshot, WeakLesson};

pub fn recommend_next_exercise(progress: &ProgressSnapshot) -> Option<&'static Exercise> {
    exercises()
        .iter()
        .find(|exercise| !progress.is_completed(exercise.id))
}

pub fn recommend_next_lesson(progress: &ProgressSnapshot) -> Option<&'static Lesson> {
    let next = recommend_next_exercise(progress)?;
    lessons().iter().find(|lesson| lesson.id == next.lesson_id)
}

pub fn stage_summaries(progress: &ProgressSnapshot) -> Vec<StageSummary> {
    Stage::all()
        .iter()
        .map(|stage| {
            let stage_lessons: Vec<&Lesson> = lessons()
                .iter()
                .filter(|lesson| lesson.stage == *stage)
                .collect();
            let exercise_count = stage_lessons
                .iter()
                .map(|lesson| exercises_for_lesson(lesson.id).len())
                .sum();
            let completed_count = stage_lessons
                .iter()
                .flat_map(|lesson| exercises_for_lesson(lesson.id))
                .filter(|exercise| progress.is_completed(exercise.id))
                .count();

            StageSummary {
                stage: *stage,
                lesson_count: stage_lessons.len(),
                exercise_count,
                completed_count,
            }
        })
        .collect()
}

pub fn lesson_progress(progress: &ProgressSnapshot) -> Vec<LessonProgress> {
    lessons()
        .iter()
        .map(|lesson| {
            let lesson_exercises = exercises_for_lesson(lesson.id);
            let total = lesson_exercises.len();
            let completed = lesson_exercises
                .iter()
                .filter(|exercise| progress.is_completed(exercise.id))
                .count();

            LessonProgress {
                lesson,
                total,
                completed,
                locked: false,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommends_first_incomplete_exercise_and_lesson() {
        let progress = ProgressSnapshot::default();

        let exercise = recommend_next_exercise(&progress).expect("exercise exists");
        let lesson = recommend_next_lesson(&progress).expect("lesson exists");

        assert_eq!(exercise.id, "syntax-let-mut");
        assert_eq!(lesson.id, "syntax-basics");
    }

    #[test]
    fn skips_completed_exercises_when_recommending() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("syntax-let-mut", true, 10);

        let exercise = recommend_next_exercise(&progress).expect("next exercise exists");

        assert_eq!(exercise.id, "syntax-output");
    }

    #[test]
    fn returns_none_when_all_exercises_are_completed() {
        let mut progress = ProgressSnapshot::default();
        for exercise in exercises() {
            progress.record_attempt(exercise.id, true, 10);
        }

        assert!(recommend_next_exercise(&progress).is_none());
        assert!(recommend_next_lesson(&progress).is_none());
    }

    #[test]
    fn builds_stage_summaries_from_static_curriculum() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("syntax-let-mut", true, 10);
        progress.record_attempt("syntax-output", true, 11);

        let summaries = stage_summaries(&progress);
        let foundation = summaries
            .iter()
            .find(|summary| summary.stage == Stage::Foundation)
            .expect("foundation summary");

        assert_eq!(summaries.len(), Stage::all().len());
        assert_eq!(foundation.lesson_count, 3);
        assert_eq!(foundation.exercise_count, 78);
        assert_eq!(foundation.completed_count, 2);
    }

    #[test]
    fn lesson_progress_counts_completed_exercises() {
        let mut progress = ProgressSnapshot::default();
        progress.record_attempt("borrowing-mut-ref", true, 10);

        let borrowing = lesson_progress(&progress)
            .into_iter()
            .find(|item| item.lesson.id == "borrowing")
            .expect("borrowing lesson progress");

        assert_eq!(borrowing.total, 26);
        assert_eq!(borrowing.completed, 1);
        assert!(!borrowing.locked);
        assert_eq!(borrowing.rate(), 1.0 / 26.0);
    }
}
