use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
      <>
        <h1 class="text-4xl font-bold"> { "404 - Page not found" }</h1>
        <p class="my-4">{ "The page you were looking for doesn't exist." }</p>
        <a href="/" class="underline">{ "Go back home" }</a>
      </>
    }
}
