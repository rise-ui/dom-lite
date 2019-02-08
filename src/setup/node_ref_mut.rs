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

use rsx_tree::types::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use node::{DOMNode, DOMNodeEdgeIds, DOMNodeId, DOMNodeIdPair, DOMNodeSiblingIds};
use setup::{DOMArenaRef, DOMArenaRefMutPair};
use traits::TGenericEvent;
use tree::DOMTree;

#[derive(Debug, PartialEq)]
pub struct DOMArenaRefMut<'a, T: 'a>
where
    T: TGenericEvent,
{
    pub raw: RefMut<'a, DOMNode<T>>,
}

impl<'a, T> From<RefMut<'a, DOMNode<T>>> for DOMArenaRefMut<'a, T>
where
    T: TGenericEvent,
{
    fn from(raw: RefMut<'a, DOMNode<T>>) -> Self {
        DOMArenaRefMut { raw }
    }
}

impl<'a, T> Into<DOMArenaRef<'a, T>> for DOMArenaRefMut<'a, T>
where
    T: TGenericEvent,
{
    fn into(self) -> DOMArenaRef<'a, T> {
        DOMArenaRef::from(Into::<Ref<DOMNode<T>>>::into(self.raw))
    }
}

impl<'a, T> Deref for DOMArenaRefMut<'a, T>
where
    T: TGenericEvent,
{
    type Target = DOMNode<T>;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<'a, T> DerefMut for DOMArenaRefMut<'a, T>
where
    T: TGenericEvent,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}

impl<'a, T> DOMArenaRefMut<'a, T>
where
    T: TGenericEvent,
{
    pub fn value(&self) -> &DOMNode<T> {
        self.raw.try_value().expect("Node deallocated")
    }

    pub fn value_mut(&mut self) -> &mut DOMNode<T> {
        self.raw.try_value_mut().expect("Node deallocated")
    }

    pub fn into_value(self) -> &'a mut DOMNode<T> {
        self.raw.try_into_value().expect("Node deallocated")
    }

    pub fn get(&mut self, id: DOMNodeId<T>) -> DOMArenaRef<T> {
        DOMArenaRef::from(self.raw.tree().get(id))
    }

    pub fn get_mut(&mut self, id: DOMNodeId<T>) -> DOMArenaRefMut<T> {
        DOMArenaRefMut::from(self.raw.tree_mut().get_mut(id))
    }

    pub fn get_mut_pair(&mut self, ids: DOMNodeIdPair<T>) -> DOMArenaRefMutPair<T> {
        DOMArenaRefMutPair::from(self.raw.tree_mut().get_mut_pair(ids))
    }

    pub fn get_mut_self_and(&mut self, id: DOMNodeId<T>) -> DOMArenaRefMutPair<T> {
        let ids = (self.id(), id);
        DOMArenaRefMutPair::from(self.raw.tree_mut().get_mut_pair(ids))
    }

    pub fn id(&self) -> DOMNodeId<T> {
        self.raw.id()
    }

    pub fn parent_id(&self) -> Option<DOMNodeId<T>> {
        self.raw.parent_id()
    }

    pub fn parent(&mut self) -> Option<DOMArenaRefMut<T>> {
        self.raw.parent().map(DOMArenaRefMut::from)
    }

    pub fn prev_sibling_id(&self) -> Option<DOMNodeId<T>> {
        self.raw.prev_sibling_id()
    }

    pub fn prev_sibling(&mut self) -> Option<DOMArenaRefMut<T>> {
        self.raw.prev_sibling().map(DOMArenaRefMut::from)
    }

    pub fn next_sibling_id(&self) -> Option<DOMNodeId<T>> {
        self.raw.next_sibling_id()
    }

    pub fn next_sibling(&mut self) -> Option<DOMArenaRefMut<T>> {
        self.raw.next_sibling().map(DOMArenaRefMut::from)
    }

    pub fn first_child_id(&self) -> Option<DOMNodeId<T>> {
        self.raw.first_child_id()
    }

    pub fn first_child(&mut self) -> Option<DOMArenaRefMut<T>> {
        self.raw.first_child().map(DOMArenaRefMut::from)
    }

    pub fn last_child_id(&self) -> Option<DOMNodeId<T>> {
        self.raw.last_child_id()
    }

    pub fn last_child(&mut self) -> Option<DOMArenaRefMut<T>> {
        self.raw.last_child().map(DOMArenaRefMut::from)
    }

    pub fn sibling_ids(&self) -> DOMNodeSiblingIds<T> {
        self.raw.sibling_ids()
    }

    pub fn edge_ids(&self) -> DOMNodeEdgeIds<T> {
        self.raw.edge_ids()
    }

    pub fn append_tree(&mut self, other: DOMTree<T>) -> bool {
        self.raw.append_tree(other.into_inner())
    }

    pub fn prepend_tree(&mut self, other: DOMTree<T>) -> bool {
        self.raw.prepend_tree(other.into_inner())
    }

    pub fn append(&mut self, node: DOMNode<T>) -> DOMArenaRefMut<T> {
        DOMArenaRefMut::from(self.raw.append(node))
    }

    pub fn prepend(&mut self, node: DOMNode<T>) -> DOMArenaRefMut<T> {
        DOMArenaRefMut::from(self.raw.prepend(node))
    }

    pub fn detach(&mut self) {
        self.raw.detach();
    }
}

impl<'a, T> DOMArenaRefMut<'a, T>
where
    T: TGenericEvent,
{
    pub fn append_with_layout(&mut self, child_id: DOMNodeId<T>) -> Result<(), ()> {
        self.raw.append_id(child_id);

        let (this_node, child_node) = self.get_mut_self_and(child_id).into_values();
        child_node.apply_measurement_metadata_to_layout();
        child_node.append_to_layout_node(this_node);

        Ok(())
    }

    pub fn remove_with_layout(&mut self, child_id: DOMNodeId<T>) -> Result<(), ()> {
        self.raw.tree_mut().get_mut(child_id).detach();

        let (this_node, child_node) = self.get_mut_self_and(child_id).into_values();
        child_node.remove_from_layout_node(this_node);

        Ok(())
    }

    pub fn build_layout(&mut self) {
        // @todo: adding set dimensions variable for style before calculate
        self.apply_measurement_metadata_to_layout();

        let mut next_child_id = self.first_child_id();
        while let Some(child_id) = next_child_id {
            {
                let mut child_ref = self.get_mut(child_id);
                child_ref.build_layout();
            }

            {
                let (this_node, child_node) = self.get_mut_self_and(child_id).into_values();
                child_node.append_to_layout_node(this_node);
            }

            next_child_id = self.get(child_id).next_sibling_id();
        }
    }

    pub fn calculate_styles(&mut self) {
        // @todo: adding set dimensions variable for style before calculate
        self.apply_measurement_metadata_to_layout();

        let mut next_child_id = self.first_child_id();
        while let Some(child_id) = next_child_id {
            {
                let mut child_ref = self.get_mut(child_id);
                child_ref.calculate_styles();
            }

            next_child_id = self.get(child_id).next_sibling_id();
        }
    }
}
