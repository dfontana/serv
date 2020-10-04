#[cfg(test)]
#[path = "./stream_test.rs"]
mod stream_test;

use std::sync::Arc;

pub struct Stream<T> {
  data: Vec<Arc<T>>,
}

pub fn of<T: Clone>(args: &[T]) -> Stream<T> {
  let mut vec = Vec::new();
  for d in args.iter() {
    vec.push(Arc::new(d.clone()))
  }
  Stream { data: vec }
}

impl<T> Stream<T> {
  pub fn map<F, K>(&self, f: F) -> Stream<K>
  where
    F: Fn(&T) -> K,
  {
    let mut vec = Vec::new();
    for d in self.data.iter() {
      vec.push(Arc::new(f(d)))
    }
    Stream { data: vec }
  }
  pub fn filter<F>(&self, f: F) -> Stream<T>
  where
    F: Fn(&T) -> bool,
  {
    let mut vec = Vec::new();
    for d in self.data.iter() {
      if f(d) {
        vec.push(d.clone());
      }
    }
    Stream { data: vec }
  }
  pub fn each<F>(&self, mut f: F)
  where
    F: FnMut(&T),
  {
    for d in self.data.iter() {
      f(d)
    }
  }
}
