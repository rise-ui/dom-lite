#[macro_use]
extern crate dom;

use dom::traits::*;
use dom::events::*;
use dom::types::*;
use dom::node::*;
use dom::tree::*;

fn main() {
  let tree: DOMTree<BasicEvent> = fragment! {
      DOMNode::from((
          DOMTagName::from(KnownElementName::Div),
          vec![
              DOMAttribute::from((DOMAttributeName::from("foo"), DOMAttributeValue::from(true))),
          ],
          vec![
              DOMNode::from((
                  DOMTagName::from("bar"),
                  vec![
                      DOMAttribute::from((
                          DOMAttributeName::from("baz"),
                          DOMAttributeValue::from(false)
                      )),
                  ],
                  vec![DOMNode::from("Hello")]
              )),
              DOMNode::from({ "world" }),
              DOMNode::from({ "!" }),
          ]
      ))
  };

  println!("{:#?}", tree);
}
