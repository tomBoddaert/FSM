use crate::AcceptStates;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A Finite State Machine.
///
/// This contains a state and a transform function.
pub struct FSM<Domain, States> {
    state: States,
    transform: fn(States, Domain) -> States,
}

impl<Domain, States> FSM<Domain, States> {
    #[must_use]
    #[inline]
    /// Create a new Finite State Machine starting with the `start_state` and with the `transform` function.
    pub const fn new(start_state: States, transform: fn(States, Domain) -> States) -> Self {
        Self {
            state: start_state,
            transform,
        }
    }

    #[must_use]
    #[inline]
    /// Create a new Finite State Machine starting with the default state and with the `transform` function.
    pub fn default_with_transform(transform: fn(States, Domain) -> States) -> Self
    where
        States: Default,
    {
        Self::new(States::default(), transform)
    }

    #[must_use]
    #[inline]
    /// Get the current state.
    pub const fn state(&self) -> &States {
        &self.state
    }

    #[must_use]
    #[inline]
    /// Get the transform function.
    pub const fn transform_function(&self) -> fn(States, Domain) -> States {
        self.transform
    }

    #[must_use]
    #[inline]
    /// Extract the current state.
    pub fn into_state(self) -> States {
        self.state
    }

    #[must_use]
    #[inline]
    /// Returns [`true`] if the current state is marked as accepted.
    pub fn is_accepted(&self) -> bool
    where
        States: AcceptStates,
    {
        self.state.is_accepted()
    }

    #[must_use = "this returns the result of the transformation, without modifying the original"]
    #[inline]
    /// Apply an input to the Finite State Machine, returning the new state of the machine.
    pub fn apply(mut self, input: Domain) -> Self {
        let new_state = (self.transform)(self.state, input);
        self.state = new_state;
        self
    }

    #[inline]
    /// Apply an input to the Finite State Machine in place.
    pub fn apply_assign(&mut self, input: Domain)
    where
        States: Clone,
    {
        let state = self.state.clone();
        let new_state = (self.transform)(state, input);
        self.state = new_state;
    }

    #[must_use = "this returns the result of the transformations, without modifying the original"]
    /// Apply a set of inputs to the Finite State Machine, returning the new state of the machine.
    pub fn run<I>(mut self, inputs: I) -> Self
    where
        I: IntoIterator<Item = Domain>,
    {
        for input in inputs {
            self = self.apply(input);
        }

        self
    }

    /// Apply a set of inputs to the Finite State Machine in place.
    pub fn run_assign<I>(&mut self, inputs: I)
    where
        States: Clone,
        I: IntoIterator<Item = Domain>,
    {
        let mut state = self.state.clone();

        for input in inputs {
            state = (self.transform)(state, input);
        }

        self.state = state;
    }
}
