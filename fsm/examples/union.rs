use fsm::{union, AcceptStates, DefineTransform};

#[derive(Clone, Copy, Default, AcceptStates)]
enum StartsWithA {
    #[default]
    Empty,
    #[accept]
    StartedWithA,
    DidNotStartWithA,
}

DefineTransform!(starts_with_a, StartsWithA, char,
    (StartsWithA::Empty, 'a') => StartsWithA::StartedWithA,
    (StartsWithA::Empty, _) => StartsWithA::DidNotStartWithA,
    (StartsWithA::StartedWithA, _) => StartsWithA::StartedWithA,
    (StartsWithA::DidNotStartWithA, _) => StartsWithA::DidNotStartWithA,
);

#[derive(Clone, Copy, Default)]
enum SameStartAndEnd {
    #[default]
    Empty,
    Same(char),
    Different(char),
}

DefineTransform!(same_start_and_end, SameStartAndEnd, char,
    (SameStartAndEnd::Empty, c) => SameStartAndEnd::Same(c),
    (
        SameStartAndEnd::Same(s) | SameStartAndEnd::Different(s),
        c
    ) if s == c => SameStartAndEnd::Same(s),
    (
        SameStartAndEnd::Same(s) | SameStartAndEnd::Different(s),
        _
    ) => SameStartAndEnd::Different(s),
);

impl AcceptStates for SameStartAndEnd {
    #[inline]
    fn is_accepted(&self) -> bool {
        matches!(self, Self::Same(_))
    }
}

fn main() {
    let machine = union::default_with_transforms_copy(starts_with_a, same_start_and_end);

    // Starts and ends with 'a'
    const TEST_1: &str = "abcdefa";
    assert!(machine.run(TEST_1.chars()).is_accepted());
    println!("\"{TEST_1}\": ✅");

    // Starts with 'a' but does not end with 'a'
    const TEST_2: &str = "abcdefg";
    assert!(machine.run(TEST_2.chars()).is_accepted());
    println!("\"{TEST_2}\": ✅");

    // Starts and ends with the same character, but does not start with 'a'
    const TEST_3: &str = "bcdefgb";
    assert!(machine.run(TEST_3.chars()).is_accepted());
    println!("\"{TEST_3}\": ✅");

    // Does not start and end with the same character and does not start with 'a'
    const TEST_4: &str = "bcdefgh";
    assert!(!machine.run(TEST_4.chars()).is_accepted());
    println!("\"{TEST_4}\": ❌");
}

#[test]
fn test() {
    main()
}
