
use leptos::*;
use leptos_router::*;
use crate::layout::DefaultLayout;

use crate::components::header::TopNav;
use crate::route::{home::Home, hireme::HireMe};
use crate::state::ValueSetter;

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(ValueSetter::default()));
    view! {
        <div id="root">
            <TopNav />
            <Router>
                <Routes>
                    <Route path="" view=DefaultLayout>
                        <Route
                            path=""
                            view=Home
                        />
                        <Route
                            path="hireme"
                            view=HireMe
                        />
                    </Route>

                </Routes>
            </Router>
        </div>
    }
}