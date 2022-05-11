use yew::{html, Component, Context, Html};

use super::check_box::CheckBox;

use crate::browser_util;

pub enum Msg {
    OnValueChange((bool, u8)),
}

#[derive(Debug, Default)]
pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnValueChange((checked, value)) => {
                browser_util::console_log(&format!("checked: {checked}; value: {value}"))
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::OnValueChange);

        let label = "Use upper case letters".to_owned();

        html! {
            <div class="layout">
                <div style="grid-column: 2/4; grid-row: 2/3;">
                    <label style="font-size: x-large;">{"WASM password generator"}</label>
                </div>
                <CheckBox on_change={on_change} checked={true} row={3} label={label} value={8}/>
            </div>
        }
    }
}
