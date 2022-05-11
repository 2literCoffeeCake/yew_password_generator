use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CheckBoxProps {
    pub label: String,
    pub row: u8,
    pub checked: bool,
    pub on_change: Callback<(bool, u8)>,
    pub value: u8
}

fn get_value_from_input_event(e: InputEvent, value: u8) -> (bool, u8) {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    (target.checked(), value)
}

#[function_component(CheckBox)]
pub fn render_check_box(props: &CheckBoxProps) -> Html {
    let CheckBoxProps { label, row, checked, on_change, value } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event, value));
    });

    let style_1 = format!("grid-column: 2/3; grid-row: {}/{};", row, row + 1);
    let style_2 = format!("grid-column: 3/4; grid-row: {}/{};", row, row + 1);

    html! {
        <>
            <div style={style_1}>
                <label>{label}</label>
            </div>
            <div style={style_2}>
                <input type="checkbox" checked={checked} {oninput}/>
            </div>
        </>
    }
}