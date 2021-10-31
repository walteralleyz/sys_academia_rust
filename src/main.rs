use crate::controller::Controller;

mod controller;
mod view;
mod model;
mod repository;
mod utils;

fn main() {
    let controller = Controller::new();
    controller.exec();
}