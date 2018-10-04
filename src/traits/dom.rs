use std::fmt::Debug;

use serde::{Deserialize as Des, Serialize as Ser};

use traits::{TGenericEvent, TLayoutNode};
use types::KnownElementName;

#[fundamental]
pub trait TDOMText: Debug + Ord + Clone + AsRef<str> + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TDOMTree: Debug + PartialEq {
    type Node: TDOMNode;

    fn get_node(&self, <Self::Node as TDOMNode>::Id) -> &Self::Node;

    fn get_node_mut(&mut self, <Self::Node as TDOMNode>::Id) -> &mut Self::Node;

    fn get_node_mut_pair(
        &mut self,
        (<Self::Node as TDOMNode>::Id, <Self::Node as TDOMNode>::Id),
    ) -> (&mut Self::Node, &mut Self::Node);
}

#[fundamental]
pub trait TDOMNode: Debug + PartialEq {
    type Id: Debug + PartialEq;
    type Data: Debug + PartialEq;
    type Event: TGenericEvent;
    type LayoutNode: TLayoutNode;

    fn data(&self) -> &Self::Data;

    fn is_void(&self) -> bool;

    fn is_shadow_host(&self) -> bool;

    fn is_text(&self) -> bool;

    fn is_normal(&self) -> bool;

    fn is_known(&self, KnownElementName) -> bool;

    fn layout_node(&self) -> &Self::LayoutNode;

    fn reflow_subtree(&mut self, u32, u32, <Self::LayoutNode as TLayoutNode>::ReflowDirection);
}
