use super::of;

struct TestStruct {
  val: i32,
}

#[test]
fn has_all_elements() {
  let expected = vec!["a", "b", "c"];
  let test = of(&["a", "b", "c"]);
  // TODO fix this
  assert_eq!(test.to_vec(), expected);
}

#[test]
fn map_produces_new_stream() {
  let expected = vec![2, 4, 6];
  let test = of(&[1, 2, 3]).map(|x| *x * 2);
  assert_eq!(test.to_vec(), expected);
}

#[test]
fn filter_omits_items() {
  let expected = vec![&1, &3];
  let test = of(&[1, 2, 3]).filter(|x| **x != 2);
  let test2 = test.to_vec();
  assert_eq!(test2, expected);
}

#[test]
fn filter_then_map() {
  let expected = vec![2, 6];
  let test = of(&[1, 2, 3]).filter(|x| **x != 2).map(|x| *x * 2);
  assert_eq!(test.to_vec(), expected);
}

#[test]
fn map_then_filter() {
  let expected = vec![4, 6];
  let test = of(&[1, 2, 3]).map(|x| *x * 2).filter(|x| *x != 2);
  assert_eq!(test.to_vec(), expected);
}

#[test]
fn with_non_clonable() {
  // TODO how to get items out of the stream that aren't cloneable?
  let a = TestStruct { val: 123 };
  let b = TestStruct { val: 456 };
  let expected = vec![a, b];
  let test = of(&[a, b]);
  assert_eq!(test.to_vec(), expected);
}
