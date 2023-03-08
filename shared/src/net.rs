use std::sync::Mutex;

use serde::{Deserialize, Serialize, Serializer};

use crate::{Game, Turn};

pub struct MutexWrapper<T: ?Sized>(pub Mutex<T>);

impl<T: ?Sized + Serialize> Serialize for MutexWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0
            .lock()
            .expect("mutex is poisoned")
            .serialize(serializer)
    }
}

#[derive(Serialize)]
pub enum OutMessage<'a> {
    Move(Turn),
    Game(&'a MutexWrapper<Game>),
}

#[derive(Debug, Deserialize)]
pub enum Message {
    Move(Turn),
    Game(Game),
}
