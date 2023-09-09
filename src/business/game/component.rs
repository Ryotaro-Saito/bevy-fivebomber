use bevy::prelude::*;

use crate::business::{question::component::{Question, QuestionRepository}, answer_log::component::AnswerLog};

/**
 * 現在出題している問題はquestionsの末尾にあるもの
 */
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

    /**
     * 問題を作成し、末尾にセットする
     */
    pub fn set_question(&mut self) {
        let question = self.question_repository.get_question();
        self.questions.push(question);
    }

    /**
     * 問題を解答する
     * 解答の結果が返される
     */
    pub fn answer(&mut self, users_answer: &String) -> &AnswerLog {
        let question = self.questions.last_mut().unwrap();
        question.answer(users_answer);
        question.get_last_answer_log()
    }

    /**
     * 現在出題している問題を取得する
     */
    pub fn get_question(&self) -> &Question {
        self.questions.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::business::{question_repository::component::{DummyQuestionRepository, DUMMY_QUESTION}, game::component::Game};

    #[test]
    fn test_make_question() {
        let question_repository = DummyQuestionRepository::new();
        let mut game = Game::new(question_repository);

        // 問題を取得
        game.set_question();
        let question = game.get_question();
        assert_eq!(question.get_question(), DUMMY_QUESTION);

        // 不正解を解答
        let answer_log = game.answer(&"answer".to_string());
        assert_eq!(answer_log.is_correct(), false);

        // 正解を解答
        let answer_log = game.answer(&"Python".to_string());
        assert_eq!(answer_log.is_correct(), true);
    }
}
