// https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8

use html5ever::driver::ParseOpts;
use html5ever::{local_name, LocalName};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::fmt::*;
use tendril::stream::TendrilSink;
use tendril::StrTendril;

#[derive(Debug)]
pub enum Error {
    Other,
    ElementNotFound,
}
pub type Result<T> = std::result::Result<T, Error>;

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
                    }
                    self.child_iter = None;
                }
                _ => (),
            }
            break;
        }
        None
    }
}

trait HandleExt {
    fn flat_tree(&self) -> FlatTreeHandle;
    fn is_element(&self) -> bool;
    fn is_name(&self, local_name: LocalName) -> bool;
    fn attr(&self, attr_name: LocalName) -> Option<StrTendril>;
    fn text(&self) -> String;
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
    fn class_is(&self, name: &str) -> bool {
        match self.class() {
            Some(t) => (&t as &str) == name,
            _ => false,
        }
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
    #[inline]
    fn text(&self) -> String {
        let f = HandleText { h: self };
        format!("{}", f)
    }
}

struct HandleText<'a> {
    h: &'a Handle,
}

impl<'a> Display for HandleText<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_handle(self.h, f)
    }
}

fn fmt_handle(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &h.data {
        NodeData::Text { contents } => write!(f, "{}", contents.borrow()),
        NodeData::Element { name, .. } => match name.local {
            local_name!("p") => fmt_p(h, f),
            local_name!("a") => fmt_a(h, f),
            local_name!("em") => fmt_em(h, f),
            local_name!("strong") => fmt_strong(h, f),
            local_name!("br") => fmt_br(h, f),
            local_name!("dl") => fmt_dl(h, f),
            local_name!("dt") => fmt_dt(h, f),
            local_name!("dd") => fmt_dd(h, f),
            local_name!("ul") => Ok(()),
            _ => {
                panic!("unnone element {}", name.local);
            }
        },
        _ => Ok(()),
    }
}

fn fmt_children(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    let children = h.children.borrow();
    for child in children.iter() {
        fmt_handle(child, f)?;
    }
    Ok(())
}

fn fmt_p(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    fmt_children(h, f);
    Ok(())
}
fn fmt_a(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    fmt_children(h, f)
}
fn fmt_em(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "_")?;
    fmt_children(h, f)?;
    write!(f, "_")?;
    Ok(())
}
fn fmt_strong(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "__")?;
    fmt_children(h, f)?;
    write!(f, "__")?;
    Ok(())
}
fn fmt_br(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "\n")
}
fn fmt_dl(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    fmt_children(h, f)
}
fn fmt_dt(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    fmt_children(h, f)
}
fn fmt_dd(h: &Handle, f: &mut Formatter<'_>) -> std::fmt::Result {
    fmt_children(h, f)
}

#[derive(Debug)]
struct EntryData {
    id: StrTendril,
    desc: String,
}

impl Display for EntryData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[event.'{}']", self.id)?;
        writeln!(f, "desc='''{}'''", self.desc)?;
        Ok(())
    }
}

fn read_entry(entry: &Handle) -> Result<EntryData> {
    let id = entry.id().unwrap();
    let (el_name, el_desc) = {
        let children = entry.children.borrow();
        let el_name = children
            .iter()
            .filter(|n| n.is_name(local_name!("dt")))
            .filter(|n| n.class_is(&"entry"))
            .next()
            .unwrap()
            .clone();
        let el_entry = children
            .iter()
            .filter(|n| n.is_name(local_name!("dd")))
            .filter(|n| n.class_is(&"entry"))
            .next()
            .unwrap()
            .clone();
        let children = el_entry.children.borrow();
        let el_desc = children
            .iter()
            .filter(|n| n.is_name(local_name!("p")))
            .next()
            .unwrap()
            .clone();
        (el_name, el_desc)
    };
    let desc = el_desc.text();
    Ok(EntryData { id, desc })
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
    let entrys = html
        .flat_tree()
        .filter(|n| n.is_name(local_name!("dl")))
        .filter(|n| n.has_id());
    for entry in entrys {
        let data = read_entry(&entry).unwrap();
        println!("{}\n", data);
    }
}

#[test]
fn read_event() {
    get_events();
}
