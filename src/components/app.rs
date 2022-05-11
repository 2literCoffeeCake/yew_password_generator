use yew::{html, Component, Context, Html};

use crate::{
    browser_util,
    password_builder::{generate_random_password, PasswordOptions},
};

use super::{CheckBox, NumericTextBoxBox};
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
            use_lower: true,
            use_numeric: true,
            use_special: true,
            use_upper: true,
            lenght: 8,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnUpperChange(checked) => self.use_upper = checked,
            Msg::OnLowerChange(checked) => self.use_lower = checked,
            Msg::OnNumericChange(checked) => self.use_numeric = checked,
            Msg::OnSpecialChange(checked) => self.use_special = checked,
            Msg::OnLenghtChange(mut lenght) => {
                lenght = lenght.floor();
                if lenght < 4.0 {
                    lenght = 4.0;
                }
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
                    <label style="font-size: x-large;">{"WASM password generator"}</label>
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
