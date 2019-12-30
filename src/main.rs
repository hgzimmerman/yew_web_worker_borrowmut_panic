use yew::{html, Component, ComponentLink, Html, ShouldRender, Bridged, Bridge};
use crate::web_worker_agent::{WWAgent, WorkUnit, CurrentState};

mod web_worker_agent;

struct Model {
    link: ComponentLink<Self>,
    state: usize,
    bridge: Box<dyn Bridge<WWAgent>>
}

enum Msg {
    DoIt,
    GotState(usize)
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        let cb = link.callback(|CurrentState(x)| Msg::GotState(x));
        let mut bridge = WWAgent::bridge(cb);
        Model { link, state: 0, bridge }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                log::info!("doing it");


                for _n in 0..100 {
                   self.bridge.send(WorkUnit);
                }

                true
            }
            Msg::GotState(x) => {
                log::info!("update model state");
                self.state = x;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::DoIt);
        html! {
            <>
                <button onclick=onclick>{ "Click me!" }</button>
                <div>
                    {self.state}
                </div>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}