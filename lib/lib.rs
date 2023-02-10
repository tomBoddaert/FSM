//! FSM is a simple crate for defining Finite State Machines
//!
//! # Examples
//! A tristate FSM, with `Prev` and `Next` inputs, that defaults
//! to the first state and does not wrap.
//! ```
//! use fsm::{self, FSM};
//!
//! #[derive(Default)]
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
//! // Define the Finite State Machine
//! fsm::MakeFSM!(Tristate, default States, Inputs,
//!     (S0, Prev) => S0,
//!     (S0, Next) => S1,
//!     (S1, Prev) => S0,
//!     (S1, Next) => S2,
//!     (S2, Prev) => S1,
//!     (S2, Next) => S2,
//! );
//!
//! // Test increasing
//! let mut machine = Tristate::default();
//! assert!(matches!(machine.state(), S0));
//!
//! machine.transform(&Next);
//! assert!(matches!(machine.state(), S1));
//!
//! machine.transform(&Next);
//! assert!(matches!(machine.state(), S2));
//!
//! machine.transform(&Next);
//! assert!(matches!(machine.state(), S2));
//!
//! // Test decreasing
//! let mut machine = Tristate::new(S2);
//! assert!(matches!(machine.state(), S2));
//!
//! machine.transform(&Prev);
//! assert!(matches!(machine.state(), S1));
//!
//! machine.transform(&Prev);
//! assert!(matches!(machine.state(), S0));
//!
//! machine.transform(&Prev);
//! assert!(matches!(machine.state(), S0));
//! ``` 

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]

#![cfg_attr(not(feature = "std"), no_std)]

use core::iter::Iterator;

/// The description of a Finite State Machine.
/// This can be implemented with the `MakeFSM` macro.
pub trait FSM {
    /// The input
    type Domain;
    type States;

    /// Create a new FSM
    fn new(state: Self::States) -> Self;
    /// Get a reference to the current state
    fn state(&self) -> &Self::States;
    /// Give an input to the FSM
    fn transform(&mut self, input: &Self::Domain);

    fn run<I>(&mut self, inputs: I)
    where
        I: Iterator<Item = Self::Domain>,
    {
        for input in inputs {
            self.transform(&input);
        }
    }
}

#[macro_export]
/// A macro to create a Finite State Machine and implement the `FSM` trait on it.
/// 
/// # Syntax
/// ```text
/// MakeFSM!(<name>, [default] <states>, <input domain>,
///     <match-like case statements, that take (<current state>, <input>), and return the new state>,
///     (<current>, <input>) => <new>,
/// );
/// ```
///
/// # Examples
///
/// This example has three numbered states and `Back`, `Stay`,
/// and `Forward` inputs. The states do not wrap.
/// ```
/// use fsm;
///
/// enum SomeInputs { Back, Stay, Forward };
/// enum SomeStates { S0, S1, S2 };
///
/// fsm::MakeFSM!(Tristate, SomeStates, SomeInputs,
///     // For state 0:
///     // If given, forward, increase state
///     (SomeStates::S0, SomeInputs::Forward) => SomeStates::S1,
///     // otherwise, stay with this state
///     (SomeStates::S0, _) => SomeStates::S0,
///     // For state 1:
///     // If given, back, decrease state
///     (SomeStates::S1, SomeInputs::Back) => SomeStates::S0,
///     // If given, stay, do not change state
///     (SomeStates::S1, SomeInputs::Stay) => SomeStates::S1,
///     // If given, forward, increase state
///     (SomeStates::S1, SomeInputs::Forward) => SomeStates::S2,
///     // For state 2:
///     // If given, back, decrease state
///     (SomeStates::S2, SomeInputs::Back) => SomeStates::S1,
///     // otherwise, stay with this state
///     (SomeStates::S2, _) => SomeStates::S2
/// );
/// ```
///
/// ## With a default state
/// ```
/// use fsm;
///
/// enum SomeInputs { A, B };
/// #[derive(Default)]
/// enum SomeStates {
///     #[default]
///     SA,
///     SB,
/// }
///
/// // Add the default keyword before the states
/// fsm::MakeFSM!(Bistate, default SomeStates, SomeInputs,
///     // If given A, switch to state SA
///     (_, SomeInputs::A) => SomeStates::SA,
///     // If given B, switch to state SB
///     (_, SomeInputs::B) => SomeStates::SB
/// );
/// ```
/// 
/// Look at the examples directory for examples of how to use your new Finite State Machine!
macro_rules! MakeFSM {
    ( $id:ident, $states:path, $dom:path, $($matcher:pat => $result:expr),* $(,)* ) => {
        struct $id ($states);

        impl $crate::FSM for $id {
            type Domain = $dom;
            type States = $states;

            fn new(state: Self::States) -> Self {
                Self(state)
            }

            fn state(&self) -> &Self::States {
                &self.0
            }

            fn transform(&mut self, input: &Self::Domain) {
                self.0 = match (&self.0, input) {
                    $($matcher => $result),*
                };
            }
        }
    };
    ( $id:ident, default $states:path, $dom:path, $($matcher:pat => $result:expr),* $(,)* ) => {
        $crate::MakeFSM!($id, $states, $dom, $($matcher => $result),*);

        impl Default for $id
        where <$id as $crate::FSM>::States: core::default::Default {
            fn default() -> Self {
                Self(<$id as $crate::FSM>::States::default())
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    // Define the input range
    enum Sigma {
        A,
        B,
    }

    // Define the states with default (optional)
    #[derive(Default)]
    enum Q {
        #[default]
        Q0,
        Q1,
        Q2,
    }

    // Implement States for Q
    MakeFSM!(No2As, default Q, Sigma,
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

    #[test]
    fn test() {
        let tests = vec![
            (Q::Q0, vec![]),
            (Q::Q1, vec![Sigma::A]),
            (Q::Q2, vec![Sigma::A, Sigma::A]),
            (Q::Q2, vec![Sigma::A, Sigma::A, Sigma::B]),
            (Q::Q0, vec![Sigma::A, Sigma::B]),
            (Q::Q1, vec![Sigma::A, Sigma::B, Sigma::A]),
            (Q::Q2, vec![Sigma::A, Sigma::B, Sigma::A, Sigma::A]),
        ];

        #[allow(unused_variables)]
        for (fs, inputs) in tests {
            // Create a FSM using the default state
            let mut machine = No2As::default();
            // Run the inputs
            machine.run(inputs.into_iter());

            assert!(matches!(machine.state(), fs));
        }
    }
}
