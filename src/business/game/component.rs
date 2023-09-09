use bevy::prelude::*;

use crate::business::question::component::{Question, QuestionRepository};

#[derive(Resource)]
pub struct Game<T: QuestionRepository> {
    questions: Vec<Question>,
    question_repository: T,
}

impl<T: QuestionRepository> Game<T> {
    pub fn new(
        question_repository: T,
    ) -> Self {
        Self {
            questions: Vec::new(),
            question_repository,
        }
    }

    fn make_question(&mut self) -> &Question {
        let question = self.question_repository.get_question();
        self.questions.push(question);
        self.questions.last().unwrap()
    }
}
