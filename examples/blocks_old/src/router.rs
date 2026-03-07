use crate::pages::app_screens::AppScreensView;
use crate::pages::blocks::BlocksView;
use crate::pages::ui_app::{AppGalleryView, UiGalleryView};
use crate::routes::Route;
use wonopui::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
     <div class="grid min-h-screen grid-cols-1 grid-rows-[1fr,auto,1fr] bg-white lg:grid-cols-[max(50%,36rem),1fr]">
       <header class="mx-auto w-full max-w-7xl px-6 pt-6 sm:pt-10 lg:col-span-2 lg:col-start-1 lg:row-start-1 lg:px-8">
         <a href="#">
           <span class="sr-only">{"WonopUI"}</span>
           /*<img class="h-10 w-auto sm:h-12" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="" />*/
         </a>
       </header>
       <main class="mx-auto w-full max-w-7xl px-6 py-24 sm:py-32 lg:col-span-2 lg:col-start-1 lg:row-start-2 lg:px-8">
         <div class="max-w-lg">
           <p class="text-base font-semibold leading-8 text-indigo-600">{"Coming Soon"}</p>
           <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">{"Wonop "}<span class="py-1 px-2 rounded bg-zinc-800 text-white">{"UI"}</span>
           </h1>
           <p class="mt-6 text-base leading-7 text-gray-600">{"Introducing a premium UI set for Rust YEW, crafted with TailwindCSS. Experience the best in design and functionality!"}</p>
         </div>
       </main>
       <footer class="self-end lg:col-span-1 lg:col-start-1 lg:row-start-3">
           <div class="border-t border-gray-100 bg-gray-50 py-10">
               <div class="flex justify-between items-center">
                   <nav class="mx-auto flex w-full max-w-7xl items-center gap-x-4 px-6 text-sm leading-7 text-gray-600 lg:px-8">
                   <a href="https://wonopstudio.com">{"Wonop Studio"}</a>
                   <svg viewBox="0 0 2 2" aria-hidden="true" class="h-0.5 w-0.5 fill-gray-300">
                       <circle cx="1" cy="1" r="1" />
                   </svg>
                   <a href="https://wonopconsulting.com">{"Wonop Consulting"}</a>
                   </nav>
                   <div class="flex space-x-6 md:order-2 px-4">
                   <a href="https://twitter.com/troelsfr" class="text-gray-500 hover:text-gray-400">
                       <span class="sr-only">{"X"}</span>
                       <svg fill="currentColor" viewBox="0 0 24 24" aria-hidden="true" class="h-6 w-6">
                       <path d="M13.6823 10.6218L20.2391 3H18.6854L12.9921 9.61788L8.44486 3H3.2002L10.0765 13.0074L3.2002 21H4.75404L10.7663 14.0113L15.5685 21H20.8131L13.6819 10.6218H13.6823ZM11.5541 13.0956L10.8574 12.0991L5.31391 4.16971H7.70053L12.1742 10.5689L12.8709 11.5655L18.6861 19.8835H16.2995L11.5541 13.096V13.0956Z">
                       </path>
                       </svg>
                   </a>
                   <a href="https://github.com/wonop-io" class="text-gray-500 hover:text-gray-400">
                       <span class="sr-only">{"GitHub"}</span>
                       <svg fill="currentColor" viewBox="0 0 24 24" aria-hidden="true" class="h-6 w-6">
                       <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd">
                       </path>
                       </svg>
                   </a>
               </div>

           </div>

           </div>
       </footer>
       <div class="hidden lg:relative lg:col-start-2 lg:row-start-1 lg:row-end-4 lg:block">
         <img src="https://images.unsplash.com/photo-1558655146-d09347e92766?q=80&w=2564&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="" class="absolute inset-0 h-full w-full object-cover" />
       </div>
     </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PlaceholderProps {
    pub name: String,
}

#[function_component(Placeholder)]
pub fn placeholder(props: &PlaceholderProps) -> Html {
    html! {
        <div>
            <h2>{ "Placeholder Component" }</h2>
            <p>{ format!("This is a placeholder for: {}", props.name) }</p>
        </div>
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    let navigator = yew_router::hooks::use_navigator();
    let _on_close = {
        match navigator {
            Some(history) => {
                let _history = history.clone();
                /*
                Callback::from(move |_| {
                    let history = history.clone();
                    // history.push(&Route::HomePage);
                })
                */
            }
            None => (), // Callback::from(move |_| {}),
        }
    };

    let render = move |routes| match routes {
        Route::HomePage => {
            html! {<HomePage />}
        }
        Route::UiApp => html! { <UiGalleryView /> },
        Route::AppGallery => html! { <AppGalleryView /> },
        Route::ScreensRoot | Route::Screens => html! { <>
            <LayoutProvider>
                <AppScreensView />
            </LayoutProvider>
        </> },
        Route::BlocksRoot | Route::Blocks => html! { <>
            <LayoutProvider>
                <BlocksView />
            </LayoutProvider>
        </> },
    };

    html! { <Switch<Route> render={render} /> }
}
