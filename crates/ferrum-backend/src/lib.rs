use leptos::*;
use ferrum_core::*;

pub fn render_app() -> impl IntoView {
    view! {
        <div class="ferrum-app">
            <h1>"Ferrum Backend"</h1>
            <p>"Backend services are running..."</p>
        </div>
    }
}