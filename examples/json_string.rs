use fsm::{self, MakeFSM, FSM};

#[derive(Default, Debug)]
enum States {
    #[default]
    Empty,
    InString,
    Special,
    Unicode(u8),
    End,
    Invalid,
}
use States::*;

MakeFSM!(JsonStringValidator, default States, char,
    (Empty, '"') => InString,
    (InString, '"') => End,

    (InString, '\\') => Special,
    (InString, _) => InString,

    (Special, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') => InString,
    (Special, 'u') => Unicode(0),

    (Unicode(3), '0'..='9' | 'a'..='f') => InString,
    (Unicode(n), '0'..='9' | 'a'..='f') => Unicode(n + 1),

    (_, _) => Invalid,
);

impl JsonStringValidator {
    fn is_valid(&self) -> bool {
        matches!(self.state(), End)
    }
}

fn main() {
    let tests = vec![
        (r#""#, false),
        (r#""""#, true),
        (r#"hello"#, false),
        (r#""hello""#, true),
        (r#""hello" world"#, false),
        (r#""hello world""#, true),
        (r#"" \z ""#, false),
        (r#"" \u ""#, false),
        (r#"" \u08a ""#, false),
        (r#"" \u08af ""#, true),
        (r#"" \u08wf ""#, false),
        (r#""hello\" world""#, true),
        (r#""hello\n\tworld""#, true),
        (r#""\""#, false),
        (r#""\\""#, true),
    ];

    for (test, expected) in tests {
        print!("{test}");

        let mut machine = JsonStringValidator::default();
        machine.run(test.chars());

        assert!(machine.is_valid() == expected);
        println!(" => {}", if expected { '✅' } else { '❌' });
    }
}
