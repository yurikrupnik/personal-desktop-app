use leptos::leptos_dom::ev::{SubmitEvent, MouseEvent};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// #[component]
// pub fn SimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
//   // create a reactive signal with the initial value
//   let (value, set_value) = create_signal(cx, initial_value);
//   let (name, set_name) = create_signal(cx, "yuri");
//   // create event handlers for our buttons
//   // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
//   let clear = move |_| set_value.set(0);
//   let decrement = move |_| set_value.update(|value| *value -= 1);
//   let increment = move |_| set_value.update(|value| *value += 1);
//
//   // this JSX is compiled to an HTML template string for performance
//   view! {
//         cx,
//         <div>
//             <button on:click=clear>"Clear"</button>
//             <button on:click=decrement>"-1"</button>
//             <span>"Value: " {move || value().to_string()} "!"</span>
//             <button on:click=increment>"+1"</button>
//         </div>
//     }
// }

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (last_name, set_last_name) = create_signal(cx, String::new());
    let (first_name, set_first_name) = create_signal(cx, String::new());

    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());
    let (cluster_msg, set_cluster_msg) = create_signal(cx, String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };
    let cluster = move |ev: MouseEvent| {
      ev.prevent_default();
      spawn_local(async move {
        if name.get().is_empty() {
          return;
        }

        let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        let new_msg = invoke("kind", args).await.as_string().unwrap();
        set_cluster_msg.set(new_msg);
      });
    };

    view! { cx,
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Hello there from Leptos!!"</p>
            <p>
                "Recommended IDE setup: "
                <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
                " + "
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
                " + "
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            </p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>
            <p><b>{ move || cluster_msg.get() }</b></p>
            <button on:click=cluster>"Create Cluster"</button>
        </main>
    }
}
