use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="text-center py-10 px-4 sm:px-6 lg:px-8">
            <h1 class="block text-2xl font-bold  sm:text-4xl animate-text bg-gradient-to-r from-teal-500 via-purple-500 to-orange-500 bg-clip-text text-transparent">Itsparser.</h1>
            <p class="mt-5 text-lg text-[#F8F9FA]">
                Fullstack Engineer // Engineering Orchestrator // Gamer
            </p>
        </div>
    }
}