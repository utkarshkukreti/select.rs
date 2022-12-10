use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn main() {
    // stackoverflow.html was fetched from
    // http://stackoverflow.com/questions/tagged/rust?sort=votes&pageSize=50 on
    // Aug 10, 2015.
    let document = Document::from(include_str!("stackoverflow.html"));

    println!("# Menu");
    for node in document.find(Attr("id", "hmenus").descendant(Name("a"))) {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    }
    println!();

    println!("# Top 5 Questions");
    for node in document.find(Class("question-summary")).take(5) {
        let question = node.find(Class("question-hyperlink")).next().unwrap();
        let votes = node.find(Class("vote-count-post")).next().unwrap().text();
        let answers = node
            .find(Class("status").descendant(Name("strong")))
            .next()
            .unwrap()
            .text();
        let tags = node
            .find(Class("post-tag"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();
        let asked_on = node.find(Class("relativetime")).next().unwrap().text();
        let asker = node
            .find(Class("user-details").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        println!(" Question: {}", question.text());
        println!("  Answers: {}", answers);
        println!("    Votes: {}", votes);
        println!("   Tagged: {}", tags.join(", "));
        println!(" Asked on: {}", asked_on);
        println!("    Asker: {}", asker);
        println!(
            "Permalink: http://stackoverflow.com{}",
            question.attr("href").unwrap()
        );
        println!();
    }

    println!("# Top 10 Related Tags");
    for node in document
        .find(Attr("id", "h-related-tags"))
        .next()
        .unwrap()
        .parent()
        .unwrap()
        .find(Name("div"))
        .take(10)
    {
        let tag = node.find(Name("a")).next().unwrap().text();
        let count = node
            .find(Class("item-multiplier-count"))
            .next()
            .unwrap()
            .text();
        println!("{} ({})", tag, count);
    }
}
