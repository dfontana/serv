use super::super::render::{Node, Visitor};
use super::Block;

#[derive(Debug)]
pub struct BlockRenderer {
  blocks: Vec<String>,
}

impl BlockRenderer {
  pub fn new() -> BlockRenderer {
    BlockRenderer { blocks: Vec::new() }
  }
}

impl Visitor<Block> for BlockRenderer {
  fn visit_node(&mut self, n: &Node<Block>) {
    self.blocks.push(n.value().name())
  }
}
