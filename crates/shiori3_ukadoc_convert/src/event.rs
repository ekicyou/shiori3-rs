// https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8

use html5ever::driver::ParseOpts;
use html5ever::interface::tree_builder::TreeSink;
use markup5ever_rcdom::{Node, NodeData, RcDom};
use std::cell::RefCell;
use std::rc::Rc;
use tendril::stream::TendrilSink;

//fn iter_node(node:&Node)-> impl  std::iter::Map<std::slice::Iter<'_, std::rc::Rc<markup5ever_rcdom::Node>>, [closure@crates\shiori3_ukadoc_convert\src\event.rs:14:37: 14:63]>
//&markup5ever_rcdom::NodeData
//&std::cell::RefCell<std::vec::Vec<std::rc::Rc<markup5ever_rcdom::Node>>>

struct RefNode<'a> {
    data: &'a NodeData,
    children: &'a RefCell<Vec<Rc<Node>>>,
}

struct Children {
    children: RefCell<Vec<Rc<Node>>>,
}
impl Iterator<Item = RefNode> for Children {
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn iter_ref_node(children: &RefCell<Vec<Rc<Node>>>) -> impl Iterator<Item = RefNode> {
    let children = children.clone();
    let items = children.iter().map(|n| RefNode {
        data: &n.data,
        children: &n.children,
    });
    items
}

pub fn get_events() {
    let html = crate::ukadoc::LIST_SHIORI_EVENT;
    let parser = html5ever::parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html);
    let doc = &dom.document;
    let children = doc.children.borrow();
    let items = children.iter().map(|n| RefNode {
        data: &n.data,
        children: &n.children,
    }); //.take(2);
    for node in items {
        println!("{:?}", node.data);
    }
}

#[test]
fn read_event() {
    get_events();
}
