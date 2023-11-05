

use lazy_static::lazy_static;
use leptos::{*};

use crate::state::ValueSetter;


lazy_static! {
    static ref LINKS: Vec<(&'static str, &'static str)> = vec![("", "About Me"), ("hireme", "Hire Me"), ("https://blog.itsparser.in", "Blog")];
}

#[component]
pub fn TopNav() -> impl IntoView {
    let state = use_context::<RwSignal<ValueSetter>>().expect("state to have been provided");
    let (_current_path, set_current_path) = create_slice(
        state,
        |state| state.clone().current_path,
        |state, n| state.current_path = n,
    );
    view! {
        <header class="flex flex-wrap sm:justify-start sm:flex-nowrap z-50 w-full bg-white text-sm py-4 bg-gradient-to-b from-indigo-950 to-indigo-900">
        //  from-sky-500
            <nav class="max w-full mx-auto px-4 sm:flex sm:items-center sm:justify-between" aria-label="Global">
                <div class="flex items-center justify-between">
                <a class="flex-none text-xl font-semibold dark:text-white" href="/">Itsparser</a>
                <div class="sm:hidden">
                    <button type="button" class="hs-collapse-toggle p-2 inline-flex justify-center items-center gap-2 rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-slate-900 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800" data-hs-collapse="#navbar-collapse-with-animation" aria-controls="navbar-collapse-with-animation" aria-label="Toggle navigation">
                        <svg class="hs-collapse-open:hidden w-4 h-4" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/>
                        </svg>
                        <svg class="hs-collapse-open:block hidden w-4 h-4" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                        </svg>
                    </button>
                </div>
                </div>
                <div id="navbar-collapse-with-animation" class="hs-collapse hidden overflow-hidden transition-all duration-300 basis-full grow sm:block">
                    <div class="flex flex-col gap-5 mt-5 sm:flex-row sm:items-center sm:justify-end sm:mt-0 sm:pl-5">
                        {LINKS.clone().into_iter()
                            .map(|(key, value)| {
                                let path = move |key: String| {key.eq(&_current_path())};
                                let mut cls = "font-medium ";
                                if path(key.to_string()){
                                    cls = "font-medium text-blue-500";
                                } else {
                                    cls = "font-medium text-gray-600 hover:text-gray-400 dark:text-gray-400 dark:hover:text-gray-500";
                                }
                                view! { <a class=cls href=key on:click=move |_| { set_current_path(key.to_string()) }>{value}</a>}
                                
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </nav>
        </header>
    }
}