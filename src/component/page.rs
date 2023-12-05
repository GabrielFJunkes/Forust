use axum::response::IntoResponse;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::auth::structs::UserJWT;

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

fn nav(logged: Option<UserJWT>) -> Markup {
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
                    @if let Some(user) = logged {
                        div class="text-xl text-gray-800 md:text-base group cursor-pointer w-fit" {
                            a href=(format!("/u/{}", user.nome)) class="text-gray-800 text-base block" { "Meu perfil" }
                            div class="hidden group-hover:block absolute mb-1 bg-white border border-gray-200 shadow-lg" {
                                a href="/api/auth/logout" class="block px-3 py-1 text-sm text-gray-800 hover:bg-gray-100 inline-flex flex"{
                                    ("Logout")
                                    svg xmlns="http://www.w3.org/2000/svg" 
                                    fill="none" 
                                    viewBox="0 0 24 24" 
                                    stroke-width="1.5" 
                                    stroke="currentColor" 
                                    class="w-5 h-5 mr-1"{
                                        path stroke-linecap="round" 
                                        stroke-linejoin="round" 
                                        d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" 
                                        {}
                                    }

                                }
                            }
                        }
                    }@else {
                        a href="/login" class="text-xl text-gray-800 md:text-base mr-2" { "Login" }
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

fn notification(cookie: (String, String)) -> Markup {
    let class = "z-10 peer-checked:hidden fade-out absolute right-2 top-2 flex flex-row p-2 max-w-xs text-sm text-white rounded-md shadow-lg mb-3 ml-3 ".to_owned() + &cookie.0;
    html!(
        input type="checkbox" id="toggle" class="hidden peer" {}
        div 
        class=(class)
        role="alert" {
            div class="flex" {
                {(cookie.1)}
            }

            div class="flex ml-auto items-center" {
                label for="toggle"
                class="inline-flex flex-shrink-0 justify-center 
                items-center h-4 w-4 rounded-md text-white/[.5] 
                hover:text-white focus:outline-none focus:ring-2 
                focus:ring-offset-2 focus:ring-offset-gray-800 
                focus:ring-gray-600 transition-all 
                text-sm dark:focus:ring-offset-gray-900 dark:focus:ring-gray-800" {
                    svg 
                    class="w-3.5 h-3.5" 
                    width="16" 
                    height="16" 
                    viewBox="0 0 16 16" 
                    fill="none" 
                    xmlns="http://www.w3.org/2000/svg" {
                        path d="M0.92524 0.687069C1.126 0.486219 1.39823 0.373377 1.68209 0.373377C1.96597 0.373377 2.2382 0.486219 2.43894 0.687069L8.10514 6.35813L13.7714 0.687069C13.8701 0.584748 13.9882 0.503105 14.1188 0.446962C14.2494 0.39082 14.3899 0.361248 14.5321 0.360026C14.6742 0.358783 14.8151 0.38589 14.9468 0.439762C15.0782 0.493633 15.1977 0.573197 15.2983 0.673783C15.3987 0.774389 15.4784 0.894026 15.5321 1.02568C15.5859 1.15736 15.6131 1.29845 15.6118 1.44071C15.6105 1.58297 15.5809 1.72357 15.5248 1.85428C15.4688 1.98499 15.3872 2.10324 15.2851 2.20206L9.61883 7.87312L15.2851 13.5441C15.4801 13.7462 15.588 14.0168 15.5854 14.2977C15.5831 14.5787 15.4705 14.8474 15.272 15.046C15.0735 15.2449 14.805 15.3574 14.5244 15.3599C14.2437 15.3623 13.9733 15.2543 13.7714 15.0591L8.10514 9.38812L2.43894 15.0591C2.23704 15.2543 1.96663 15.3623 1.68594 15.3599C1.40526 15.3574 1.13677 15.2449 0.938279 15.046C0.739807 14.8474 0.627232 14.5787 0.624791 14.2977C0.62235 14.0168 0.730236 13.7462 0.92524 13.5441L6.59144 7.87312L0.92524 2.20206C0.724562 2.00115 0.611816 1.72867 0.611816 1.44457C0.611816 1.16047 0.724562 0.887983 0.92524 0.687069Z" fill="currentColor" {}
                    }
                }
            }
        }
        (PreEscaped("
        <style>
        .fade-out {
          opacity: 1;
          animation: fadeOut ease-in 1s;
          animation-fill-mode: forwards;
          animation-delay: 2s;
        }
        
        @keyframes fadeOut {
          100% {
            opacity: 0;
          }
        }
        </style>
        "))
    )
}

pub fn is_logged_in(cookie: Option<&Cookie>) -> bool {
    match cookie {
        Some(cookie) => {
            !cookie.value().is_empty()
        },
        None => false,
    }
}

pub fn is_logged_in_with_data(cookie: Option<&Cookie>) -> Option<UserJWT> {
    match cookie {
        Some(cookie) => {
            if !cookie.value().is_empty() {
                let token = match cookie.value().split("=").next() {
                    Some(str) => str,
                    None => "",
                };
                match decode::<UserJWT>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256)) {
                    Ok(data) => { Some(data.claims) },
                    Err(_) => { None }
                }
            }else{
                None
            }
        },
        None => None,
    }
}

fn consume_notification_cookie(jar: CookieJar) -> (Option<(String, String)>, CookieJar) {
    let mut notification: Option<(String, String)> = None;
    let new_jar: CookieJar;
    if let Some(cookie) = jar.get("success_msg") {
        notification = Some(("bg-green-500".to_string(), cookie.value().to_owned().to_string()));
        let mut cookie = Cookie::named("success_msg");
        cookie.set_path("/");
        new_jar = jar.remove(cookie);
        
    }else if let Some(cookie) = jar.get("error_msg") {
        notification = Some(("bg-red-500".to_string(), cookie.value().to_owned().to_string()));
        let mut cookie = Cookie::named("error_msg");
        cookie.set_path("/");
        new_jar = jar.remove(cookie);
    }else{
        new_jar = jar;
    }
    (notification, new_jar)
}

pub async fn build_page(title: &str, content: Markup, jar: CookieJar) -> impl IntoResponse {
    let logged_in = is_logged_in_with_data(jar.get("session_jwt"));
    let (option_noti, _jar) = consume_notification_cookie(jar);
    let html = html!(
        (DOCTYPE);
        (header(title));
        body class="flex flex-col min-h-screen" {
            (nav(logged_in));
            div class="relative flex flex-grow" {
                @if let Some(noti) = option_noti {
                    (notification(noti));
                }
                (content);
            }
            (footer());
        }
    );
    html
}