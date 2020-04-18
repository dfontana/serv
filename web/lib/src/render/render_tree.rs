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
  fn accept<V: Visitor<T>>(&self, visitor: &mut V) {
    self.root.accept(visitor);
  }
}

impl<T: Clone> Visitable<T> for Node<T> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V) {
    visitor.visit_node(self);
    // TODO ensure visit order is by position.
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