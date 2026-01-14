use yew::prelude::*;
 use yew_router::prelude::Link;

 use crate::route::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let link_class = "hover:text-just-red aria-[current]:text-just-red";
    let mobile_link_class = "block py-2 px-4 hover:text-just-red aria-[current]:text-just-red";

    // State for handling the mobile menu toggle
    let is_menu_open = use_state(|| false);

    // Toggle menu visibility
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    // Helper for conditional class
    let mobile_menu_class = if *is_menu_open {
        "md:hidden flex flex-col items-center"
    } else {
        "hidden md:hidden flex flex-col items-center"
    };

    html! {
        <div class="font-bold">
            <nav class="w-full min-h-32 max-tablet:min-h-16 top-0 left-0 z-10">
                <div class="mx-auto px-4">
                    <div class="flex justify-end items-center pt-8 max-tablet:py-4">
                        
                        /* Desktop Menu */
                        <div class="flex max-tablet:hidden space-x-16 mt-12 pb-7 px-16">
                            <Link<Route> to={Route::Home} classes={link_class}>
                                { "Home" }
                            </Link<Route>>
                            <a href="/portfolio" class="hover:text-just-red aria-[current]:text-just-red">
                                { "Portfolio" }
                            </a>
                            <a href="/articles" class="hover:text-just-red aria-[current]:text-just-red">
                                { "Articles" }
                            </a>
                            <a href="/about" class="hover:text-just-red aria-[current]:text-just-red">
                                { "About" }
                            </a>
                        </div>

                        /* Mobile Hamburger Menu */
                        <div class="hidden max-tablet:flex">
                            // Assuming ThemeToggle is another Yew component
                            // <ThemeToggle /> 
                            <div class="pr-3"></div>
                            <button onclick={toggle_menu} class="focus:outline-none">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="feather feather-menu"
                                >
                                    <line x1="3" y1="12" x2="21" y2="12"></line>
                                    <line x1="3" y1="6" x2="21" y2="6"></line>
                                    <line x1="3" y1="18" x2="21" y2="18"></line>
                                </svg>
                            </button>
                        </div>
                    </div>

                    /* Mobile Menu Items */
                    <div class={mobile_menu_class}>
                        <a href="/" class="block py-2 px-4 hover:text-just-red">
                            { "Home" }
                        </a>
                        <a href="/portfolio" class="block py-2 px-4 hover:text-just-red aria-[current]:text-just-red">
                            { "Portfolio" }
                        </a>
                        <a href="/articles" class="block py-2 px-4 hover:text-just-red aria-[current]:text-just-red">
                            { "Articles" }
                        </a>
                        <a href="/about" class="block py-2 px-4 hover:text-just-red aria-[current]:text-just-red">
                            { "About" }
                        </a>
                    </div>
                </div>
            </nav>
        </div>
    }
}
