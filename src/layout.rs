
use leptos::*;
use leptos_router::*;

#[component]
pub fn DefaultLayout() -> impl IntoView {
  view! {
    <div>
      <Outlet/>
    </div>
  }
}