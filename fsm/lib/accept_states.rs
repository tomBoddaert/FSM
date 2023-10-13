#[allow(clippy::module_name_repetitions)]
/// This trait defines accepted states for state sets for finite state machines.
pub trait AcceptStates {
    #[must_use]
    /// Returns [`true`] if the state is marked as accepted.
    fn is_accepted(&self) -> bool;
}
