use log::info;
use serde_derive::{Deserialize, Serialize};
use yew::worker::*;

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
    state: usize,
    hidden: u32
}

impl Agent for Worker {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Worker {
            link,
            state: 0,
            hidden: 0
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
//        info!("Request: {:?}", msg);
        match msg {
            Request::Work => {
                do_work(self);
                self.link.respond(who, Response::CurrentState(self.state));
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "bin/native_worker.js"
    }
}


fn do_work(worker: &mut Worker) {

    let x = fibonacci(35);
    worker.hidden = worker.hidden.wrapping_add(x);
    worker.state += 1;
}


#[no_mangle]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}