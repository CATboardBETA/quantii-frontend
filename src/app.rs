#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;

use components::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {

    let launch_window = || {

    };
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

    html!{
        <div class="row no-padding no-margin">
            <Sidebar items={items} />
            <Desktop>
                <Window width=500 height=500 title="Test" onclose={onclose} onminimize={onminimize} onmaximize={onmaximize}>
                    <div>
                        {"Hello, world!"}
                    </div>
                </Window>
            </Desktop>
        </div>
    }
}
