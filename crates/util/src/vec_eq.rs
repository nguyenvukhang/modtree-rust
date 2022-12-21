/// Compares vectors up to and including length of 200.
pub fn vec_eq<T, F: Fn(&T, &T) -> bool>(
    a: &Vec<T>,
    b: &Vec<T>,
    cmp: F,
) -> bool {
    if a.len() > 200 || b.len() > 200 || a.len() != b.len() {
        return false;
    }
    let (a, b, len) = (a.as_slice(), b.as_slice(), a.len());
    let (mut ok_a, mut ok_b) = ([false; 200], [false; 200]);
    for idx_a in 0..len {
        if ok_a[idx_a] {
            continue;
        }
        for idx_b in 0..len {
            if ok_b[idx_b] {
                continue;
            }
            if cmp(&a[idx_a], &b[idx_b]) {
                ok_a[idx_a] = true;
                ok_b[idx_b] = true;
            }
        }
    }
    (0..len).fold(true, |a, x| a && ok_a[x] && ok_b[x])
}

#[test]
fn test_vec_eq() {
    macro_rules! test {
        ($a:expr, $b:expr, $expected:expr) => {
            let [a, b]: [Vec<u8>; 2] = [$a, $b];
            assert!(!$expected ^ vec_eq(&a, &b, |a, b| a.eq(b)));
        };
    }
    test!(vec![1, 2, 3, 4], vec![4, 3, 2, 1], true);
    test!(vec![2, 3, 4], vec![4, 3, 1], false);
    test!(vec![2, 3, 4], vec![4, 3, 1], false);
    test!(vec![], vec![1], false);
}
