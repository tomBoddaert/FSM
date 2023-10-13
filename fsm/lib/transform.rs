#[macro_export]
#[allow(clippy::module_name_repetitions)]
/// A macro to define a Finite State Machine's transformation function with a match-like syntax.
///
/// # Syntax
/// ```text
/// DefineTransform!([attributes] <name>, <states>, <input domain>,
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
/// fsm::DefineTransform!(tristate, SomeStates, SomeInputs,
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
/// There are more examples in the examples directory.
macro_rules! DefineTransform {
    ( $(#[$attr:meta])* $id:ident, $states:path, $dom:path, $($matcher:pat $(if $test:expr)? => $result:expr),* $(,)? ) => {
        $(#[$attr])*
        #[allow(clippy::missing_const_for_fn)]
        fn $id(state: $states, input: $dom) -> $states {
            match (state, input) {
                $($matcher $(if $test)? => $result),*
            }
        }
    };
}
