use yew::{html, Component, Context, Html};

use crate::{
    browser_util,
    password_builder::{generate_random_password, PasswordOptions},
};

use super::{CheckBox, NumericTextBoxBox};

static UPPER_CASE_KEY: &str = "upper_value";
static LOWER_CASE_KEY: &str = "lower_value";
static NUMERIC_KEY: &str = "numeric_value";
static SPECIAL_KEY: &str = "special_value";
static LENGHT_KEY: &str = "lenght_value";

pub enum Msg {
    OnUpperChange(bool),
    OnLowerChange(bool),
    OnNumericChange(bool),
    OnSpecialChange(bool),
    OnLenghtChange(f64),
    OnClick,
}

#[derive(Debug, Default)]
pub struct App {
    use_upper: bool,
    use_lower: bool,
    use_numeric: bool,
    use_special: bool,
    lenght: u16,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            use_lower: read_checked_from_client_storage(LOWER_CASE_KEY),
            use_numeric: read_checked_from_client_storage(NUMERIC_KEY),
            use_special: read_checked_from_client_storage(SPECIAL_KEY),
            use_upper: read_checked_from_client_storage(UPPER_CASE_KEY),
            lenght: get_lenght_from_client_storage(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnUpperChange(checked) => {
                save_checked_in_client_storage(UPPER_CASE_KEY, checked);
                self.use_upper = checked;
            }
            Msg::OnLowerChange(checked) => {
                save_checked_in_client_storage(LOWER_CASE_KEY, checked);
                self.use_lower = checked;
            }
            Msg::OnNumericChange(checked) => {
                save_checked_in_client_storage(NUMERIC_KEY, checked);
                self.use_numeric = checked;
            }
            Msg::OnSpecialChange(checked) => {
                save_checked_in_client_storage(SPECIAL_KEY, checked);
                self.use_special = checked;
            }
            Msg::OnLenghtChange(mut lenght) => {
                lenght = lenght.floor();
                if lenght < 4.0 {
                    lenght = 4.0;
                }
                browser_util::save_item_to_client_storage(LENGHT_KEY, &lenght.to_string());
                self.lenght = lenght as u16;
            }
            Msg::OnClick => {
                let options = PasswordOptions::new()
                    .use_upper_case(self.use_upper)
                    .use_lower_case(self.use_lower)
                    .use_numeric(self.use_numeric)
                    .use_special(self.use_special)
                    .lenght(self.lenght as usize)
                    .build();
                let password = generate_random_password(&options);

                browser_util::window_alert(&password);
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_upper_change = ctx.link().callback(Msg::OnUpperChange);
        let on_lower_change = ctx.link().callback(Msg::OnLowerChange);
        let on_numeric_change = ctx.link().callback(Msg::OnNumericChange);
        let on_special_change = ctx.link().callback(Msg::OnSpecialChange);

        let on_lenght_change = ctx.link().callback(Msg::OnLenghtChange);

        let on_click = ctx.link().callback(|_| Msg::OnClick);

        html! {
            <div class="layout">
                <div style="grid-column: 2/4; grid-row: 2/3;">
                    <label style="font-size: x-large;">{"YEW password generator"}</label>
                </div>
                <CheckBox on_change={on_upper_change} checked={self.use_upper} row={3} label={"Use upper case letters".to_owned()}/>
                <CheckBox on_change={on_lower_change} checked={self.use_lower} row={4} label={"Use lower case letters".to_owned()}/>
                <CheckBox on_change={on_numeric_change} checked={self.use_numeric} row={5} label={"Use numeric".to_owned()}/>
                <CheckBox on_change={on_special_change} checked={self.use_special} row={6} label={"Use special".to_owned()}/>
                <NumericTextBoxBox on_change={on_lenght_change} value={self.lenght} label={"Password lenght".to_owned()}/>
                <div style={"grid-column: 2/4; grid-row: 8/9; justify-content: center;"}>
                    <button onclick={on_click}>{"Generate"}</button>
                </div>
            </div>
        }
    }
}

fn save_checked_in_client_storage(key: &str, checked: bool) {
    let mut value = "0";
    if checked {
        value = "1";
    }
    browser_util::save_item_to_client_storage(key, value);
}

fn read_checked_from_client_storage(key: &str) -> bool {
    let value = browser_util::get_item_from_client_storage(key).unwrap_or("1".to_owned());
    match value.as_str() {
        "1" => true,
        "0" | _ => false,
    }
}

fn get_lenght_from_client_storage() -> u16 {
    let mut value = browser_util::get_item_from_client_storage(LENGHT_KEY)
        .unwrap_or("8".to_owned())
        .parse::<u16>()
        .unwrap_or(8 as u16);
    if value < 4{
        value = 4;
    }
    value
}
