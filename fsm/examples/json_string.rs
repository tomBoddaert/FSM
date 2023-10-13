use fsm::{AcceptStates, DefineTransform, FSM};

#[derive(Debug, Default, PartialEq)]
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

// Define the transformation function
DefineTransform!(validate_json_char, States, char,
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

impl AcceptStates for States {
    fn is_accepted(&self) -> bool {
        self == &End
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

        let mut machine = FSM::default_with_transform(validate_json_char);
        machine = machine.run(test.chars());

        assert!(machine.is_accepted() == expected);
        println!(" => {}", if expected { '✅' } else { '❌' });
    }
}

#[test]
fn test() {
    main()
}
