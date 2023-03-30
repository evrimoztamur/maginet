#![forbid(missing_docs)]
#![feature(is_some_and)]

//! The `shared` crate contains all the components which are used by both the client and the server, which includes the entire game logic too.

mod game;
mod lobby;
mod net;
// mod vecmap;

pub use game::*;
pub use lobby::*;
pub use net::*;
// pub use vecmap::*;
