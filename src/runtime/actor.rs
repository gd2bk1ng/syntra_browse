/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/runtime/actor.rs
   Module:      Actor System Skeleton
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal actor system skeleton for Syntra. Defines ActorId, Message, and a basic
                ActorSystem trait for future async/tab orchestration.

   Notes:
     - Axiom Three keeps this purely conceptual and synchronous.
   ================================================================================================ */

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorId(pub u64);

#[derive(Debug, Clone)]
pub enum Message {
    Text(String),
    Intent(String),
}

pub trait Actor {
    fn id(&self) -> ActorId;
    fn handle(&mut self, msg: Message);
}

pub struct ActorSystem {
    next_id: u64,
    actors: HashMap<ActorId, Box<dyn Actor>>,
}

impl ActorSystem {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            actors: HashMap::new(),
        }
    }

    pub fn spawn<A: Actor + 'static>(&mut self, actor: A) -> ActorId {
        let id = actor.id();
        self.actors.insert(id, Box::new(actor));
        id
    }

    pub fn send(&mut self, id: ActorId, msg: Message) {
        if let Some(actor) = self.actors.get_mut(&id) {
            actor.handle(msg);
        }
    }
}
