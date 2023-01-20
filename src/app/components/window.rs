use yew::prelude::*;

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

        html! {
            <div class="window column" style={format!("width={width}; height={height};")} class={class} id={id}>
                <div class="window-title row flex-center">
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
