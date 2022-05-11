use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct NumericTextBoxProps {
    pub label: String,
    pub value: u16,
    pub on_change: Callback<f64>,
}

fn get_value_from_input_event(e: InputEvent) -> f64 {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value_as_number()
}

#[function_component(NumericTextBoxBox)]
pub fn render_numeric_text_box(props: &NumericTextBoxProps) -> Html {
    let NumericTextBoxProps { label, value, on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <>
            <div style={"grid-column: 2/3; grid-row: 7/8;"}>
                <label>{label}</label>
            </div>
            <div style={"grid-column: 3/4; grid-row: 7/8;"}>
                <input type="number" value={value.to_string()} {oninput}/>
            </div>
        </>
    }
}