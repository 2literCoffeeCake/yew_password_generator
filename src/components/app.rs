use yew::{html, Component, Context, Html};

use super::check_box::CheckBox;
pub enum Msg {
    OnUpperChange(bool),
    OnLowerChange(bool),
    OnNumericChange(bool),
    OnSpecialChange(bool),
}

#[derive(Debug, Default)]
pub struct App{
    use_upper: bool,
    use_lower: bool,
    use_numeric: bool,
    use_special: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            use_lower: true,
            use_numeric: true,
            use_special: true,
            use_upper: true
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnUpperChange(checked) => self.use_upper = checked,
            Msg::OnLowerChange(checked) => self.use_lower = checked,
            Msg::OnNumericChange(checked) => self.use_numeric = checked,
            Msg::OnSpecialChange(checked) => self.use_special = checked,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_upper_change = ctx.link().callback(Msg::OnUpperChange);
        let on_lower_change = ctx.link().callback(Msg::OnLowerChange);
        let on_numeric_change = ctx.link().callback(Msg::OnNumericChange);
        let on_special_change = ctx.link().callback(Msg::OnSpecialChange);
        html! {
            <div class="layout">
                <div style="grid-column: 2/4; grid-row: 2/3;">
                    <label style="font-size: x-large;">{"WASM password generator"}</label>
                </div>
                <CheckBox on_change={on_upper_change} checked={self.use_upper} row={3} label={"Use upper case letters".to_owned()}/>
                <CheckBox on_change={on_lower_change} checked={self.use_lower} row={4} label={"Use lower case letters".to_owned()}/>
                <CheckBox on_change={on_numeric_change} checked={self.use_numeric} row={5} label={"Use numeric".to_owned()}/>
                <CheckBox on_change={on_special_change} checked={self.use_special} row={6} label={"Use special".to_owned()}/>
            </div>
        }
    }
}
