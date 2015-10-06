extern crate select;
use select::document::Document;
use select::predicate::*;

pub fn main() {
    // stackoverflow.html was fetched from
    // http://stackoverflow.com/questions/tagged/rust?sort=votes&pageSize=50 on
    // Aug 10, 2015.
    let document = Document::from_str(include_str!("stackoverflow.html"));

    println!("# Menu");
    for node in document.find(Attr("id", "hmenus")).find(Name("a")).iter() {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    }
    println!("");

    println!("# Top 5 Questions");
    for node in document.find(Class("question-summary")).iter().take(5) {
        let question = node.find(Class("question-hyperlink")).first().unwrap();
        let votes = node.find(Class("vote-count-post")).first().unwrap().text();
        let answers = node.find(Class("status")).find(Name("strong")).first().unwrap().text();
        let tags = node.find(Class("post-tag")).iter().map(|tag| tag.text()).collect::<Vec<_>>();
        let asked_on = node.find(Class("relativetime")).first().unwrap().text();
        let asker = node.find(Class("user-details")).find(Name("a")).first().unwrap().text();
        println!(" Question: {}", question.text());
        println!("  Answers: {}", answers);
        println!("    Votes: {}", votes);
        println!("   Tagged: {}", tags.join(", "));
        println!(" Asked on: {}", asked_on);
        println!("    Asker: {}", asker);
        println!("Permalink: http://stackoverflow.com{}", question.attr("href").unwrap());
        println!("");
    }

    println!("# Top 10 Related Tags");
    for node in document.find(Attr("id", "h-related-tags")).parent().find(Name("div")).iter().take(10) {
        let tag = node.find(Name("a")).first().unwrap().text();
        let count = node.find(Class("item-multiplier-count")).first().unwrap().text();
        println!("{} ({})", tag, count);
    }
}
