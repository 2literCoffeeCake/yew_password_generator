use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yew::{html, Component, Context, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct NumericTextBoxProps {
    pub value: u8,
    pub min_value: Option<u8>,
    pub max_value: Option<u8>,
    pub on_change: Callback<u8>,
}

#[derive(Debug, Default)]
pub struct NumericTextBox {
    min_value: u8,
    max_value: u8
}

pub enum Msg {
    OnValueChange(u8),
    Increment,
    Decrement,
}

impl Component for NumericTextBox {
    type Message = Msg;

    type Properties = NumericTextBoxProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            min_value: props.min_value.unwrap_or(u8::MIN),
            max_value: props.max_value.unwrap_or(u8::MAX),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let NumericTextBoxProps {
            mut value,
            on_change,
            min_value: _,
            max_value: _
        } = ctx.props().clone();
        value = match msg {
            Msg::OnValueChange(value) => value,
            Msg::Increment => value + 1,
            Msg::Decrement => value - 1,
        };
        if value < self.min_value {
            value = self.min_value;
        }
        if value > self.max_value{
            value = self.max_value;
        }
        on_change.emit(value);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let on_decrement_click = ctx.link().callback(|_| Msg::Decrement);
        let on_increment_click = ctx.link().callback(|_| Msg::Increment);

        let on_input_change = ctx.link().callback(|event: Event| {
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            let value = target.value_as_number().floor();
            Msg::OnValueChange(value as u8)
        });

        let dec_enabled = props.value > self.min_value;
        let inc_enabled = props.value < self.max_value;

        html! {
            <div class={"numeric_text_box_wrapper"}>
                <div class={&get_button_classes(dec_enabled)} onclick={on_decrement_click}>
                    <svg viewBox="0 0 384 512"><path d="M376 232H8c-4.42 0-8 3.58-8 8v32c0 4.42 3.58 8 8 8h368c4.42 0 8-3.58 8-8v-32c0-4.42-3.58-8-8-8z"/></svg>
                </div>
                <input type="number" value={props.value.to_string()} onchange={on_input_change}/>
                <div class={&get_button_classes(inc_enabled)} onclick={on_increment_click}>
                    <svg viewBox="0 0 384 512"><path d="M376 232H216V72c0-4.42-3.58-8-8-8h-32c-4.42 0-8 3.58-8 8v160H8c-4.42 0-8 3.58-8 8v32c0 4.42 3.58 8 8 8h160v160c0 4.42 3.58 8 8 8h32c4.42 0 8-3.58 8-8V280h160c4.42 0 8-3.58 8-8v-32c0-4.42-3.58-8-8-8z"/></svg>
                </div>
            </div>
        }
    }
}

fn get_button_classes(enabled: bool) -> String{
    let mut classes = String::from("numeric_text_box_wrapper__button numeric_text_box_wrapper__button-");
    if enabled {
        classes.push_str("enabled");
    } else {
        classes.push_str("disabled");
    };
    classes
} 
