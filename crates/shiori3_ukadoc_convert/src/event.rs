// https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8


use markup5ever_rcdom as rcdom;

use std::cell::RefCell;
use std::rc::Rc;
use html5ever::{Attribute, LocalName, QualName};
use html5ever::driver::ParseOpts;
use html5ever::{local_name, ns, namespace_url};
use html5ever::{parse_document, parse_fragment};
use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::StrTendril;
use html5ever::tendril::stream::TendrilSink;
use rcdom::{Handle, Node, NodeData, RcDom};

pub fn get_events(){
    let html = crate::ukadoc::LIST_SHIORI_EVENT;
    let parser = parse_document(RcDom::default(), ParseOpts::default());
let dom = parser.one(html);

}
