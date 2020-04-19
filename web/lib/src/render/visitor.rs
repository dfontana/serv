/**
 * Trait describing something that can visit nodes, and something that can
 * be visited
 */
use super::Node;

pub trait Visitor<T: Clone> {
  fn visit_node(&mut self, n: &Node<T>);
}

pub trait Visitable<T: Clone> {
  fn accept<V: Visitor<T>>(&self, visitor: &mut V);
}