# FSM

FSM is a simple crate for defining Finite State Machines

## Adding to your project

Add this to your `Cargo.toml`

``` toml
[dependencies]
fsm = { git = "https://github.com/tomboddaert/fsm" }
```

## Example

See more examples in the [examples](/examples) directory.

A tristate FSM, with `Prev` and `Next` inputs, that defaults to the first state and does not wrap.

```rust
use fsm::{DefineTransform, FSM};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

// Define the transform function
DefineTransform!(tristate, States, Inputs,
    (S0, Prev) => S0,
    (S0, Next) => S1,
    (S1, Prev) => S0,
    (S1, Next) => S2,
    (S2, Prev) => S1,
    (S2, Next) => S2,
);

// Test increasing
let mut machine = FSM::default_with_transform(tristate);
assert_eq!(machine.state(), &S0);

machine.apply_assign(Next);
assert_eq!(machine.state(), &S1);

machine.apply_assign(Next);
assert_eq!(machine.state(), &S2);

machine.apply_assign(Next);
assert_eq!(machine.state(), &S2);

// Test decreasing
let mut machine = FSM::new(S2, tristate);
assert_eq!(machine.state(), &S2);

machine.apply_assign(Prev);
assert_eq!(machine.state(), &S1);

machine.apply_assign(Prev);
assert_eq!(machine.state(), &S0);

machine.apply_assign(Prev);
assert_eq!(machine.state(), &S0);
```
