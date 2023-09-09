use crate::business::answer::component::Answer;

pub struct AnswerLog {
    users_answer: String,
    answer: Option<Answer>,
}

impl AnswerLog {
    pub fn new(
        users_answer: String,
        answer: Option<Answer>
    ) -> Self {
        Self {
            users_answer,
            answer,
        }
    }

    pub fn is_correct(&self) -> bool {
        self.answer.is_some()
    }
}
