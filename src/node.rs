/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use jss::traits::TStyleContext;
use jss::types::{DimensionType, Style};

use hashbrown::{HashMap, hash_map::{ Drain }};
use std::borrow::{Borrow, Cow};
use rsx_tree::types::Id;
use layout::LayoutNode;
use std::cmp::Ordering;
use yoga::Direction;
use tree::DOMTree;
use std::rc::Rc;

use jss::traits::TStyleCollect;
use traits::{TDOMNode, TDOMText, TGenericEvent, TLayoutNode};
use types::{Closure, EventType, KnownAttributeName, KnownElementName, Prop};
use util::is_event_listener;

pub type DOMNodeId<T> = Id<DOMNode<T>>;
pub type DOMNodeIdPair<T> = (DOMNodeId<T>, DOMNodeId<T>);

pub type DOMNodeSiblingIds<T> = (Option<DOMNodeId<T>>, Option<DOMNodeId<T>>);
pub type DOMNodeEdgeIds<T> = (Option<DOMNodeId<T>>, Option<DOMNodeId<T>>);
// pub type DOMAttributes<T> = Vec<DOMAttribute<T>>;
pub type DOMChildren<T> = Vec<DOMNodeId<T>>;

#[derive(Debug, PartialEq)]
pub struct DOMNode<T>
where
    T: TGenericEvent,
{
    pub layout_node: LayoutNode,
    pub data: DOMData<T>,
    pub styles: Style,
}

#[derive(Debug, PartialEq)]
pub enum DOMData<T>
where
    T: TGenericEvent,
{
    Normal(DOMNormalNode<T>),
    ShadowHost(DOMTree<T>),
    Text(DOMTextNode),
    Void,
}

