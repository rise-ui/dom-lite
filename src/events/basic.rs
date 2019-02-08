use serde::{Deserialize, Serialize};
use types::{DOMNodeRawId, EventType};

use traits::{TEvent, TGenericEvent, TKeyboardEvent, TMouseEvent, TUIEvent};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MouseEvent {
    target_mouse_code: u64,

    client_x: u32,
    client_y: u32,
    
    offset_x: u32,
    offset_y: u32,

    page_x: u32,
    page_y: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MouseButton {
    Main = 0, // Main button pressed, usually the left button or the un-initialized state
    Auxiliary = 1, // Auxiliary button pressed, usually the wheel button or the middle button (if present)
    Secondary = 2, // Secondary button pressed, usually the right button
    //Fourth = 3, // Fourth button, typically the Browser Back button
    //Fifth = 4, // Fifth button, typically the Browser Forward button
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KeyboardEvent {
    shift_pressed: bool,
    ctrl_pressed: bool,
    meta_pressed: bool,
    alt_pressed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BasicEvent {
    event_type: EventType,
    node_id: DOMNodeRawId,

    keyboard: KeyboardEvent,
    mouse: MouseEvent,
}

impl TGenericEvent for BasicEvent {}

impl TEvent for BasicEvent {
    fn target(&self) -> DOMNodeRawId {
        self.node_id
    }

    fn ty(&self) -> EventType {
        self.event_type
    }
}

impl TUIEvent for BasicEvent {
    fn alt_key(&self) -> bool {
        self.keyboard.alt_pressed
    }

    fn ctrl_key(&self) -> bool {
        self.keyboard.ctrl_pressed
    }

    fn meta_key(&self) -> bool {
        self.keyboard.meta_pressed
    }

    fn shift_key(&self) -> bool {
        self.keyboard.shift_pressed
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
    type MouseButton = MouseButton;

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
