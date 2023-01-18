use magnus::{Integer, RArray};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_vecs() {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RArray = RArray::new();
    input.push(Integer::from_i64(1)).unwrap();
    input.push(Integer::from_i64(2)).unwrap();
    input.push(Integer::from_i64(3)).unwrap();

    let output: Vec<i64> = deserialize(input).unwrap();
    assert_eq!(&[1, 2, 3], &output[..]);
}
