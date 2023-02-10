# FSM

FSM is a simple crate for defining Finite State Machines

This is not recommended for actual use but it could be used for program state management.

## Adding to your project

Add this to your `Cargo.toml`

``` toml
[dependencies]
fsm = { git = "https://github.com/tomboddaert/fsm" }
```

## Example

See more examples in the [examples](/examples) directory.

A tristate FSM, with `Prev` and `Next` inputs, that defaults to the first state and does not wrap.

``` rust
use fsm::{self, FSM, MakeFSM};

#[derive(Default)]
enum States {
    #[default]
    S0,
    S1,
    S2,
}
use States::*;

enum Inputs {
    Prev,
    Next,
}
use Inputs::*;

// Define the Finite State Machine
MakeFSM!(Tristate, default States, Inputs,
    (S0, Prev) => S0,
    (S0, Next) => S1,
    (S1, Prev) => S0,
    (S1, Next) => S2,
    (S2, Prev) => S1,
    (S2, Next) => S2,
);

// Test increasing
let mut machine = Tristate::default();
assert!(matches!(machine.state(), S0));

machine.transform(&Next);
assert!(matches!(machine.state(), S1));

machine.transform(&Next);
assert!(matches!(machine.state(), S2));

machine.transform(&Next);
assert!(matches!(machine.state(), S2));

// Test decreasing
let mut machine = Tristate::new(S2);
assert!(matches!(machine.state(), S2));

machine.transform(&Prev);
assert!(matches!(machine.state(), S1));

machine.transform(&Prev);
assert!(matches!(machine.state(), S0));

machine.transform(&Prev);
assert!(matches!(machine.state(), S0));
```