fn main() {
    let mut v: Vec<i64> = vec![1, 2, 3, 4, 5];
    let mut s: Vec<&mut i64> = v
        .iter_mut()
        .filter(|val| **val < 2_i64)
        .collect();
    if s.len() == 0 {
        s = v
            .iter_mut()
            .filter(|val| **val > 2_i64)
            .collect();
    }
    *s[0] = 0;
    println!("{:?}", v);
}
