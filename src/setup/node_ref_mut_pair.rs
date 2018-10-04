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

use node::DOMNode;
use rsx_tree::types::RefMutPair;
use traits::TGenericEvent;

#[derive(Debug, PartialEq)]
pub struct DOMArenaRefMutPair<'a, T: 'a>
where
    T: TGenericEvent,
{
    raw: RefMutPair<'a, DOMNode<T>>,
}

impl<'a, T> From<RefMutPair<'a, DOMNode<T>>> for DOMArenaRefMutPair<'a, T>
where
    T: TGenericEvent,
{
    fn from(raw: RefMutPair<'a, DOMNode<T>>) -> Self {
        DOMArenaRefMutPair { raw }
    }
}

impl<'a, T> DOMArenaRefMutPair<'a, T>
where
    T: TGenericEvent,
{
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    pub fn values(&mut self) -> (&mut DOMNode<T>, &mut DOMNode<T>) {
        self.raw.try_values().expect("Nodes deallocated")
    }

    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    pub fn into_values(self) -> (&'a mut DOMNode<T>, &'a mut DOMNode<T>) {
        self.raw.try_into_values().expect("Nodes deallocated")
    }
}