#[derive(Debug, Eq, Ord, Clone, Serialize, Deserialize)]
pub enum DOMText {
    Static(Cow<'static, str>),
    Owned(Rc<String>),
}

#[derive(Debug, PartialEq)]
pub struct DOMTextNode {
    pub content: DOMText,
}

#[derive(Debug, PartialEq)]
pub struct DOMNormalNode<T>
where
    T: TGenericEvent,
{
    pub attributes: DOMAttributes<T>,
    pub tag: DOMTagName,
}

#[derive(Debug, PartialEq)]
pub enum DOMTagName {
    NamedspacedName(&'static str, &'static str),
    KnownName(KnownElementName),
    Simple(&'static str),
}

#[derive(Debug, PartialEq)]
pub struct DOMAttribute<T: TGenericEvent>(pub DOMAttributeName, pub DOMAttributeValue<T>);

#[derive(Debug, PartialEq)]
pub struct DOMAttributes<T: TGenericEvent> {
    pub namespaced: HashMap<(&'static str, &'static str), DOMAttributeValue<T>>,
    pub common: HashMap<KnownAttributeName, DOMAttributeValue<T>>,
    pub simple: HashMap<&'static str, DOMAttributeValue<T>>,
    pub listeners: HashMap<EventType, Closure<T>>,
}

#[derive(Debug, PartialEq)]
pub enum DOMAttributeName {
    NamedspacedName(&'static str, &'static str),
    KnownName(KnownAttributeName),
    EventType(EventType),
    Simple(&'static str),
}

#[derive(Debug, PartialEq)]
pub enum DOMAttributeValue<T>
where
    T: TGenericEvent,
{
    Boolean(bool),
    Number(f64),
    Char(char),
    Str(DOMText),
    Styles(Style),
    Prop(Prop),
    EventListener(Closure<T>),
    Node(DOMNode<T>),
}

impl<T> Default for DOMNode<T>
where
    T: TGenericEvent,
{
    fn default() -> Self {
        DOMNode::from(DOMTagName::from(KnownElementName::Fragment))
    }
}

impl<T> DOMNode<T>
where
    T: TGenericEvent,
{
    pub fn new(data: DOMData<T>) -> Self {
        let layout_node = LayoutNode::default();
        let styles = Style::default();

        DOMNode {
            styles,
            layout_node,
            data,
        }
    }

    pub fn shadow_dom(self) -> DOMTree<T> {
        match self.data {
            DOMData::ShadowHost(tree) => tree,
            DOMData::Void | DOMData::Text(_) | DOMData::Normal(_) => DOMTree::default(),
        }
    }
}

impl<T> TDOMNode for DOMNode<T>
where
    T: TGenericEvent,
{
    type LayoutNode = LayoutNode;
    type Id = DOMNodeId<T>;
    type Data = DOMData<T>;
    type Event = T;

    fn data(&self) -> &DOMData<T> {
        &self.data
    }

    fn is_void(&self) -> bool {
        self.data.is_void()
    }

    fn is_shadow_host(&self) -> bool {
        self.data.is_shadow_host().is_some()
    }

    fn is_text(&self) -> bool {
        self.data.is_text().is_some()
    }

    fn is_normal(&self) -> bool {
        self.data.is_normal().is_some()
    }

    fn is_known(&self, name: KnownElementName) -> bool {
        self.data.is_known(name).is_some()
    }

    fn layout_node(&self) -> &LayoutNode {
        &self.layout_node
    }

    fn reflow_subtree(&mut self, width: u32, height: u32, direction: Direction) {
        self.layout_node.reflow_subtree(width, height, direction);
    }
}

impl<T> DOMNode<T>
where
    T: TGenericEvent,
{
    pub fn apply_measurement_metadata_to_layout(&mut self) {
        // use self::KnownElementName::*;

        // preset dimensions for calculate
        let layout = self.layout_node.get_layout();

        // Set current node dimensions to style context
        self.styles
            .context
            .set_dimension(DimensionType::Parent, Some(layout.clone()));

        let layout_node = &mut self.layout_node;
        self.styles.calculate_layout();
        self.styles.calculate_appearance();

        // println!("Computed: {:?}", &self.styles.computed.layout);
        layout_node.node.apply_styles(&self.styles.computed.layout);
    }

    pub fn append_to_layout_node(&mut self, parent: &mut DOMNode<T>) {
        let parent = &mut parent.layout_node;
        let child = &mut self.layout_node;
        let count = parent.child_count();

        parent.insert_child(child, count);
    }

    pub fn remove_from_layout_node(&mut self, parent: &mut DOMNode<T>) {
        let parent = &mut parent.layout_node;
        let child = &mut self.layout_node;
        parent.remove_child(child);
    }
}

impl TDOMText for DOMText {}

impl PartialEq for DOMText {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl PartialOrd for DOMText {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Borrow<str> for DOMText {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for DOMText {
    fn as_ref(&self) -> &str {
        match self {
            &DOMText::Static(ref v) => v.as_ref(),
            &DOMText::Owned(ref v) => v.as_ref(),
        }
    }
}

#[allow(dead_code)]
impl<T> DOMData<T>
where
    T: TGenericEvent,
{
    pub fn text(&self) -> Option<&DOMText> {
        match self {
            &DOMData::Text(DOMTextNode { ref content }) => Some(content),
            &DOMData::Void | &DOMData::ShadowHost(_) | &DOMData::Normal(_) => None,
        }
    }

    pub fn tag(&self) -> Option<&DOMTagName> {
        match self {
            &DOMData::Void | &DOMData::ShadowHost(_) | &DOMData::Text(_) => None,
            &DOMData::Normal(DOMNormalNode { ref tag, .. }) => Some(tag),
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            &DOMData::Void => true,
            _ => false,
        }
    }

    pub fn is_shadow_host(&self) -> Option<&DOMTree<T>> {
        match self {
            &DOMData::ShadowHost(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn is_text(&self) -> Option<&DOMTextNode> {
        match self {
            &DOMData::Text(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn is_normal(&self) -> Option<&DOMNormalNode<T>> {
        match self {
            &DOMData::Normal(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn is_known(&self, name: KnownElementName) -> Option<&DOMNormalNode<T>> {
        match self {
            &DOMData::Normal(ref value) if value.tag == DOMTagName::KnownName(name) => Some(value),
            _ => None,
        }
    }

    pub fn drop_event_listeners<'a>(&mut self) -> Option<Drain<EventType, Closure<T>>> {
        match self {
            &mut DOMData::Void | &mut DOMData::ShadowHost(_) | &mut DOMData::Text(_) => None,
            &mut DOMData::Normal(DOMNormalNode { ref mut attributes, .. }) => {
                let cleaned = attributes.listeners.drain();
                Some(cleaned)
            }
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn attributes_ref(&self) -> Option<&DOMAttributes<T>> {
        match self {
            &DOMData::Void | &DOMData::ShadowHost(_) | &DOMData::Text(_) => None,
            &DOMData::Normal(DOMNormalNode { ref attributes, .. }) => Some(attributes)
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn attributes_mut(&mut self) -> Option<&mut DOMAttributes<T>> {
        match self {
            &mut DOMData::Void | &mut DOMData::ShadowHost(_) | &mut DOMData::Text(_) => None,
            &mut DOMData::Normal(DOMNormalNode { ref mut attributes, .. }) => Some(attributes)
        }
    }
}
