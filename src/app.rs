#![allow(non_snake_case)]

use rand::prelude::ThreadRng;
use rand::RngCore;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

mod components;
use components::*;

#[derive(Store, PartialEq, Default)]
struct UsedIds { ids: Vec<u32> }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    let thread_rng: ThreadRng = rand::thread_rng();

    let launch_window = || {};
    let home_callback: Callback<MouseEvent, ()> = Callback::from(move |_| {
        log("Home");
        launch_window()
    });

    let items = vec![
        SidebarItemProps {
            name: AttrValue::from("home"),
            icon: AttrValue::from("public/home.svg"),
            callback: home_callback,
        },
        SidebarItemProps {
            name: AttrValue::from("settings"),
            icon: AttrValue::from("public/settings.svg"),
            callback: Callback::noop(),
        },
    ];

    let onclose = Callback::noop();
    let onminimize = Callback::noop();
    let onmaximize = Callback::noop();

    html! {
        <div class="row no-padding no-margin">
            <Sidebar items={items} />
            <Desktop class="desktop no-padding no-margin">
                <Window
                    width=500
                    height=500
                    title="Test"
                    onclose={onclose}
                    onminimize={onminimize}
                    onmaximize={onmaximize}
                    id={random_id(thread_rng)}
                >
                    <div>
                        {"Hello, world!"}
                    </div>
                </Window>
            </Desktop>
        </div>
    }
}

fn random_id(mut rng: ThreadRng) -> String {
    let (usedids, set_usedids) = use_store::<UsedIds>();
    let id: u32 = rng.next_u32();
    let not_foundid: bool = true;
    while not_foundid {
        if usedids.ids.contains(&id) {
            let id: u32 = rng.next_u32();
        } else {
            set_usedids.reduce_mut_callback(|used_ids| used_ids.ids.push(id));
            let not_foundid: bool = false;
        }
    }
    id.to_string()
}