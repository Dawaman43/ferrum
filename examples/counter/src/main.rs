//! Counter Example - Demonstrating Ferrum's core capabilities
//! 
//! This example showcases:
//! - Component creation with state management
//! - CSS-in-Rust styling with utility classes
//! - Event handling and reactivity
//! - Type-safe operations

use ferrum_core::{component, state::create_signal, css, css::UtilityClass};
use ferrum_shared::User;
use leptos::*;

fn main() {
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("failed to init log");
    
    // Mount the app to the body
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Create reactive state
    let (count, set_count) = create_signal(0);
    let (name, set_name) = create_signal("Ferrum".to_string());
    
    // Create a user state
    let user = create_signal(User {
        id: uuid::Uuid::new_v4(),
        username: "ferrum_dev".to_string(),
        email: "dev@ferrum.dev".to_string(),
        created_at: chrono::Utc::now(),
    });
    
    view! {
        <div class={css!(Flex, FlexCol, ItemsCenter, JustifyCenter, HFull, P(8))}>
            <div class={css!(Border2, RoundedLg, P(6), ShadowLg, WAuto)}>
                // Header with dynamic greeting
                <header class={css!(TextXl, FontBold, TextGray800, Mb(4))}>
                    "Welcome to " {move || name.get()}
                </header>
                
                // Counter component demonstration
                <Counter count=count set_count=set_count />
                
                // User information display
                <UserInfo user=user />
                
                // Interactive name changer
                <NameChanger name=name set_name=set_name />
                
                // Footer with current count
                <footer class={css!(Mt(6), TextSm, TextGray800)}>
                    "Current count: " {move || count.get()}
                </footer>
            </div>
        </div>
    }
}

#[component]
fn Counter(
    count: ReadSignal<i32>,
    set_count: WriteSignal<i32>,
) -> impl IntoView {
    // CSS for counter buttons
    let button_style = css!(
        Border, Rounded, P(2), M(1),
        BgBlue500, TextWhite,
        Hover(Style::new().property("transform", "scale(1.05)"))
    );
    
    view! {
        <div class={css!(Flex, ItemsCenter, JustifyBetween, Mb(4))}>
            <button 
                class={button_style.clone()}
                on:click=move |_| set_count.update(|n| *n -= 1)
            >
                "−"
            </button>
            
            <span class={css!(TextLg, FontBold, P(2))}>
                {move || count.get()}
            </span>
            
            <button 
                class={button_style}
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "+"
            </button>
        </div>
    }
}

#[component]
fn UserInfo(
    user: ReadSignal<User>,
) -> impl IntoView {
    view! {
        <div class={css!(BgGray100, Rounded, P(4), Mb(4))}>
            <h3 class={css!(FontBold, TextGray800, Mb(2))}>
                "User Information"
            </h3>
            <p class={css!(TextSm, TextGray600)}>
                "ID: " {move || user.get().id.to_string()}
            </p>
            <p class={css!(TextSm, TextGray600)}>
                "Username: " {move || user.get().username}
            </p>
            <p class={css!(TextSm, TextGray600)}>
                "Email: " {move || user.get().email}
            </p>
            <p class={css!(TextSm, TextGray600)}>
                "Created: " {move || user.get().created_at.format("%Y-%m-%d").to_string()}
            </p>
        </div>
    }
}

#[component]
fn NameChanger(
    name: ReadSignal<String>,
    set_name: WriteSignal<String>,
) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(String::new());
    
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let new_name = input_value.get();
        if !new_name.is_empty() {
            set_name.set(new_name);
            set_input_value.set(String::new());
        }
    };
    
    view! {
        <form on:submit=on_submit class={css!(Flex, FlexCol, Gap(2))}>
            <input 
                type="text"
                placeholder="Enter new name"
                class={css!(Border, Rounded, P(2), TextBase)}
                prop:value=move || input_value.get()
                on:input=move |ev| {
                    set_input_value.set(event_target_value(&ev));
                }
            />
            
            <button 
                type="submit"
                class={css!(BgGreen500, TextWhite, Rounded, P(2), FontBold)}
            >
                "Change Name"
            </button>
        </form>
    }
}