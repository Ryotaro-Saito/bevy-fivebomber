/**
 * 一つの回答に対しての情報
 * 回答の判定に対する責務を持つ
 * 別解の判定も行う
 */
#[derive(Clone)]
pub struct Answer {
    answer: String,
    other_asnwers: Vec<String>,
    is_already_answered: bool,
}

impl Answer {
    pub fn new(
        answer: String,
        other_asnwers: Vec<String>,
    ) -> Self {
        Self {
            answer,
            other_asnwers,
            is_already_answered: false,
        }
    }

    // 回答のチェック
    pub fn check_answer(&self, users_answer: &String) -> bool {
        if users_answer == &self.answer {
            return true;
        } else {
            return self.other_asnwers.iter().any(|a| a == users_answer);
        }
    }

    pub fn answered(&mut self) {
        self.is_already_answered = true;
    }

    pub fn is_already_answered(&self) -> bool {
        self.is_already_answered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_answer() {
        let answer = Answer::new(
            "answer".to_string(),
            vec!["other1".to_string(), "other2".to_string()]
        );
        assert_eq!(answer.check_answer(&"answer".to_string()), true);
        assert_eq!(answer.check_answer(&"other1".to_string()), true);
        assert_eq!(answer.check_answer(&"other2".to_string()), true);
        assert_eq!(answer.check_answer(&"other3".to_string()), false);
    }
}
