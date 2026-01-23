# THE COGNITIVE LOOP

Syntra’s architecture is organized around a single invariant:

Cognition is a loop, not a function.

The loop consists of five phases:

1. Intent Declaration
   Intents are structured descriptions of desired outcomes.
   They are not commands, and they do not imply implementation.

2. Pipeline Resolution
   Intents are translated into staged execution pipelines.
   Each stage represents a transformation of context.

3. Actor Execution
   Pipelines are executed by autonomous actors scheduled
   by the runtime kernel.

4. Observation
   Execution is continuously observed through diagnostics,
   tracing, and logging.

5. Context Update
   Observations are recorded into a cognitive context,
   enabling future introspection and adaptation.

At Axiom Zero, phases 1, 3, and 4 are operational.
Phases 2 and 5 are instantiated conceptually and under construction.

This loop is the contract Syntra enforces on itself.
