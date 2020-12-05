#![recursion_limit="512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    count: i64
}

enum Msg {
    AddOne,
    SubOne
}

fn fibonacci(n: i64) -> i64
{
    if n == 0 || n == 1
    {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            count: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                self.value = fibonacci(self.count);
            },
            Msg::SubOne => {
                self.count -= 1;
                if self.count < 0
                {
                    self.count = 0;
                }
                self.value = fibonacci(self.count);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {   
        // let string = progress(self.count);
        html! {
            <div class="shadow-lg p-3 mb-5 bg-white rounded">
                <h1>{"Sequência de Fibonacci"}</h1>
                <h3>{"Rust + Wasm == ❤️"}</h3>
                <div class="btn-group" role="group" aria-label="Basic example">
                    <button type="button" class="btn btn-secondary" onclick=self.link.callback(|_| Msg::AddOne)>{"+1"}</button>
                    <button type="button" class="btn btn-secondary" onclick=self.link.callback(|_| Msg::SubOne)>{"-1"}</button>
                </div>
                <p>{"Posição: "}{"\""}{self.count}{"\""}</p>
                <p>{"Resulado = "}{self.value}</p>
                // <div class="progress">
                // <div class="progress-bar progress-bar-striped progress-bar-animated" role="progressbar"  style=string aria-valuenow="0" aria-valuemin="10" aria-valuemax="100"></div>
                // </div>
                // <div class="alert alert-warning alert-dismissible fade show" role="alert">
                // <strong>{"Holy guacamole!"}</strong> {"You should check in on some of those fields below."}
                // </div>
            </div>
        }
    }
}

fn progress(n: i64) -> String
{
    let concat = n.to_string().to_owned();
    let widht = "width: ".to_string();
    let porcent = "%".to_string();
    let concatenation = format!("{}{}{}",widht, concat, porcent);
    concatenation
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}