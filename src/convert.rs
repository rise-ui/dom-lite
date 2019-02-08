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

use hashbrown::HashMap;
use jss::types::Style;
use std::borrow::Cow;
use std::rc::Rc;

use types::{Closure, EventType, KnownAttributeName, KnownElementName, Prop};

use node::{
    DOMAttribute, DOMAttributeName, DOMAttributeValue, DOMAttributes, DOMData, DOMNode,
    DOMNormalNode, DOMTagName, DOMText, DOMTextNode,
};

use traits::TGenericEvent;

use tree::DOMTree;

impl<E> From<()> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from(_: ()) -> Self {
        DOMNode::new(DOMData::Void)
    }
}

impl<E> From<Style> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from(style: Style) -> Self {
        let mut node = DOMNode::new(DOMData::Void);
        node.styles = style;
        node
    }
}

impl<E> From<&'static str> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from(text: &'static str) -> Self {
        DOMNode::new(DOMData::Text(DOMTextNode {
            content: DOMText::from(text),
        }))
    }
}

impl<E> From<String> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from(text: String) -> Self {
        DOMNode::new(DOMData::Text(DOMTextNode {
            content: DOMText::from(text),
        }))
    }
}

impl_text_node_from_stringifiable!(bool);
impl_text_node_from_stringifiable!(i8);
impl_text_node_from_stringifiable!(u8);
impl_text_node_from_stringifiable!(i16);
impl_text_node_from_stringifiable!(u16);
impl_text_node_from_stringifiable!(i32);
impl_text_node_from_stringifiable!(u32);
impl_text_node_from_stringifiable!(i64);
impl_text_node_from_stringifiable!(u64);
impl_text_node_from_stringifiable!(f32);
impl_text_node_from_stringifiable!(f64);
impl_text_node_from_stringifiable!(isize);
impl_text_node_from_stringifiable!(usize);
impl_text_node_from_stringifiable!(char);

impl <E>Default for DOMAttributes<E>
where E: TGenericEvent {
    fn default() -> DOMAttributes<E> {
        let namespaced: HashMap<(&'static str, &'static str), DOMAttributeValue<E>> = HashMap::default();
        let common: HashMap<KnownAttributeName, DOMAttributeValue<E>> = HashMap::default();
        let simple: HashMap<&'static str, DOMAttributeValue<E>> = HashMap::default();
        let listeners: HashMap<EventType, Closure<E>> = HashMap::default();

        DOMAttributes {
            namespaced,
            listeners,
            common,
            simple,
        }
    }
}

impl<E> From<Vec<DOMAttribute<E>>> for DOMAttributes<E>
where E: TGenericEvent,
{
    fn from(list: Vec<DOMAttribute<E>>) -> DOMAttributes<E> {
        let mut attributes = DOMAttributes::default();
        use self::DOMAttributeName::*;
        
        for attr in list {
            let value = attr.1;
            let key = attr.0;

            let _:bool = match key {
                NamedspacedName(group, subgroup) => attributes.namespaced.insert((group, subgroup), value).is_some(),
                KnownName(name) => attributes.common.insert(name, value).is_some(),
                Simple(name) => attributes.simple.insert(name, value).is_some(),

                EventType(event_type) => {
                    if let DOMAttributeValue::EventListener(listener) = value {
                        attributes.listeners.insert(event_type, listener).is_some();
                        true
                    } else {
                        false
                    }
                },
            };
        }

        attributes
    }
}

impl<E> From<DOMTagName> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from(tag: DOMTagName) -> Self {
        let attributes = DOMAttributes::default();
        DOMNode::new(DOMData::Normal(DOMNormalNode { tag, attributes }))
    }
}

impl<E> From<(DOMTagName, DOMAttributes<E>)> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from((tag, attributes): (DOMTagName, DOMAttributes<E>)) -> Self {
        DOMNode::new(DOMData::Normal(DOMNormalNode { tag, attributes }))
    }
}

impl<E> From<(DOMTagName, Style)> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from((tag, style): (DOMTagName, Style)) -> Self {
        let mut node = DOMNode::from((tag, DOMAttributes::default()));
        node.styles = style;
        node
    }
}

impl<E> From<(DOMTagName, DOMAttributes<E>, Style)> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from((tag, attributes, style): (DOMTagName, DOMAttributes<E>, Style)) -> Self {
        let mut node = DOMNode::from((tag, attributes));
        node.styles = style;
        node
    }
}

