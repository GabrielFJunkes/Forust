pub mod form;

use maud::{html, Markup, PreEscaped, DOCTYPE};

fn header(page_title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { (page_title) };
            (PreEscaped("<script src='https://cdn.tailwindcss.com'></script>"));
        }
    }
}

fn nav(logged: bool) -> Markup {
    html!(
        nav class="px-6 py-4 bg-white shadow" {
            div class="flex justify-between place-items-center" {
                div class="basis-1/4" {
                    a href="/" class="text-xl font-bold text-gray-800 md:text-2xl" { "Forust" }
                }
                div class="basis-2/4 relative mx-auto" {
                    input 
                    class="border-2 border-gray-300 bg-white h-8 w-full px-5 pr-16 rounded-lg text-sm focus:outline-none" 
                    type="search"
                    name="search"
                    placeholder="Search" {}
                    button type="submit" class="absolute right-2 top-2" {
                        svg
                        class="text-gray-600 h-4 w-4 fill-current"
                        xmlns="http://www.w3.org/2000/svg"
                        xmlns:xlink="http://www.w3.org/1999/xlink"
                        version="1.1"
                        id="Capa_1"
                        x="0px"
                        y="0px"
                        viewBox="0 0 56.966 56.966"
                        style="enable-background: new 0 0 56.966 56.966"
                        xml:space="preserve"
                        width="512px"
                        height="512px" {
                            path d="M55.146,51.887L41.588,37.786c3.486-4.144,5.396-9.358,5.396-14.786c0-12.682-10.318-23-23-23s-23,10.318-23,23  s10.318,23,23,23c4.761,0,9.298-1.436,13.177-4.162l13.661,14.208c0.571,0.593,1.339,0.92,2.162,0.92  c0.779,0,1.518-0.297,2.079-0.837C56.255,54.982,56.293,53.08,55.146,51.887z M23.984,6c9.374,0,17,7.626,17,17s-7.626,17-17,17  s-17-7.626-17-17S14.61,6,23.984,6z" {}
                        }
                    }
                }
                  
                div dir="rtl" class="basis-1/4" {
                    @if logged {
                        a href="#" class="text-xl text-gray-800 md:text-base mr-2" { "Meu perfil" }
                    }@else {
                        a href="login" class="text-xl text-gray-800 md:text-base mr-2" { "Login" }
                    }
                }
            }
        }
    )
}

fn footer() -> Markup {
    html! {
        footer class="px-6 py-2 text-gray-100 bg-gray-800 w-full" {
            div class="container flex flex-col items-center justify-between mx-auto md:flex-row" {
                a href="/" class="text-2xl font-bold" { "Forust" }
                p class="mt-2 md:mt-0 text-center text-white text-xs max-w-md" {
                    {(PreEscaped("&copy;")) "2023 Forust. All rights reserved."}
                }
            }
        }
    }
}

pub async fn build_page(title: &str, content: Markup) -> Markup {
    html!(
        (DOCTYPE);
        (header(title));
        body class="flex flex-col min-h-screen" {
            (nav(false));
            (content);
            (footer());
        }
    )
}