// use ::yew_router::prelude::*;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
            <>
                <Header />
                <div
                 id="wrapper"
                 class="p-2 mx-auto max-w-3xl flex flex-col justify-center">
                    <h1 class="max-tablet:text-2xl text-4xl">{ "Hello, I'am" }</h1>
                    <h1 class="max-tablet:text-4xl text-6xl font-extrabold font-mordern">
                    {"Abhinandh S"}
                    <span class="text-just-red">{"."}</span>
                    </h1>
                    <h1 class="pt-8 text-2xl font-sans font-bold">
                    { "Welcome to my corner of Internet"}
                  <span class="text-just-red">
                  {"."}
                  </span>
                </h1>
                <h1 class="border-l-4 border-l-just-red pl-4 font-bold max-tablet:text-3xl text-4xl mt-12">
                { "About Me" }
                  <span class="text-just-red">
                  {"."}
                  </span>
                </h1>
                <br />
    <p>
    { "I'm Abhinandh S. I am a 21 old guy from India, who loves computers
                  and softwares." }
                </p>
                <p>
    { "This place is home for all my psychological dysfunctioning. A
                  place where I am in control, with no censorship or manupilation." }
                </p>

                <div id="list articles" class="">
                  <h1 class="border-l-4 border-l-just-red pl-4 font-bold max-tablet:text-3xl text-4xl mt-12">
    { "Recent Posts"}
                    <span class="text-just-red">
    { "." }
                    </span>
                  </h1>
                  <ul class="mt-8">
           //         {posts.map((post) => <PostEntry post={post} />)}
                  </ul>
                </div>

    <div className="border-b broder-latte-text dark:border-mocha-text">
                </div>

               <Footer />
                </div>
            </>
        }
}
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

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="text-center clear-both">
        <div>
          <br />
          <br />
          <div class="flex justify-center">
            <a
              class="p-2"
              href="https://github.com/abhi-xyz/"
              target="_blank"
            >
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
                class="feather feather-github"
              >
                <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22">
                </path>
              </svg>
            </a>
            <a class="p-2" href="mailto:abhinandhsuby@proton.me">
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
                class="feather feather-mail"
              >
                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z">
                </path>
                <polyline points="22,6 12,13 2,6"></polyline>
              </svg>
            </a>
            <a class="p-2" href="/feed.atom" target="_blank">
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
                class="feather feather-rss"
              >
                <path d="M4 11a9 9 0 0 1 9 9"></path>
                <path d="M4 4a16 16 0 0 1 16 16"></path>
                <circle cx="5" cy="19" r="1"></circle>
              </svg>
            </a>
            <a class="p-2" href="https://x.com/Ungraduate_Abhi" target="_blank">
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
                class="feather feather-twitter"
              >
                <path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z">
                </path>
              </svg>
            </a>
          </div>
          <p class="p-3">
          { "This site is built with the " }
            <a
              class="text-just-red"
              // href="https://fresh.deno.dev"
            >
            { "Fresh"}
            </a>{" "}
            { "framework on Deno. The website’s source code is licensed under the"}
            {" "}
            <a
              class="text-just-red"
              // href="https://opensource.org/license/mit"
            >
            { "MIT License" }
            </a>{" "}
            {"and is available"} {" "}
            <a
              class="text-just-red"
              // href="https://github.com/abhi-xyz/abhinandhs.in"
            >
            { "here" }
            </a>{". Articles are licensed under"}{" "}
            <a
              class="text-just-red"
              // href="https://creativecommons.org/licenses/by-sa/4.0/deed.en"
            >
            {"Creative Commons"}
            </a>{" "}
            {"with the Share-alike Clause (CC-BY-SA 4.0)."}
          </p>
          <br />
          <br />
        </div>
      </footer>
    }
}
