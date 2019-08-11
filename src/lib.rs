extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;


pub struct Model {
    message: String,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            message: String::from("Hello ")
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="greeting">
                <h2> {&format!("{} World!", self.message)}</h2>
            </div>
        }
    }
}

