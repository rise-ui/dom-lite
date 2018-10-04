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

use node::{DOMAttribute, DOMAttributeName, DOMAttributeValue, DOMText};
use traits::TGenericEvent;
use types::{Closure, EventType, KnownAttributeName};

pub fn is_event_listener<T>(attribute: &DOMAttribute<T>) -> Option<&Closure<T>>
where
    T: TGenericEvent,
{
    use self::DOMAttributeName::KnownName;
    use self::DOMAttributeValue::EventListener;
    use self::KnownAttributeName::Style;
    match attribute {
        &DOMAttribute(KnownName(Style), EventListener(ref l)) => Some(l),
        _ => None,
    }
}

pub fn is_src<T>(attribute: &DOMAttribute<T>) -> Option<&DOMText>
where
    T: TGenericEvent,
{
    use self::DOMAttributeName::KnownName;
    use self::DOMAttributeValue::Str;
    use self::KnownAttributeName::Src;
    match attribute {
        &DOMAttribute(KnownName(Src), Str(ref s)) => Some(s),
        _ => None,
    }
}

pub fn find_attribute<'a, I, T>(iter: I, name: &DOMAttributeName) -> Option<&'a DOMAttribute<T>>
where
    I: IntoIterator<Item = &'a DOMAttribute<T>>,
    T: TGenericEvent,
{
    iter.into_iter().find(|v| &v.0 == name)
}

pub fn find_event_listener<'a, I, T: 'a>(iter: I, ty: EventType) -> Option<&'a Closure<T>>
where
    I: IntoIterator<Item = &'a DOMAttribute<T>>,
    T: TGenericEvent,
{
    let name = DOMAttributeName::EventType(ty);
    is_event_listener(find_attribute(iter, &name)?)
}

pub fn find_src<'a, I, T: 'a>(iter: I) -> Option<&'a DOMText>
where
    I: IntoIterator<Item = &'a DOMAttribute<T>>,
    T: TGenericEvent,
{
    let name = DOMAttributeName::KnownName(KnownAttributeName::Src);
    is_src(find_attribute(iter, &name)?)
}
