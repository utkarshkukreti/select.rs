#![feature(plugin, test)]
#![plugin(speculate)]

extern crate test;

extern crate html5ever;
extern crate select;

speculate! {
    context "struct.Vec.html (228,512 bytes)" {
        before {
            let str = include_str!("../tests/fixtures/struct.Vec.html");
        }

        bench "constructing html5ever::rcdom::RcDom" |b| {{
            use html5ever::{parse, one_input, rcdom};
            b.iter(|| {
                let rc_dom: rcdom::RcDom = parse(one_input(str.into()),
                                                 Default::default());
                rc_dom
            });
        };}

        bench "constructing select::document::Document" |b| {
            b.iter(|| select::document::Document::from_str(str));
        }
    }
}
