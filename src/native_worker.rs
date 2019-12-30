use log::info;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::worker::*;
use yew::services::fetch::FetchService;
use yew::services::interval::IntervalService;
use yew::services::Task;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Work,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    CurrentState(usize),
}

pub enum Msg {
    Updating,
}

pub struct Worker {
    link: AgentLink<Worker>,
    state: usize
}

impl Agent for Worker {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Worker {
            link,
            state: 0
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                info!("Tick...");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::Work => {
                self.state += 1;
                self.link.respond(who, Response::CurrentState(self.state));
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "bin/native_worker.js"
    }
}
