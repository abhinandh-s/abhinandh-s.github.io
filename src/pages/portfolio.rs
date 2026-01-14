use yew::prelude::*;

#[function_component(Portfolio)]
pub fn portfolio_page() -> Html {
    html! {
      <>
       <crate::components::header::Header />
       <crate::components::footer::Footer />
      </>
    }
}
