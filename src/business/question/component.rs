use crate::business::{answer::component::Answer, answer_log::component::AnswerLog};

pub struct Question {
    question: String,
    answers: Vec<Answer>,
    answer_logs: Vec<AnswerLog>,
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
            answer_logs: Vec::new(),
        }
    }

    /**
     * 問題に対する回答を行う
     * 回答済みでない回答で、正解の回答があれば正解とする
     * 正解の場合は解答済みフラグを立てる
     * 正解・不正解に関わらず「解答ログ」を作成する
     */
    pub fn answer(&mut self, users_answer: &String) {
        let solved_answer = self.answers.iter_mut().find(|a| !a.is_already_answered() && a.check_answer(users_answer));

        let cloned_answer = if let Some(solved_answer) = solved_answer {
            // 解答済みフラグを立てる
            solved_answer.answered();
            Some(solved_answer.clone())
        } else {
            None
        };

        /*
         * answerの呼び出し元で解答後のイベントを発火させたい。
         * そのため、answerの結果がどうだったか、という情報を返す必要がある。
         * そのため、users_asnwerが何のanswerに対して解答ができたかを確認したい。
         */ 
        // 解答ログを作成
        self.answer_logs.push(AnswerLog::new(
            users_answer.clone(),
            cloned_answer,
        ));
    }

    pub fn get_question(&self) -> &String {
        &self.question
    }

    pub fn get_last_answer_log(&self) -> &AnswerLog {
        &self.answer_logs.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        let mut question = Question::new(
            "question".to_string(),
            vec![
                Answer::new(
                    "answer".to_string(),
                    vec!["other1".to_string(), "other2".to_string()],
                ),
                Answer::new(
                    "answer2".to_string(),
                    vec!["other3".to_string(), "other4".to_string()],
                ),
            ],
        );
        // 通常正解
        question.answer(&"answer".to_string());
        assert_eq!(question.get_last_answer_log().is_correct(), true);
        // すでに回答済み
        question.answer(&"other1".to_string());
        assert_eq!(question.get_last_answer_log().is_correct(), false);
        // すでに回答済み
        question.answer(&"other2".to_string());
        assert_eq!(question.get_last_answer_log().is_correct(), false);
        // 別解正解
        question.answer(&"other3".to_string());
        assert_eq!(question.get_last_answer_log().is_correct(), true);
        // すでに回答済み
        question.answer(&"other4".to_string());
        assert_eq!(question.get_last_answer_log().is_correct(), false);
        // 不正解
        question.answer(&"other5".to_string());
        assert_eq!(question.get_last_answer_log().is_correct(), false);
        assert_eq!(question.answer_logs.len(), 6);
    }
}
