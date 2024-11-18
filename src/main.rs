mod controller;
mod model;
mod view;

use controller::Controller;

fn main() {
    Controller::new().start();
}
