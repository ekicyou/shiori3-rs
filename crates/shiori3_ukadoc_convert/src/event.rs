// https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8

use markup5ever_rcdom::{RcDom};
use html5ever::driver::ParseOpts;
use tendril::stream::TendrilSink;

pub fn get_events() -> (){
    let html = crate::ukadoc::LIST_SHIORI_EVENT;
    let parser = html5ever::parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html);
    ()
}

#[test]
fn read_event(){
    get_events();
}