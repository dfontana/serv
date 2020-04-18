#[derive(Clone, Debug)]
pub enum Block {
  Layer,
  Solid,
  Image,
  Text,
  Empty,
}

impl Block {
  pub fn name(&self) -> String {
    match self {
      Block::Layer => "layer".to_string(),
      Block::Solid => "solid".to_string(),
      Block::Image => "image".to_string(),
      Block::Text => "text".to_string(),
      Block::Empty => "empty".to_string(),
    }
  }
}
