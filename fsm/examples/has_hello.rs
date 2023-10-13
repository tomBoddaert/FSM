use fsm::{DefineTransform, FSM};

// Define the states
#[derive(Default)]
enum Q {
    #[default]
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
}
use Q::*;

// Define the transformation function
DefineTransform!(has_hello, Q, char,
    (Q0, 'h') => Q1,
    (Q1, 'e') => Q2,
    (Q2, 'l') => Q3,
    (Q3, 'l') => Q4,
    (Q4, 'o') => Q5,
    (Q5, _) => Q5,
    (_, _) => Q0,
);

fn main() {
    let tests = vec!["", "abc", "hello", "a hello a", "h ello", "hell o"];

    for test in tests {
        let mut machine = FSM::default_with_transform(has_hello);
        machine = machine.run(test.chars());

        println!(
            "'{test}' => {}",
            match machine.state() {
                Q::Q5 => '✅',
                _ => '❌',
            }
        );
    }
}

#[test]
fn test() {
    main()
}
