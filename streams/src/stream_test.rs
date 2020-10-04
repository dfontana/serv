use super::{of, Stream};

// TODO Stream.collect()
fn collect_stream<T: Copy>(s: Stream<T>) -> Vec<T> {
  let mut out = Vec::new();
  s.each(|item| {
    out.push(*item);
  });
  return out;
}

#[test]
fn has_all_elements() {
  let expected = vec!["a", "b", "c"];
  let test = of(&["a", "b", "c"]);
  assert_eq!(collect_stream(test), expected);
}

#[test]
fn map_produces_new_stream() {
  let expected = vec![2, 4, 6];
  let test = of(&[1, 2, 3]).map(|x| x * 2);
  assert_eq!(collect_stream(test), expected);
}

#[test]
fn filter_omits_items() {
  let expected = vec![1, 3];
  let test = of(&[1, 2, 3]).filter(|x| *x != 2);
  assert_eq!(collect_stream(test), expected);
}

#[test]
fn filter_then_map() {
  let expected = vec![2, 6];
  let test = of(&[1, 2, 3]).filter(|x| *x != 2).map(|x| x * 2);
  assert_eq!(collect_stream(test), expected);
}

#[test]
fn map_then_filter() {
  let expected = vec![4, 6];
  let test = of(&[1, 2, 3]).map(|x| x * 2).filter(|x| *x != 2);
  assert_eq!(collect_stream(test), expected);
}
