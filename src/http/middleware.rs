// struct Middleware {}
//
// impl Middleware {
// pub fn handle() {}
//
// pub fn terminate() {}
// }

pub trait Middleware {
    fn handle();
    fn terminate();
}
