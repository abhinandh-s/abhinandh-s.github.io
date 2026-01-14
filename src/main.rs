use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use self::pages::post::Article;

mod articles;
mod pages;
mod route;
mod components;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::home::HomePage /> },
        Route::Articles { id } => html! { <Article post_id={id} /> },
        Route::ArticlesRoute => html! { <pages::post::ArticleIndex /> },
        Route::NotFound => html! { <pages::_404::NotFound /> },
    }
}
