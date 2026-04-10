/* ================================================================================================
   SYNTRA KERNEL - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/runtime/scheduler.rs
   Module:      Syntra Kernel :: Scheduler Skeleton
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: High-level scheduler skeleton for coordinating actors, timers, and IO in Syntra.

   Notes:
     - Axiom Three keeps this synchronous and conceptual.
   ================================================================================================ */

use std::time::{Duration, Instant};
use std::thread::sleep;

use super::actor::ActorSystem;

pub struct Scheduler {
    pub actors: ActorSystem,
    pub tick_rate: Duration,
    running: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SchedulerConfig {
   pub tick_rate: Duration,
}

pub type RuntimeScheduler = Scheduler;

impl Scheduler {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            actors: ActorSystem::new(),
            tick_rate,
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        println!("Syntra Kernel Scheduler started.");

        while self.running {
            let start = Instant::now();

            self.tick();

            let elapsed = start.elapsed();
            if elapsed < self.tick_rate {
                sleep(self.tick_rate - elapsed);
            }
        }
    }

    pub fn tick(&mut self) {
        // The kernel heartbeat
        // Every subsystem will hang off this moment of time.

        // For now we simply broadcast a "system tick"
        // (we'll wire cognition here next)
        println!("[kernel] tick");
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
