//! FSM is a simple crate for defining Finite State Machines
//!
//! # Examples
//! A tristate FSM, with `Prev` and `Next` inputs, that defaults
//! to the first state and does not wrap.
//! ```
//! use fsm::{DefineTransform, FSM};
//!
//! #[derive(Clone, Copy, Debug, Default, PartialEq)]
//! enum States {
//!     #[default]
//!     S0,
//!     S1,
//!     S2,
//! }
//! use States::*;
//!
//! enum Inputs {
//!     Prev,
//!     Next,
//! }
//! use Inputs::*;
//!
//! // Define the transform function
//! DefineTransform!(tristate, States, Inputs,
//!     (S0, Prev) => S0,
//!     (S0, Next) => S1,
//!     (S1, Prev) => S0,
//!     (S1, Next) => S2,
//!     (S2, Prev) => S1,
//!     (S2, Next) => S2,
//! );
//!
//! // Test increasing
//! let mut machine = FSM::default_with_transform(tristate);
//! assert_eq!(machine.state(), &S0);
//!
//! machine.apply_assign(Next);
//! assert_eq!(machine.state(), &S1);
//!
//! machine.apply_assign(Next);
//! assert_eq!(machine.state(), &S2);
//!
//! machine.apply_assign(Next);
//! assert_eq!(machine.state(), &S2);
//!
//! // Test decreasing
//! let mut machine = FSM::new(S2, tristate);
//! assert_eq!(machine.state(), &S2);
//!
//! machine.apply_assign(Prev);
//! assert_eq!(machine.state(), &S1);
//!
//! machine.apply_assign(Prev);
//! assert_eq!(machine.state(), &S0);
//!
//! machine.apply_assign(Prev);
//! assert_eq!(machine.state(), &S0);
//! ```

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo,
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::get_unwrap,
    clippy::panic_in_result_fn,
    clippy::todo,
    clippy::undocumented_unsafe_blocks
)]
#![cfg_attr(not(feature = "std"), no_std)]

mod accept_states;
mod fsm;
mod transform;
pub use accept_states::AcceptStates;
pub use fsm::FSM;
pub mod intersection;
pub mod union;

#[cfg(feature = "derive")]
pub use fsm_derive::AcceptStates;

#[cfg(test)]
mod test {
    use crate as fsm;
    #[cfg(feature = "derive")]
    use fsm::AcceptStates;
    use fsm::{DefineTransform, FSM};

    #[cfg_attr(feature = "derive", derive(AcceptStates))]
    #[derive(Debug, Default, PartialEq, Eq)]
    enum States {
        #[cfg_attr(feature = "derive", accept)]
        #[default]
        Q1,
        Q2,
    }

    enum Domain {
        A,
        B,
    }

    #[cfg(feature = "derive")]
    #[test]
    fn test_derive() {
        assert!(States::Q1.is_accepted());
        assert!(!States::Q2.is_accepted());
    }

    DefineTransform!(transform, States, Domain,
        (States::Q1, _) => States::Q2,
        (States::Q2, Domain::A) => States::Q1,
        (States::Q2, Domain::B) => States::Q2,
    );

    #[test]
    fn test_apply() {
        assert_eq!(
            FSM::new(States::Q1, transform)
                .apply(Domain::A)
                .into_state(),
            States::Q2
        );
        assert_eq!(
            FSM::new(States::Q1, transform)
                .apply(Domain::B)
                .into_state(),
            States::Q2
        );
        assert_eq!(
            FSM::new(States::Q2, transform)
                .apply(Domain::A)
                .into_state(),
            States::Q1
        );
        assert_eq!(
            FSM::new(States::Q2, transform)
                .apply(Domain::B)
                .into_state(),
            States::Q2
        );
    }
}
