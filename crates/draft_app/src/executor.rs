use crate::App;

pub trait Executor {
    fn run(&mut self, app: App);
}
