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

use rsx_tree::types::Tree;
use std::ops::{Deref, DerefMut};
use traits::{TDOMTree, TGenericEvent};

use node::{DOMNode, DOMNodeId, DOMNodeIdPair};
use setup::{DOMArenaRef, DOMArenaRefMut, DOMArenaRefMutPair};

#[derive(Debug, PartialEq)]
pub struct DOMTree<T>
where
    T: TGenericEvent,
{
    raw: Tree<DOMNode<T>>,
}

impl<T> Default for DOMTree<T>
where
    T: TGenericEvent,
{
    fn default() -> Self {
        DOMTree {
            raw: Tree::new(DOMNode::default()),
        }
    }
}

impl<T> Deref for DOMTree<T>
where
    T: TGenericEvent,
{
    type Target = DOMNode<T>;

    fn deref(&self) -> &Self::Target {
        self.root().into_value()
    }
}

impl<T> DerefMut for DOMTree<T>
where
    T: TGenericEvent,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.root_mut().into_value()
    }
}

impl<T> DOMTree<T>
where
    T: TGenericEvent,
{
    pub(crate) fn into_inner(self) -> Tree<DOMNode<T>> {
        self.raw
    }

    pub fn root(&self) -> DOMArenaRef<T> {
        let id = self.raw.root();
        DOMArenaRef::from(self.raw.get(id))
    }

    pub fn root_mut(&mut self) -> DOMArenaRefMut<T> {
        let id = self.raw.root();
        DOMArenaRefMut::from(self.raw.get_mut(id))
    }

    pub fn document(&self) -> DOMArenaRef<T> {
        let id = self.root().first_child_id().unwrap();
        DOMArenaRef::from(self.raw.get(id))
    }

    pub fn document_mut(&mut self) -> DOMArenaRefMut<T> {
        let id = self.root().first_child_id().unwrap();
        DOMArenaRefMut::from(self.raw.get_mut(id))
    }

    pub fn alloc(&mut self, node: DOMNode<T>) -> DOMNodeId<T> {
        self.raw.alloc(node)
    }

    pub fn get(&self, id: DOMNodeId<T>) -> DOMArenaRef<T> {
        DOMArenaRef::from(self.raw.get(id))
    }

    pub fn get_mut(&mut self, id: DOMNodeId<T>) -> DOMArenaRefMut<T> {
        DOMArenaRefMut::from(self.raw.get_mut(id))
    }

    pub fn get_mut_pair(&mut self, ids: DOMNodeIdPair<T>) -> DOMArenaRefMutPair<T> {
        DOMArenaRefMutPair::from(self.raw.get_mut_pair(ids))
    }
}

impl<T> TDOMTree for DOMTree<T>
where
    T: TGenericEvent,
{
    type Node = DOMNode<T>;

    fn get_node(&self, id: DOMNodeId<T>) -> &Self::Node {
        self.get(id).into_value()
    }

    fn get_node_mut(&mut self, id: DOMNodeId<T>) -> &mut Self::Node {
        self.get_mut(id).into_value()
    }

    fn get_node_mut_pair(&mut self, ids: DOMNodeIdPair<T>) -> (&mut Self::Node, &mut Self::Node) {
        self.get_mut_pair(ids).into_values()
    }
}

impl<T> DOMTree<T>
where
    T: TGenericEvent,
{
    pub fn generate_layout_tree(&mut self) {
        self.root_mut().build_layout();
    }
}
