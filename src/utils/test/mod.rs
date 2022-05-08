use crate::utils::assert_len;

#[test]
#[should_panic]
fn assert_len_panic() {
    assert_len(1, 0)
}
