# ACTORS

Actors are the fundamental units of execution in Syntra.

An actor is not a thread.
An actor is not a task.
An actor is a bounded responsibility.

## Actor Properties

• Owns its execution context
• Executes autonomously
• Is scheduled deterministically
• Can be externally observed

## Current Role

At Axiom Zero:
• Actors execute runtime responsibilities
• Actors do not negotiate intent
• Actors do not modify themselves

## Future Role

Later phases introduce:
• Intent-bound actors
• Actor collectives
• Dynamic actor graphs

Actors are designed to act first,
and reason later.
