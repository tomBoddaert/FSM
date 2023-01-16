use fsm::{self, FSM};

// Define the input range
#[derive(Debug)]
enum Sigma {
    A,
    B,
}

// Define the states
#[derive(Default)]
enum Q {
    #[default]
    Q0,
    Q1,
    Q2,
}

// Implement States for Q
fsm::MakeFSM!(No2As, default Q, Sigma,
    // If an A is given, move to Q1
    (Q::Q0, Sigma::A) => Q::Q1,
    (Q::Q0, Sigma::B) => Q::Q0,
    // If another A is given, move to Q3,
    //  else move back to Q0
    (Q::Q1, Sigma::A) => Q::Q2,
    (Q::Q1, Sigma::B) => Q::Q0,
    // Stay on Q3
    (Q::Q2, _) => Q::Q2,
);

fn main() {
    let tests = vec![
        vec![],
        vec![Sigma::A],
        vec![Sigma::A, Sigma::A],
        vec![Sigma::A, Sigma::A, Sigma::B],
        vec![Sigma::A, Sigma::B],
        vec![Sigma::A, Sigma::B, Sigma::A],
        vec![Sigma::A, Sigma::B, Sigma::A, Sigma::A],
    ];

    for inputs in tests {
        print!("{inputs:?}");

        // Create a new FSM using the default state
        let mut machine = No2As::default();
        // Run the inputs
        machine.run(inputs.into_iter());

        // Check if the end state is a 'final' state
        let output = matches!(machine.state(), Q::Q0 | Q::Q1);
        println!(" => {}", if output { '✅' } else { '❌' });
    }
}
