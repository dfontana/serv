use regex::Regex;

#[derive(Clone, Debug)]
pub enum BlockType {
  Layer,
  Solid,
  Image,
  Text,
  Empty,
}

// TODO finish adding properties to blocks
#[derive(Clone, Debug)]
pub struct Block {
  size: Size,
  color: Hex,
  b_type: BlockType,
}

#[derive(Clone, Debug)]
struct Size {
  width: i32,
  height: i32,
}

#[derive(Clone, Debug)]
struct Hex {
  val: String,
}

impl BlockType {
  pub fn name(&self) -> String {
    match self {
      BlockType::Layer => "layer".to_string(),
      BlockType::Solid => "solid".to_string(),
      BlockType::Image => "image".to_string(),
      BlockType::Text => "text".to_string(),
      BlockType::Empty => "empty".to_string(),
    }
  }
}

impl Block {
  // TODO should get a builder for this
  pub fn new(b_type: BlockType) -> Block {
    Block {
      size: Size {
        width: 0,
        height: 0,
      },
      color: Hex::new("#000").unwrap(),
      b_type,
    }
  }

  pub fn b_type(&self) -> &BlockType {
    &self.b_type
  }
}

impl Hex {
  pub fn new(val: &str) -> Option<Hex> {
    lazy_static! {
      // TODO define this; should be #RGB, #RRGGBB, #RGBA, #RRGGBBAA
      static ref RE: Regex = Regex::new(r"").unwrap();
    }
    if RE.is_match(val) {
      Some(Hex {
        val: val.to_string(),
      })
    } else {
      None
    }
  }
}
