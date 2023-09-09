use crate::business::question::component::{QuestionRepository, Question};

pub struct DummyQuestionRepository;

impl DummyQuestionRepository {
    pub fn new() -> Self {
        Self
    }
}

impl QuestionRepository for DummyQuestionRepository {
    fn get_question(&self) -> Question {
        Question::new(
            "dummy question".to_string(),
            vec![],
        )
    }
}
