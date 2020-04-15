use std::collections::HashMap;

pub trait Visitor<T> {
  fn visit_node(&mut self, n: &Node<T>);
}

pub trait Visitable<T> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V);
}

pub struct RenderTree<T> {
  root: Node<T>,
}

pub struct Node<T> {
  parent: Option<T>,
  children: HashMap<i32, Node<T>>,
  value: T,
}

impl<T> RenderTree<T> {
  pub fn new(value: T) -> RenderTree<T> {
    return RenderTree {
      root: Node::new(value),
    };
  }
}

impl<T> Visitable<T> for RenderTree<T> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V) {
    visitor.visit_node(&self.root);
    // TODO go to the children in order, consider BTreeMap if that helps
    self
      .root
      .children
      .values()
      .for_each(|n| visitor.visit_node(&n));
  }
}

impl<T> Node<T> {
  fn new(value: T) -> Node<T> {
    return Node {
      parent: None,
      children: HashMap::new(),
      value,
    };
  }

  pub fn value(&self) -> &T {
    return &self.value;
  }
}
