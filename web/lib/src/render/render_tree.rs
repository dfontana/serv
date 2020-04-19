use super::{Node, Visitable, Visitor};

#[derive(Clone, Debug)]
pub struct RenderTree<T: Clone> {
  root: Node<T>,
}

impl<T: Clone> Visitable<T> for RenderTree<T> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V) {
    self.root.accept(visitor);
  }
}

impl<T: Clone> From<Node<T>> for RenderTree<T> {
  fn from(node: Node<T>) -> RenderTree<T> {
    RenderTree {
      root: node
    }
  }
} 
