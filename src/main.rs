use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use self::pages::articles::Article;

mod articles;
mod components;
mod pages;
mod route;

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
        Route::Portfolio => html! { <pages::portfolio::Portfolio /> },
        Route::Articles { id } => html! { <Article post_id={id} /> },
        Route::ArticlesRoute => html! { <pages::articles::ArticleIndex /> },
        Route::About => html! { <pages::about::About /> },
        Route::NotFound => html! { <pages::_404::NotFound /> },
    }
}
