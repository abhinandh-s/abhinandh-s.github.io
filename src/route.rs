use yew_router::prelude::*;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    ArticlesRoute,
    #[at("/articles/:id")]
    Articles { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
