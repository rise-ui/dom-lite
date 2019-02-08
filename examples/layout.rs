#[macro_use]
extern crate dom;
extern crate jss;
extern crate yoga;

use dom::events::*;
use dom::node::*;
use dom::setup::*;
use dom::traits::*;
use dom::tree::*;
use dom::types::*;

use jss::types::*;
use yoga::Direction;

fn debug_layout<'a>(node: &mut DOMArenaRefMut<'a, BasicEvent>) {
    println!(
        "Node: {:#?}\n",
        node.raw
            .try_value()
            .and_then(|value| Some(value.layout_node.get_layout()))
    );

    let mut next_child_id = node.first_child_id();

    while let Some(child_id) = next_child_id {
        {
            let mut child_ref = node.get_mut(child_id);
            debug_layout(&mut child_ref);
        }

        next_child_id = node.get(child_id).next_sibling_id();
    }
}

fn main() {
    let container_style = StyleBuilder::default()
        .case(Case::Ignore)
        .parse_from_str(
            r#"{
        "justify-content": "space-between",
        "align-items": "center"
    }"#,
        )
        .unwrap();

    let item_style = StyleBuilder::default()
        .case(Case::Ignore)
        .parse_from_str(
            r#"{
        "justify-content": "space-between",
        "align-items": "center",
        "height": "300px",
        "width": "200px",
        "transform": [
            "rotate(90deg,90deg)"
        ]
    }"#,
        )
        .unwrap();

    println!("{:#?}", item_style);

    let mut tree: DOMTree<BasicEvent> = {
        let mut fragment = DOMTree::default();

        {
            let mut parent = fragment.root_mut();
            {
                let mut parent = parent.append(DOMNode::from((
                    DOMTagName::from(KnownElementName::Div),
                    vec![DOMAttribute::from((
                        DOMAttributeName::from("name"),
                        DOMAttributeValue::from("body"),
                    ))],
                    container_style,
                )));

                {
                    let mut first_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![DOMAttribute::from((
                            DOMAttributeName::from("name"),
                            DOMAttributeValue::from("item"),
                        ))],
                        item_style.clone(),
                    )));
                }

                {
                    let mut second_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![DOMAttribute::from((
                            DOMAttributeName::from("name"),
                            DOMAttributeValue::from("item"),
                        ))],
                        item_style.clone(),
                    )));
                }
            }
        }

        fragment
    };

    // Build build-layout
    let mut document = tree.document_mut();
    {
        document.build_layout();
        document
            .value_mut()
            .reflow_subtree(1024, 768, Direction::LTR);
    }

    // Iterator
    debug_layout(&mut document);
}
