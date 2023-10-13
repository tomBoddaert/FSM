use crate::{AcceptStates, FSM};

#[must_use]
#[inline]
/// Create a new intersection machine.
/// This runs two machines in parallel and is in an accept state only if both machines are.
///
/// This variant clones each input. For a version that copies each input, see [`new_copy`].
pub const fn new<Domain, StatesA, StatesB>(
    start_state_a: StatesA,
    start_state_b: StatesB,
    transform_a: fn(StatesA, Domain) -> StatesA,
    transform_b: fn(StatesB, Domain) -> StatesB,
) -> FSM<Domain, State<Domain, StatesA, StatesB>>
where
    Domain: Clone,
{
    FSM::new(
        State {
            a: FSM::new(start_state_a, transform_a),
            b: FSM::new(start_state_b, transform_b),
        },
        State::apply,
    )
}

#[must_use]
#[inline]
/// Create a new intersection machine.
/// This runs two machines in parallel and is in an accept state only if both machines are.
///
/// This variant copies each input. For a version that clones each input, see [`new`].
pub const fn new_copy<Domain, StatesA, StatesB>(
    start_state_a: StatesA,
    start_state_b: StatesB,
    transform_a: fn(StatesA, Domain) -> StatesA,
    transform_b: fn(StatesB, Domain) -> StatesB,
) -> FSM<Domain, State<Domain, StatesA, StatesB>>
where
    Domain: Copy,
{
    FSM::new(
        State {
            a: FSM::new(start_state_a, transform_a),
            b: FSM::new(start_state_b, transform_b),
        },
        State::apply_copy,
    )
}

#[must_use]
#[inline]
/// Create a new intersection machine with default start states.
/// This runs two machines in parallel and is in an accept state only if both machines are.
///
/// This variant clones each input. For a version that copies each input, see [`default_with_transforms_copy`].
pub fn default_with_transforms<Domain, StatesA: Default, StatesB: Default>(
    transform_a: fn(StatesA, Domain) -> StatesA,
    transform_b: fn(StatesB, Domain) -> StatesB,
) -> FSM<Domain, State<Domain, StatesA, StatesB>>
where
    Domain: Clone,
{
    FSM::new(
        State {
            a: FSM::default_with_transform(transform_a),
            b: FSM::default_with_transform(transform_b),
        },
        State::apply,
    )
}

#[must_use]
#[inline]
/// Create a new intersection machine with default start states.
/// This runs two machines in parallel and is in an accept state only if both machines are.
///
/// This variant copies each input. For a version that clones each input, see [`default_with_transforms`].
pub fn default_with_transforms_copy<Domain, StatesA: Default, StatesB: Default>(
    transform_a: fn(StatesA, Domain) -> StatesA,
    transform_b: fn(StatesB, Domain) -> StatesB,
) -> FSM<Domain, State<Domain, StatesA, StatesB>>
where
    Domain: Copy,
{
    FSM::new(
        State {
            a: FSM::default_with_transform(transform_a),
            b: FSM::default_with_transform(transform_b),
        },
        State::apply_copy,
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The state of an intersection machine.
/// If `StatesA` and `StatesB` implement [`HasAcceptState`], this will be accepted if both machines are in an accept state.
pub struct State<Domain, StatesA, StatesB> {
    a: FSM<Domain, StatesA>,
    b: FSM<Domain, StatesB>,
}

impl<Domain, StatesA, StatesB> State<Domain, StatesA, StatesB> {
    #[must_use]
    #[inline]
    fn apply(self, input: Domain) -> Self
    where
        Domain: Clone,
    {
        Self {
            a: self.a.apply(input.clone()),
            b: self.b.apply(input),
        }
    }

    #[must_use]
    #[inline]
    fn apply_copy(self, input: Domain) -> Self
    where
        Domain: Copy,
    {
        Self {
            a: self.a.apply(input),
            b: self.b.apply(input),
        }
    }
}

impl<Domain, StatesA: AcceptStates, StatesB: AcceptStates> AcceptStates
    for State<Domain, StatesA, StatesB>
{
    #[inline]
    fn is_accepted(&self) -> bool {
        self.a.is_accepted() && self.b.is_accepted()
    }
}
