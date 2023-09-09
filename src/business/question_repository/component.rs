use crate::business::{question::component::{QuestionRepository, Question}, answer::component::Answer};

pub struct DummyQuestionRepository;

pub static DUMMY_QUESTION: &str = "2023年1月プログラミング言語ランキングで、Top10に入った言語を5つ答えよ（RedMonk調べ）";

impl DummyQuestionRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl QuestionRepository for DummyQuestionRepository {
    fn get_question(&self) -> Question {
        Question::new(
            DUMMY_QUESTION.to_string(),
            vec![
                omit("Python", vec![
                    "python",
                    "ぱいそん"
                ]),
                omit("JavaScript", vec![
                    "javascript",
                    "js",
                    "じゃばすくりぷと"
                ]),
                omit("Java", vec![
                    "java",
                    "じゃば"
                ]),
                omit("PHP", vec![
                    "php",
                    "ぴーえいちぴー"
                ]),
                omit("C#", vec![
                    "c#",
                    "cシャープ",
                    "しーしゃーぷ"
                ]),
                omit("C++", vec![
                    "c++",
                    "cプラスプラス",
                    "しーぷらすぷらす"
                ]),
                omit("CSS", vec![
                    "css",
                    "しーえすえす"
                ]),
                omit("TypeScript", vec![
                    "typescript",
                    "ts",
                    "たいぷすくりぷと"
                ]),
                omit("Ruby", vec![
                    "ruby",
                    "るびー"
                ]),
                omit("C", vec![
                    "c",
                    "しー"
                ]),
            ],
        )
    }
}

fn omit(
    answer: &str,
    other_answers: Vec<&str>,
) -> Answer {
    Answer::new(
        answer.to_string(),
        other_answers.iter().map(|a| a.to_string()).collect(),
    )
}
