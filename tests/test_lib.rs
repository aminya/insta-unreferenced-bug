use insta_bug::something;
use insta::assert_snapshot;

#[test]
fn test_lib() {
    assert_snapshot!(something());
}