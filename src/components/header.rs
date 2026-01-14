use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
              <>
                   <div class="font-bold">
          <nav class="w-full min-h-32 max-tablet:min-h-16 top-0 left-0 z-10">
            <div class="mx-auto px-4">
              <div class="flex justify-end items-center pt-8 max-tablet:py-4">
    //          {/* Desktop Menu */}
                <div class="flex max-tablet:hidden space-x-16 mt-12 pb-7 px-16">
                  // <ThemeToggle />
                  <a
                   // href="/"
                    class="hover:text-just-red aria-[current='page']:text-just-red"
                  >
                  { "Home" }
                  </a>
                  <a
                    // href="/portfolio"
                    class="hover:text-just-red aria-[current]:text-just-red"
                  >
                  { "Portfolio" }
                  </a>
                  <a
                    // href="/articles"
                    class="hover:text-just-red aria-[current]:text-just-red"
                  >
                  { "Articles" }
                  </a>
                  <a
                    // href="/about"
                    class="hover:text-just-red aria-[current]:text-just-red"
                  >
                  { "About" }
                  </a>
                </div>
                      </div>
                      </div>

          </nav>
        </div>

          </>
          }
}

