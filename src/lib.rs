#![recursion_limit = "128"]

pub mod native_worker;

use log::info;
use yew::worker::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::native_worker::Response;

pub struct Model {
    link: ComponentLink<Self>,
    worker: Box<dyn Bridge<native_worker::Worker>>,
    state: usize
}

pub enum Msg {
    SendToWorker,
    GotState(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|Response::CurrentState(state)| Msg::GotState(state));
        let worker = native_worker::Worker::bridge(callback);
        Model {
            link,
            worker,
            state: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToWorker => {
                for _ in 0..1000 {
                    self.worker.send(native_worker::Request::Work);
                }
            }
            Msg::GotState(state) => {
                info!("DataReceived");
                self.state = state;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::SendToWorker)>{ "Send to Thread" }</button>
                </nav>
                {self.state}
            </div>
        }
    }
}
