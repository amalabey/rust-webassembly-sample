extern crate yew_sample_lib;
extern crate yew;

use yew_sample_lib::Model;
use yew::prelude::App;

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
