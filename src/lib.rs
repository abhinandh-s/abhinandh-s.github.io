use route::Route;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use self::pages::articles::Article;

mod utils;
mod components;
mod pages;
mod route;

#[function_component(App)]
fn app() -> Html {
    html!(
        <BrowserRouter> // `HashRouter` is needed for github pages
            <Switch<Route> render={switch} />
        </BrowserRouter>
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

#[wasm_bindgen]
pub async fn render() -> String {
    let renderer = yew::ServerRenderer::<App>::new();
    renderer.render().await
}
