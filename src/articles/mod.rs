use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};

mod generated;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub matter: FrontMatter,
    pub content: String,
}

// The default `Pod` data type can be a bit unwieldy, so
// you can also deserialize it into a custom struct
#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub published_at: String,
    pub snippet: String,
    pub tags: Option<Vec<String>>,
}

pub fn get_all_articles() -> Vec<Article> {
    let mut articles = Vec::new();
    let mut dbg = String::new();

    for (id, ctx) in generated::ARTICLES.to_owned() {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        match matter.parse::<FrontMatter>(ctx) {
            Ok(result) => {
                articles.push(Article {
                    id: id.to_owned(),
                    matter: result.data.unwrap_or_default(),
                    content: result.content,
                });
            }
            Err(err) => dbg.push_str(err.to_string().as_str()),
        }
    }
    articles
}

// input: `2026-01-12 21:34`
// return it as [`Monday, November 25, 2024`]
pub fn date_time(input: &str) -> String {
    // 1. Parse the input string based on its format
    // %Y-%m-%d %H:%M matches "YYYY-MM-DD HH:MM"
    match chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d") {
    // 2. Format it to the desired output: "Monday, January 12, 2026"
    // %A = Full weekday, %B = Full month, %e = Day of month, %Y = Year
        Ok(date_time) => date_time.format("%A, %B %e, %Y").to_string(),
        Err(err) => err.to_string(),
    }
}

pub fn get_article_by_id(id: &str) -> Option<Article> {
    get_all_articles().into_iter().find(|f| f.id == id)
}

pub fn markdown_to_html(md: &str) -> String {
    let options = pulldown_cmark::Options::all();
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
