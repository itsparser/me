use leptos::*;

#[component]
pub fn HireMe() -> impl IntoView {
    view! {
        <div class="text-center py-10 px-4 sm:px-6 lg:px-8">
            <h2 class="text-2xl font-semibold mt-4">Why Hire Me?</h2>
            <ul class="list-disc list-inside">
                <li>Expertise in HTML, CSS, and JavaScript.</li>
                <li>Proficient in various web frameworks and libraries.</li>
                <li>Strong problem-solving and debugging skills.</li>
                <li>Excellent communication and teamwork abilities.</li>
                <li>Proven track record of delivering high-quality projects on time.</li>
            </ul>
            <h2 class="text-2xl font-semibold mt-4">Contact Me</h2>
            <p class="text-lg">
                If your interested in working with me or have any questions, please feel free to reach out:
            </p>
            <ul class="list-disc list-inside">
                <li>Email:mail@itsparser.in</li>
            </ul>
        </div>
    }
}
