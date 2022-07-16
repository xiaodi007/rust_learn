pub fn sum(values: &[u32]) -> Option<u32> {
    if values.len() == 0 {
        return None
    }
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    return Some(res)
}
