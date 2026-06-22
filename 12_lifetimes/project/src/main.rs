// HTML Builder — Build HTML strings using a struct that borrows tag names
//
// HtmlBuilder<'a> holds a &'a str for tag names from external config.
// Demonstrates that struct lifetimes let you borrow configuration
// rather than owning/cloning it.

use std::fmt::Write;  // write! macro for String

/// Configuration that provides tag names for HTML generation.
/// The builder borrows from this — it doesn't own the config.
struct HtmlConfig<'a> {
    pub h1_tag: &'a str,
    pub p_tag: &'a str,
    pub ul_tag: &'a str,
    pub li_tag: &'a str,
    pub a_tag: &'a str,
}

/// Builder that generates HTML. All tags are borrowed from a config.
struct HtmlBuilder<'a> {
    config: &'a HtmlConfig<'a>,
}

impl<'a> HtmlBuilder<'a> {
    fn new(config: &'a HtmlConfig<'a>) -> Self {
        HtmlBuilder { config }
    }

    /// Generate <h1>text</h1> using borrowed tag name
    fn h1(&self, text: &str) -> String {
        format!("<{}>{}</{}>", self.config.h1_tag, text, self.config.h1_tag)
    }

    /// Generate <p>text</p>
    fn p(&self, text: &str) -> String {
        format!("<{}>{}</{}>", self.config.p_tag, text, self.config.p_tag)
    }

    /// Generate <ul>items</ul> where items is a list of pre-built <li> strings
    fn ul(&self, items: &[String]) -> String {
        let mut html = format!("<{}>", self.config.ul_tag);
        for item in items {
            html.push_str(item);
            html.push('\n');
        }
        html.push_str(&format!("</{}>", self.config.ul_tag));
        html
    }

    /// Generate <li>text</li>
    fn li(&self, text: &str) -> String {
        format!("<{}>{}</{}>", self.config.li_tag, text, self.config.li_tag)
    }

    /// Generate <a href="url">text</a>
    fn a(&self, href: &str, text: &str) -> String {
        format!("<{} href=\"{}\">{}</{}>", self.config.a_tag, href, text, self.config.a_tag)
    }

    /// Generate a full HTML page with all content
    fn page(&self, title: &str, body: &[String]) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head><title>");
        html.push_str(title);
        html.push_str("</title></head>\n<body>\n");
        for item in body {
            html.push_str("  ");
            html.push_str(item);
            html.push('\n');
        }
        html.push_str("</body>\n</html>");
        html
    }
}

fn main() {
    // Config lives on the stack — HtmlBuilder borrows from it
    let config = HtmlConfig {
        h1_tag: "h1",
        p_tag: "p",
        ul_tag: "ul",
        li_tag: "li",
        a_tag: "a",
    };

    let builder = HtmlBuilder::new(&config);

    // Build some content
    let heading = builder.h1("Welcome to My Site");
    let para = builder.p("This page was built with borrowed tag names.");
    let link = builder.a("https://rust-lang.org", "Learn Rust");
    let list_items = vec![
        builder.li("Zero-cost abstractions"),
        builder.li("Move semantics"),
        builder.li("Lifetime guarantees"),
    ];
    let list = builder.ul(&list_items);

    let body = vec![heading, para, link, list];
    let page = builder.page("Borrowed Tags", &body);

    println!("{}", page);

    // Prove the config lives long enough — we can still use it
    println!("\n--- Tag names came from config: h1={}, p={} ---",
             config.h1_tag, config.p_tag);
}
