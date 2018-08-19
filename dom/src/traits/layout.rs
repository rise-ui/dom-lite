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

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[fundamental]
pub trait TClientRect: Debug + PartialEq + Copy + Serialize + for<'a> Deserialize<'a> {
  type Position: TClientPosition;
  type Size: TClientSize;

  fn position(&self) -> Self::Position;

  fn size(&self) -> Self::Size;

  fn offset_from_page(&self, (u32, u32)) -> (u32, u32);

  fn client_from_page(&self, (u32, u32)) -> (u32, u32);

  fn contains_point(&self, (u32, u32)) -> bool;
}

#[fundamental]
pub trait TClientPosition: Debug + PartialEq + Copy + Serialize + for<'a> Deserialize<'a> {}

#[fundamental]
pub trait TClientSize: Debug + PartialEq + Copy + Serialize + for<'a> Deserialize<'a> {}

#[fundamental]
pub trait TLayoutNode: Debug + PartialEq {
  // type TextMeasureMetadata;
  type ReflowDirection;

  fn is_tainted(&self) -> bool;

  fn insert_child(&mut self, &mut Self, u32);

  fn append_child(&mut self, &mut Self);

  fn remove_child(&mut self, &mut Self);

  fn reflow_subtree(&mut self, u32, u32, Self::ReflowDirection);

  fn child_count(&self) -> u32;
}
