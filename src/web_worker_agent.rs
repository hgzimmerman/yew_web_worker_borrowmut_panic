use yew::agent::{Agent, HandlerId, AgentLink, Public, Context};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct WWAgent {
    state: usize,
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkUnit;
#[derive(Serialize, Deserialize)]
pub struct CurrentState(pub usize);

impl Agent for WWAgent {
    type Reach = Public;
    type Message = ();
    type Input = WorkUnit;
    type Output = CurrentState;

    fn create(link: AgentLink<Self>) -> Self {
        WWAgent {
            state: 0,
            link
        }
    }

    fn update(&mut self, _msg: Self::Message) {
    }

    fn handle_input(&mut self, _msg: Self::Input, id: HandlerId) {
        log::info!("Got message");
        self.state += 1;
        log::info!("new_state = {}", self.state);
        self.link.respond(id, CurrentState(self.state))
    }
}