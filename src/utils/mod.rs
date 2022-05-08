#[cfg(test)]
mod test;

pub fn assert_len(expect: usize, got: usize) {
    if expect != got {
        panic!("invalid vector length")
    }
}
