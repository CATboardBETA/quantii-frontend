use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Properties)]
pub struct WindowProps {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub children: Children,
    pub onclose: Callback<MouseEvent, ()>,
    pub onminimize: Callback<MouseEvent, ()>,
    pub onmaximize: Callback<MouseEvent, ()>,
    #[prop_or("".to_owned())]
    pub class: String,
    #[prop_or("".to_owned())]
    pub id: String,
}

pub struct Window;

impl Component for Window {
    type Message = ();
    type Properties = WindowProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let width = ctx.props().width;
        let height = ctx.props().height;
        let title = ctx.props().title.clone();
        let children = ctx.props().children.clone();
        let onclose = ctx.props().onclose.clone();
        let onminimize = ctx.props().onminimize.clone();
        let onmaximize = ctx.props().onmaximize.clone();
        let class = ctx.props().class.clone();
        let id = ctx.props().id.clone();
        let header_id = crate::app::random_id(rand::thread_rng());

        // Make the window draggable
        let onmousedown = Callback::from(move |down_event: MouseEvent| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let window_element = document.get_element_by_id(&id).unwrap();
            let header_element = document.get_element_by_id(&header_id).unwrap();

            let window_rect = window_element.get_bounding_client_rect();
            let header_rect = header_element.get_bounding_client_rect();

            let x = down_event.client_x() - window_rect.left() as i32;
            let y = down_event.client_y() - window_rect.top() as i32;

            let onmousemove = Callback::from(move |move_event: MouseEvent| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let window_element = document.get_element_by_id(&id).unwrap();

                let left = move_event.client_x() - x;
                let top = move_event.client_y() - y;

                window_element.set_style("left", &format!("{}px", left)).unwrap();
                window_element.set_style("top", &format!("{}px", top)).unwrap();
            });
            
            let onmouseup = Callback::from(move |up_event: MouseEvent| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let window_element = document.get_element_by_id(&id).unwrap();

                let left = up_event.client_x() - x;
                let top = up_event.client_y() - y;

                window_element.set_style("left", &format!("{}px", left)).unwrap();
                window_element.set_style("top", &format!("{}px", top)).unwrap();
            });
        });

        // We need to re-clone the ID here because we moved it into the `onmousedown` callback
        let id = ctx.props().id.clone();

        html! {
            <div class="window column" style={format!("width={width}px; height={height}px;")} class={class} id={id}>
                <div class={format!("window-title row flex-center {}", header_id)}>
                    <div class="window-title-text">{title}</div>
                    <div class="window-title-button window-title-button-close">
                        <button type="button" onclick={onclose} class="window-title-button-close-icon dot no-padding no-margin"></button>
                    </div>
                    <div class="window-title-button window-title-button-minimize">
                        <button type="button" onclick={onminimize} class="window-title-button-minimize-icon dot no-padding no-margin"></button>
                    </div>
                    <div class="window-title-button window-title-button-maximize">
                        <button type="button" onclick={onmaximize} class="window-title-button-maximize-icon dot no-padding no-margin"></button>
                    </div>
                </div>
                <div class="window-content container">
                    {children}
                </div>
            </div>
        }
    }
}
