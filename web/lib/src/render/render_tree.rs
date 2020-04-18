use std::collections::HashMap;

// TODO clean up the packaging this feels weird

pub trait Visitor<T: Clone> {
  fn visit_node(&mut self, n: &Node<T>);
}

pub trait Visitable<T: Clone> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V);
}

#[derive(Clone, Debug)]
pub struct RenderTree<T: Clone> {
  root: Node<T>,
}


#[derive(Clone, Debug)]
pub struct Node<T: Clone> {
  children: HashMap<i32, Node<T>>,
  value: T,
}

#[derive(Clone, Debug)]
pub struct NodeBuilder<T: Clone> {
  children: HashMap<i32, Node<T>>,
  value: T,
}

impl<T: Clone> Visitable<T> for RenderTree<T> {
  // TODO this is wrong, there's no recurse step since renderTree is only at the top level
  //      when we call visit on each node, it won't go to the child note. We should make the
  //      not Visitable, so it can properly dispatch
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

impl<T: Clone> From<Node<T>> for RenderTree<T> {
  fn from(node: Node<T>) -> RenderTree<T> {
    RenderTree {
      root: node
    }
  }
} 

impl<T: Clone> NodeBuilder<T> {
  fn new(value: T) -> NodeBuilder<T> {
    NodeBuilder {
      children: HashMap::new(),
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