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

use rsx_tree::types::Ref;
use std::ops::Deref;

use node::{DOMNode, DOMNodeEdgeIds, DOMNodeId, DOMNodeSiblingIds};
use traits::TGenericEvent;

#[derive(Debug, PartialEq)]
pub struct DOMArenaRef<'a, T: 'a>
where
  T: TGenericEvent,
{
  raw: Ref<'a, DOMNode<T>>,
}

impl<'a, T> From<Ref<'a, DOMNode<T>>> for DOMArenaRef<'a, T>
where
  T: TGenericEvent,
{
  fn from(raw: Ref<'a, DOMNode<T>>) -> Self {
    DOMArenaRef {
      raw,
    }
  }
}

impl<'a, T> Deref for DOMArenaRef<'a, T>
where
  T: TGenericEvent,
{
  type Target = DOMNode<T>;

  fn deref(&self) -> &Self::Target {
    self.value()
  }
}

impl<'a, T> DOMArenaRef<'a, T>
where
  T: TGenericEvent,
{
  pub(crate) fn value(&self) -> &'a DOMNode<T> {
    self.raw.try_value().expect("Node deallocated")
  }

  pub(crate) fn into_value(self) -> &'a DOMNode<T> {
    self.raw.try_into_value().expect("Node deallocated")
  }

  pub fn get(&self, id: DOMNodeId<T>) -> DOMArenaRef<'a, T> {
    DOMArenaRef::from(self.raw.tree().get(id))
  }

  pub fn id(&self) -> DOMNodeId<T> {
    self.raw.id()
  }

  pub fn parent_id(&self) -> Option<DOMNodeId<T>> {
    self.raw.parent_id()
  }

  pub fn parent(&self) -> Option<DOMArenaRef<'a, T>> {
    self.raw.parent().map(DOMArenaRef::from)
  }

  pub fn prev_sibling_id(&self) -> Option<DOMNodeId<T>> {
    self.raw.prev_sibling_id()
  }

  pub fn prev_sibling(&self) -> Option<DOMArenaRef<'a, T>> {
    self.raw.prev_sibling().map(DOMArenaRef::from)
  }

  pub fn next_sibling_id(&self) -> Option<DOMNodeId<T>> {
    self.raw.next_sibling_id()
  }

  pub fn next_sibling(&self) -> Option<DOMArenaRef<'a, T>> {
    self.raw.next_sibling().map(DOMArenaRef::from)
  }

  pub fn first_child_id(&self) -> Option<DOMNodeId<T>> {
    self.raw.first_child_id()
  }

  pub fn first_child(&self) -> Option<DOMArenaRef<'a, T>> {
    self.raw.first_child().map(DOMArenaRef::from)
  }

  pub fn last_child_id(&self) -> Option<DOMNodeId<T>> {
    self.raw.last_child_id()
  }

  pub fn last_child(&self) -> Option<DOMArenaRef<'a, T>> {
    self.raw.last_child().map(DOMArenaRef::from)
  }

  pub fn sibling_ids(&self) -> DOMNodeSiblingIds<T> {
    self.raw.sibling_ids()
  }

  pub fn edge_ids(&self) -> DOMNodeEdgeIds<T> {
    self.raw.edge_ids()
  }

  pub fn children_iter(&self) -> impl Iterator<Item = &'a DOMNode<T>> {
    self.raw.children_values_iter()
  }

  pub fn descendants_iter(&self) -> impl Iterator<Item = &'a DOMNode<T>> {
    self.raw.descendants_values_iter()
  }

  pub fn traverse_iter(&self) -> impl Iterator<Item = &'a DOMNode<T>> {
    self.raw.traverse_values_iter()
  }
}
