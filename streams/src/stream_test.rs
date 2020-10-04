use super::of;

#[test]
fn build_and_iter() {
  let test = of(&["a", "b", "c"]);
  let expected = vec!["a", "b", "c"];
  let mut actual = Vec::new();
  test.each(|item| {
    actual.push(*item);
  });
  assert_eq!(actual, expected);
}