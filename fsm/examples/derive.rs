use fsm::AcceptStates;

#[derive(AcceptStates)]
enum States {
    Q0,
    #[accept]
    Q1,
    #[accept]
    Q2,
}

fn main() {
    assert!(!States::Q0.is_accepted());
    assert!(States::Q1.is_accepted());
    assert!(States::Q2.is_accepted());
}

#[test]
fn test() {
    main()
}
