// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (COMMAND REGISTRY)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/mod.rs
//   Module:      Command Registry
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Command handlers for the Syntra Terminal. Mirrors the original PowerShell shell
//                while routing into the Rust Cortex and AGI Core.
// ================================================================================================

#![allow(dead_code)]

pub mod help;
pub mod status;
pub mod diagnose;
pub mod self_check;
pub mod sync;

pub mod browse;
pub mod knowledge;
pub mod task;
pub mod perceive;
pub mod act;

pub mod evolve;
pub mod sandbox;

pub mod thoughts;
pub mod ecosystem;
pub mod propose;
pub mod propose_safe;
pub mod safety;

pub mod banner;
pub mod introspect;
