use deep_cli::add;

#[test]
fn integration_add() {
    assert_eq!(add(3, 4), 7);
}
