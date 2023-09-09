pub struct Question {
    question: String,
    answers: Vec<Answer>,
    answer_log: Vec<AnswerLog>,
}

pub trait QuestionRepository {
    fn get_question(&self) -> Question;
}

impl Question {
    pub fn new(
        question: String,
        answers: Vec<Answer>,
    ) -> Self {
        Self {
            question,
            answers,
            answer_log: Vec::new(),
        }
    }

    fn answer(&mut self, answer: &String) {
        let is_correct = self.answers.iter().any(|a| a.check_answer(answer));
        self.answer_log.push(AnswerLog {
        answer: answer.clone(),
        is_correct,
        });
    }
}

/**
 * 一つの回答に対しての情報
 * 回答の判定に対する責務を持つ
 * 別解の判定も行う
 */
pub struct Answer;

impl Answer {
    pub fn new() -> Self {
        Self {}
    }

    // 回答のチェック
    pub fn check_answer(&self, _answer: &str) -> bool {
        // TODO: 回答のチェック
        true
    }
}

struct AnswerLog {
    answer: String,
    is_correct: bool,
}
