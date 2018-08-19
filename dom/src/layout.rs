use traits::TLayoutNode;
use yoga::{Direction, Node};

#[derive(Debug, PartialEq)]
pub struct LayoutNode {
  node: Node,
}

impl Default for LayoutNode {
  fn default() -> Self {
    Self {
      node: Node::new(),
    }
  }
}

impl TLayoutNode for LayoutNode {
  type ReflowDirection = Direction;

  fn is_tainted(&self) -> bool {
    false
  }

  fn insert_child(&mut self, child: &mut Self, index: u32) {
    self.node.insert_child(&mut child.node, index);
  }

  fn append_child(&mut self, child: &mut Self) {
    let count = self.node.child_count();
    self.insert_child(child, count);
  }

  fn remove_child(&mut self, child: &mut Self) {
    self.node.remove_child(&mut child.node);
  }

  fn reflow_subtree(&mut self, width: u32, height: u32, direction: Self::ReflowDirection) {
    self.node.calculate_layout(width as f32, height as f32, direction);
  }

  fn child_count(&self) -> u32 {
    self.node.child_count()
  }
}
