


use leetcode;

#[test]
fn it_adds_two() {
    assert_eq!(3, leetcode::add(1, 2));
}

#[test]
fn remove_duplicates_test() {
    let mut arr = vec![0,0,1,1,1,2,2,3,3,4];
    let res = leetcode::remove_duplicates(&mut arr);
    assert_eq!(5, res);
}
