use yew::{html, Component, Context, Html};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct NumericTextBoxProps {
    pub value: u16,
    pub on_change: Callback<u16>,
}

#[derive(Debug, Default)]
pub struct NumericTextBox;

pub enum Msg{
    OnValueChange(u16),
    Increment,
    Decrement,
}

impl Component for NumericTextBox{
    type Message = Msg;

    type Properties = NumericTextBoxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let NumericTextBoxProps { mut value, on_change } = ctx.props().clone();
        value = match msg {
            Msg::OnValueChange(value) => value,
            Msg::Increment => value + 1,
            Msg::Decrement => value -1,
        };
        on_change.emit(value);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let on_decrement_click = ctx.link().callback(|_| Msg::Decrement);
        let on_increment_click = ctx.link().callback(|_| Msg::Increment);

        let on_input_change = ctx.link().callback(|event: Event|{
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            let value = target.value_as_number().floor();
            Msg::OnValueChange(value as u16)
        });

        html! {
            <div class={"numeric_text_box_wrapper"}>
                <div class={"numeric_text_box_wrapper__button"} onclick={on_decrement_click}>
                {"-"}
                </div>
                <input type="number" value={props.value.to_string()} onchange={on_input_change}/>
                <div class={"numeric_text_box_wrapper__button"} onclick={on_increment_click}>
                {"+"}
                </div>
            </div>
        }
    }
}