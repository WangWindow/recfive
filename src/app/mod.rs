mod home;

use home::Home;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen p-4 bg-gray-50">
            <div class="my-8 w-full max-w-2xl">
                <Home />
            </div>
        </div>
    }
}
