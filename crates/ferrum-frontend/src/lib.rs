use leptos::*;

pub fn mount_app() {
    mount_to_body(|| {
        view! {
            <div>
                <h1>"Ferrum Frontend"</h1>
                <p>"Frontend library is loading..."</p>
            </div>
        }
    });
}
