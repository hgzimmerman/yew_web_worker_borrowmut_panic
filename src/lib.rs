#![recursion_limit = "128"]

pub mod native_worker;

use yew::worker::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::native_worker::Response;

pub struct Model {
    link: ComponentLink<Self>,
    worker: Option<Box<dyn Bridge<native_worker::Worker>>>,
    state: usize
}

fn create_bridge(link: &ComponentLink<Model>) -> Option<Box<dyn Bridge<native_worker::Worker>>> {
    let callback = link.callback(|Response::CurrentState(state)| Msg::GotState(state));
    let worker = Some(native_worker::Worker::bridge(callback));
    worker
}

pub enum Msg {
    SendToWorker,
    GotState(usize),
    KillBridge,
    CreateBridge
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let worker = create_bridge(&link);
        Model {
            link,
            worker,
            state: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToWorker => {
                if let Some(worker) = self.worker.as_mut() {

                    for _ in 0..50 {
                        worker.send(native_worker::Request::Work);
                    }
                } else {
                    log::info!("Worker not available, recreating. Try again.");
                    self.worker = create_bridge(&self.link);
                }
                return true
            }
            Msg::GotState(state) => {
                self.state = state;
            }
            Msg::KillBridge => {
                self.worker = None;
            }
            Msg::CreateBridge => {
                self.worker = create_bridge(&self.link);
            }
        }
        true
    }

    fn view(&self) -> Html {
        if self.state > 1 {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::SendToWorker)>{ "Send to Thread" }</button>
                    <button onclick=self.link.callback(|_| Msg::KillBridge)>{ "Destroy Bridge" }</button>
                    <button onclick=self.link.callback(|_| Msg::CreateBridge)>{ "Create Bridge" }</button>
                </nav>
                {self.state}
                    <Model/>
            </div>
        }
        } else {
            html! {
                <div>
                    <nav class="menu">
                        <button onclick=self.link.callback(|_| Msg::SendToWorker)>{ "Send to Thread" }</button>
                        <button onclick=self.link.callback(|_| Msg::KillBridge)>{ "Destroy Bridge" }</button>
                        <button onclick=self.link.callback(|_| Msg::CreateBridge)>{ "Create Bridge" }</button>
                    </nav>
                    {self.state}
                </div>
            }
        }
    }
}
