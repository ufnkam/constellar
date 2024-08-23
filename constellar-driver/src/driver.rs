// use crate::engine::{Backend, ConnectionStash};
// use crate::sql::Column;

// pub trait Driver<B: Backend> {
//     fn get_name(self) -> String;

//     fn make_stash(&self) -> Box<ConnectionStash<B>> {
//         return Box::new(ConnectionStash::new());
//     }
// }
