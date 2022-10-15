pub fn assert_len(expect: usize, got: usize) {
    if expect != got {
        panic!("invalid vector length")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn assert_len_panic() {
        assert_len(1, 0)
    }
}
