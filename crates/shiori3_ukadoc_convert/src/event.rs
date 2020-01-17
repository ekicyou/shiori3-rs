// https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8

use html5ever::driver::ParseOpts;
use html5ever::{local_name, LocalName};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use tendril::stream::TendrilSink;
use tendril::StrTendril;

//fn iter_node(node:&Node)-> impl  std::iter::Map<std::slice::Iter<'_, std::rc::Rc<markup5ever_rcdom::Node>>, [closure@crates\shiori3_ukadoc_convert\src\event.rs:14:37: 14:63]>
//&markup5ever_rcdom::NodeData
//&std::cell::RefCell<std::vec::Vec<std::rc::Rc<markup5ever_rcdom::Node>>>

struct FlatTreeHandle {
    owner: Handle,
    index: usize,
    child_iter: Option<Box<dyn Iterator<Item = Handle>>>,
}

impl FlatTreeHandle {
    pub fn new(owner: &Handle) -> FlatTreeHandle {
        FlatTreeHandle {
            owner: owner.clone(),
            index: 0,
            child_iter: Some(Box::new(Some(owner.clone()).into_iter())),
        }
    }
}

impl Iterator for FlatTreeHandle {
    type Item = Handle;
    fn next(&mut self) -> Option<Handle> {
        loop {
            match &mut self.child_iter {
                Some(iter) => {
                    if let Some(item) = iter.next() {
                        return Some(item);
                    }
                    let v = self.owner.children.borrow();
                    if v.len() > self.index {
                        self.child_iter = Some(Box::new(Self::new(&v[self.index])));
                        self.index += 1;
                        continue;
                    } else {
                        self.child_iter = None;
                        return None;
                    }
                }
                _ => return None,
            }
        }
    }
}

trait HandleExt {
    fn flat_tree(&self) -> FlatTreeHandle;
    fn is_element(&self) -> bool;
    fn is_name(&self, local_name: LocalName) -> bool;
    fn attr(&self, attr_name: LocalName) -> Option<StrTendril>;
    fn id(&self) -> Option<StrTendril> {
        self.attr(local_name!("id"))
    }
    fn has_id(&self) -> bool {
        self.id().is_some()
    }
    fn class(&self) -> Option<StrTendril> {
        self.attr(local_name!("class"))
    }
    fn has_class(&self) -> bool {
        self.class().is_some()
    }
}
impl HandleExt for Handle {
    fn flat_tree(&self) -> FlatTreeHandle {
        FlatTreeHandle::new(self)
    }
    fn attr(&self, attr_name: LocalName) -> Option<StrTendril> {
        match &self.data {
            NodeData::Element { attrs, .. } => {
                let value = attrs
                    .borrow()
                    .iter()
                    .filter(|a| a.name.local == attr_name)
                    .next()
                    .map(|a| a.value.clone());
                value
            }
            _ => None,
        }
    }

    #[inline]
    fn is_element(&self) -> bool {
        match &self.data {
            NodeData::Element { .. } => true,
            _ => false,
        }
    }
    #[inline]
    fn is_name(&self, local_name: LocalName) -> bool {
        match &self.data {
            NodeData::Element { name, .. } => name.local == local_name,
            _ => false,
        }
    }
}

pub fn get_events() {
    let html_text = crate::ukadoc::LIST_SHIORI_EVENT;
    let parser = html5ever::parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html_text);
    let dom_children = dom.document.children.borrow();
    let html = dom_children
        .iter()
        .filter(|n| n.is_element())
        .next()
        .unwrap();
    println!("{:?}", html.data);
    let html_children = html.children.borrow();
    // body
    let body = html_children
        .iter()
        .filter(|n| n.is_name(local_name!("body")))
        .next()
        .unwrap();
    println!("{:?}", body.data);
    // flat map
    let all = html
        .flat_tree()
        .filter(|n| n.is_name(local_name!("dl")))
        .filter(|n| n.has_id());
    for entry in all {
        let id = entry.id().unwrap();
        println!("id={}", id);
    }
}

#[test]
fn read_event() {
    get_events();
}
