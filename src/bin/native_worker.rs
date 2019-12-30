use yew::agent::Threaded;
use yew_web_workerborrowmut_panic::native_worker;

fn main() {
    web_logger::init();
    yew::initialize();
    native_worker::Worker::register();
    yew::run_loop();
}
