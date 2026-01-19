<!-- ================================================================================================
     SYNTRA BROWSER — AXIOM ZERO
     ------------------------------------------------------------------------------------------------
     File:        codex/intent_ontology.md
     Module:      Intent Ontology v1.0
     Author:      Alexandr Roussinov (gd2bk1ng)
     Description: A first‑pass ontology for representing user and system intents within Syntra’s
                  AGI Core. This document defines conceptual categories and examples.

     Notes:
       This is a living document. As Syntra’s cognitive capabilities grow, this ontology should be
       refined, expanded, and versioned.
     ================================================================================================ -->

# Syntra Intent Ontology — Version 1.0

Syntra’s AGI Core operates on **intents** — structured representations of what the user or system
is trying to achieve. This ontology defines the first generation of intent categories.

---

## 1. Navigation Intents

Intents related to moving through information spaces.

- `Navigate.ToUrl` — open a specific URL  
- `Navigate.Back` — go to previous state  
- `Navigate.Forward` — go to next state  
- `Navigate.Home` — return to a defined “home” context  

---

## 2. Query Intents

Intents related to asking questions or retrieving information.

- `Query.Search` — general information retrieval  
- `Query.Explain` — request for explanation or breakdown  
- `Query.Compare` — compare entities, options, or states  

---

## 3. Control Intents

Intents that modify Syntra’s internal state or behavior.

- `Control.Theme.Switch` — change visual theme  
- `Control.Layout.Adjust` — modify UI layout  
- `Control.Mode.Switch` — change operating mode (e.g., “focus”, “explore”)  

---

## 4. System Intents

Intents originating from Syntra itself.

- `System.Heartbeat` — periodic health/status check  
- `System.Optimize` — internal performance tuning  
- `System.Update` — apply new models or configurations  

---

## 5. Meta Intents

Intents about the interaction itself.

- `Meta.ExplainDecision` — ask Syntra to explain its reasoning  
- `Meta.SummarizeSession` — summarize recent activity  
- `Meta.AdjustPersonality` — tune interaction style  

---

## Representation

In code, intents are currently represented as:

```rust
pub struct Intent {
    pub label: String,
    pub confidence: f32,
}
