use super::block::{BlockRenderer, BlockType::{Layer, Text, Solid}, Block};
use super::render::{Node, RenderTree, Visitable};

pub fn run_test() -> String {
  let tree: RenderTree<Block> = Node::builder(Block::new(Layer))
    .add(1, Block::new(Solid))
    .nest(
      2,
      Node::builder(Block::new(Layer))
        .add(1, Block::new(Text))
        .add(2, Block::new(Text))
        .build(),
    )
    .add(3, Block::new(Text))
    .build()
    .into();

  let mut node_list = BlockRenderer::new();
  tree.accept(&mut node_list);
  format!("{:?}", node_list)
}
