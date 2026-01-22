/* ================================================================================================
   SYNTRA BROWSER - AXIOM TWO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/request_lobe.rs
   Module:      Cortex - Request Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Handles incoming user requests, classifies them, and routes them to the appropriate
                cognitive lobe. This is Syntra's primary "intent router" for Axiom Two.

   Notes:
     - Lightweight and dependency-minimal.
     - Integrates with diagnostics, tracing, and memory.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{info, warn, trace_enter, trace_exit};
use crate::cortex::{MemoryLobe, PlanLobe, ReflectionLobe, EvolutionLobe};

/// High-level request categories.
#[derive(Debug, Clone)]
pub enum RequestKind {
    Query,
    SelfReflection,
    Planning,
    Evolution,
    Unknown,
}

/// Request payload.
#[derive(Debug, Clone)]
pub struct Request {
    pub text: String,
    pub kind: RequestKind,
}

impl Request {
    pub fn classify(text: &str) -> RequestKind {
        let lower = text.to_lowercase();

        if lower.contains("how should we evolve")
            || lower.contains("improve")
            || lower.contains("upgrade")
        {
            return RequestKind::Evolution;
        }

        if lower.contains("plan")
            || lower.contains("steps")
            || lower.contains("design")
        {
            return RequestKind::Planning;
        }

        if lower.contains("what can you do")
            || lower.contains("who are you")
            || lower.contains("self")
        {
            return RequestKind::SelfReflection;
        }

        if lower.contains("?") || lower.split_whitespace().count() > 1 {
            return RequestKind::Query;
        }

        RequestKind::Unknown
    }
}

/// Main request router.
pub struct RequestLobe;

impl RequestLobe {
    pub fn handle(text: &str, memory: &mut MemoryLobe) -> String {
        trace_enter("RequestLobe::handle");

        let kind = Request::classify(text);
        memory.store_intent(text, &format!("{:?}", kind));

        let response = match kind {
            RequestKind::Query => {
                ReflectionLobe::reflect(text)
            }
            RequestKind::SelfReflection => {
                ReflectionLobe::self_reflect()
            }
            RequestKind::Planning => {
                PlanLobe::generate_plan(text)
            }
            RequestKind::Evolution => {
                EvolutionLobe::propose_evolution(text)
            }
            RequestKind::Unknown => {
                warn("Unknown request type; defaulting to reflection.");
                ReflectionLobe::reflect(text)
            }
        };

        memory.store_response(&response);
        trace_exit("RequestLobe::handle");
        response
    }
}
