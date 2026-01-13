#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub post_id: String,
}

use yew::prelude::*;

use crate::articles::{get_article_by_id, markdown_to_html};

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    let post = get_article_by_id(&props.post_id);
    match post {
        Some(post) => {
            let html_content = markdown_to_html(&post.content);
            let ctx = Html::from_html_unchecked(html_content.into());

            html! {
                <>
                    <div  class="h-lvh !bg-latte-crust dark:!bg-mocha-crust !text-latte-text dark:!text-mocha-text">
                    <div class="p-2 mx-auto max-w-3xl flex flex-col justify-center">
                    <h1>{ post.matter.title }</h1>
                    <div>

                    <div>
                    { ctx }
                    </div>
                    
                    <h1>{ "this" }</h1>
                    <h1>{ "this" }</h1>
                    </div>
                    </div>
                    </div>
                </>
            }
        }
        None => {
            html!()
        }
    }
}
