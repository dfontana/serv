use super::{Visitable, Visitor};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Node<T: Clone> {
  children: BTreeMap<i32, Node<T>>,
  value: T,
}

#[derive(Clone, Debug)]
pub struct NodeBuilder<T: Clone> {
  children: BTreeMap<i32, Node<T>>,
  value: T,
}

impl<T: Clone> Visitable<T> for Node<T> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V) {
    visitor.visit_node(self);
    self.children.values().for_each(|n| n.accept(visitor));
  }
}

impl<T: Clone> Node<T> {
  pub fn new(value: T) -> Node<T> {
    NodeBuilder::new(value).build()
  }

  pub fn builder(value: T) -> NodeBuilder<T> {
    NodeBuilder::new(value)
  }

  pub fn value(&self) -> &T {
    &self.value
  }
}

impl<T: Clone> NodeBuilder<T> {
  fn new(value: T) -> NodeBuilder<T> {
    NodeBuilder {
      children: BTreeMap::new(),
      value,
    }
  }

  pub fn nest(&mut self, pos: i32, c: Node<T>) -> &mut Self {
    if pos <= 0 {
      panic!("Attempted to add invalid child position");
    }
    self.children.insert(pos, c);
    self
  }

  pub fn add(&mut self, pos: i32, c: T) -> &mut Self {
    self.nest(pos, Node::new(c))
  }

  pub fn build(&self) -> Node<T> {
    Node {
      children: self.children.clone(),
      value: self.value.clone(),
    }
  }
}