impl<E> From<(DOMTagName, Vec<DOMAttribute<E>>, Style)> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from((tag, attributes, style): (DOMTagName, Vec<DOMAttribute<E>>, Style)) -> Self {
        let mut node = DOMNode::from((tag, DOMAttributes::from(attributes)));
        node.styles = style;
        node
    }
}

impl<E> From<(DOMTagName, Vec<DOMAttribute<E>>)> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from((tag, attributes): (DOMTagName, Vec<DOMAttribute<E>>)) -> Self {
        DOMNode::from((tag, DOMAttributes::from(attributes)))
    }
}

impl<E> From<DOMTree<E>> for DOMNode<E>
where
    E: TGenericEvent,
{
    fn from(tree: DOMTree<E>) -> Self {
        DOMNode::new(DOMData::ShadowHost(tree))
    }
}

impl From<&'static str> for DOMText {
    fn from(value: &'static str) -> Self {
        DOMText::Static(Cow::from(value))
    }
}

impl From<String> for DOMText {
    fn from(value: String) -> Self {
        DOMText::Owned(Rc::new(value))
    }
}

impl From<KnownElementName> for DOMTagName {
    fn from(name: KnownElementName) -> Self {
        DOMTagName::KnownName(name)
    }
}

impl From<&'static str> for DOMTagName {
    fn from(name: &'static str) -> Self {
        DOMTagName::Simple(name)
    }
}

impl From<(&'static str, &'static str)> for DOMTagName {
    fn from((namespace, name): (&'static str, &'static str)) -> Self {
        DOMTagName::NamedspacedName(namespace, name)
    }
}

impl<E> From<(DOMAttributeName, DOMAttributeValue<E>)> for DOMAttribute<E>
where
    E: TGenericEvent,
{
    fn from((name, value): (DOMAttributeName, DOMAttributeValue<E>)) -> Self {
        DOMAttribute(name, value)
    }
}

impl From<KnownAttributeName> for DOMAttributeName {
    fn from(name: KnownAttributeName) -> Self {
        DOMAttributeName::KnownName(name)
    }
}

impl From<EventType> for DOMAttributeName {
    fn from(name: EventType) -> Self {
        DOMAttributeName::EventType(name)
    }
}

impl From<&'static str> for DOMAttributeName {
    fn from(name: &'static str) -> Self {
        DOMAttributeName::Simple(name)
    }
}

impl From<(&'static str, &'static str)> for DOMAttributeName {
    fn from((namespace, name): (&'static str, &'static str)) -> Self {
        DOMAttributeName::NamedspacedName(namespace, name)
    }
}

impl<E> From<bool> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: bool) -> Self {
        DOMAttributeValue::Boolean(value)
    }
}

impl<E> From<f64> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: f64) -> Self {
        DOMAttributeValue::Number(value)
    }
}

impl_number_attribute_from_countable!(i8);
impl_number_attribute_from_countable!(u8);
impl_number_attribute_from_countable!(i16);
impl_number_attribute_from_countable!(u16);
impl_number_attribute_from_countable!(i32);
impl_number_attribute_from_countable!(u32);
impl_number_attribute_from_countable!(i64);
impl_number_attribute_from_countable!(u64);
impl_number_attribute_from_countable!(f32);
impl_number_attribute_from_countable!(isize);
impl_number_attribute_from_countable!(usize);

impl<E> From<char> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: char) -> Self {
        DOMAttributeValue::Char(value)
    }
}

impl<E> From<&'static str> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: &'static str) -> Self {
        DOMAttributeValue::Str(DOMText::from(value))
    }
}

impl<E> From<String> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: String) -> Self {
        DOMAttributeValue::Str(DOMText::from(value))
    }
}

impl<E> From<DOMText> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: DOMText) -> Self {
        DOMAttributeValue::Str(value)
    }
}

impl<E> From<Prop> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: Prop) -> Self {
        DOMAttributeValue::Prop(value)
    }
}

impl<E> From<Closure<E>> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: Closure<E>) -> Self {
        DOMAttributeValue::EventListener(value)
    }
}

impl<E> From<DOMNode<E>> for DOMAttributeValue<E>
where
    E: TGenericEvent,
{
    fn from(value: DOMNode<E>) -> Self {
        DOMAttributeValue::Node(value)
    }
}
