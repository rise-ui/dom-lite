use serde::{Serialize, Deserialize};
use types::{DOMNodeRawId, EventType};

use traits::{
  TKeyboardEvent,
  TGenericEvent,
  TMouseEvent,
  TUIEvent,
  TEvent,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BasicEvent {}

impl TGenericEvent for BasicEvent {
}

impl TEvent for BasicEvent {
  fn target(&self) -> DOMNodeRawId {
    unimplemented!()
  }

  fn ty(&self) -> EventType {
    unimplemented!()
  }
}

impl TUIEvent for BasicEvent {
  fn alt_key(&self) -> bool {
    unimplemented!()
  }

  fn ctrl_key(&self) -> bool {
    unimplemented!()
  }

  fn meta_key(&self) -> bool {
    unimplemented!()
  }

  fn shift_key(&self) -> bool {
    unimplemented!()
  }
}

impl TKeyboardEvent for BasicEvent {
  type KeyCode = ();

  fn code(&self) -> Self::KeyCode {
    unimplemented!()
  }

  fn key(&self) -> &'static str {
    unimplemented!()
  }

  fn get_modifier_state(&self) -> bool {
    unimplemented!()
  }

  fn repeat(&self) -> bool {
    unimplemented!()
  }
}

impl TMouseEvent for BasicEvent {
  type MouseButton = ();

  fn button(&self) -> Self::MouseButton {
    unimplemented!()
  }

  fn client_x(&self) -> u32 {
    unimplemented!()
  }

  fn client_y(&self) -> u32 {
    unimplemented!()
  }

  fn offset_x(&self) -> u32 {
    unimplemented!()
  }

  fn offset_y(&self) -> u32 {
    unimplemented!()
  }

  fn page_x(&self) -> u32 {
    unimplemented!()
  }

  fn page_y(&self) -> u32 {
    unimplemented!()
  }
}
