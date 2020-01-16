// https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8

use html5ever::driver::ParseOpts;
use html5ever::interface::tree_builder::TreeSink;
use markup5ever_rcdom::RcDom;
use tendril::stream::TendrilSink;

pub fn get_events() {
    let html = crate::ukadoc::LIST_SHIORI_EVENT;
    let parser = html5ever::parse_document(RcDom::default(), ParseOpts::default());
    let mut dom = parser.one(html);
    let doc = dom.get_document();
    let children = doc.children.borrow();
    let items = children.iter().map(|n| (&n.data, &n.children)); //.take(2);
    for (data, children) in items {
        println!("{:?}", data);
    }
}

#[test]
fn read_event() {
    get_events();
}
