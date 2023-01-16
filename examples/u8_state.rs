use fsm::{self, FSM};

// In this example,
// false = -1
// true = 1
// and the numbers are in the
// range 0..=31 and wrap

// Define the FSM
fsm::MakeFSM!(Wrap31, u8, bool,
    (0, false) => 31,
    (31, true) => 0,
    (n, false) => n - 1,
    (n, true) => n + 1,
);

fn main() {
    let tests = vec![
        (0, false, 31),
        (0, true, 1),
        (1, false, 0),
        (1, true, 2),
        (30, false, 29),
        (30, true, 31),
        (31, false, 30),
        (31, true, 0),
    ];

    for (init, input, test) in tests {
        print!("{init} + {input}");

        let mut machine = Wrap31::new(init);
        machine.transform(&input);

        assert!(*machine.state() == test);
        println!(" => {}", machine.state());
    }
}
