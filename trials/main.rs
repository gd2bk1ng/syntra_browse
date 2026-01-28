/* ================================================================================================
   SYNTRA KERNEL — AXIOM THREE
   ------------------------------------------------------------------------------------------------
        .\s/.
       :: S ::
        '/s\'

   File:        trials/main.rs
   Module:      Syntra Trials Sandbox
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Experimental sandbox binary for trying out new runtime ideas, intent execution,
                pipeline resolution, actor-backed execution, and cognitive loop validation.

   Notes:
     - Axiom Three introduces executable intent via static pipelines.
     - Observation, inspection, and failure-aware routing are enabled.
     - No learning, no adaptation, no autonomy.
     - This file is a laboratory surface for cognitive experimentation.
   ================================================================================================ */

use syntra_kernel::cli::inspect::inspect_context;

use syntra_kernel::cognition::context::CognitiveContext;
use syntra_kernel::cognition::observation::ObservationEmitter;

use syntra_kernel::intent::Intent;
use syntra_kernel::intent::pipeline_map::resolve_pipeline;

use syntra_kernel::pipeline::executor::execute_pipeline_with_actor;

fn main() {
    env_logger::init();

    println!("Syntra Kernel Trials — Axiom Three");
    println!("Executable intent with actor-backed pipelines\n");

    // ------------------------------------------------------------
    // Cognitive Context Initialization
    // ------------------------------------------------------------

    let mut cognitive_context = CognitiveContext::new();
    let mut observer = ObservationEmitter::new(&mut cognitive_context);

    observer.emit(
        "trial_executor",
        "trial_started",
        None,
        true,
    );

    // ------------------------------------------------------------
    // Intent → Pipeline → Actor-backed Execution
    // ------------------------------------------------------------

    let intent = Intent::RunTrial;

    observer.emit(
        "intent_system",
        "intent_declared",
        None,
        true,
    );

    let pipeline = resolve_pipeline(&intent);

    observer.emit(
        "pipeline_system",
        "pipeline_resolved",
        None,
        true,
    );

    let execution_result = execute_pipeline_with_actor(&pipeline, &mut observer);

    // ------------------------------------------------------------
    // Failure-Aware Routing (Non-Adaptive)
    // ------------------------------------------------------------

    observer.emit(
        "pipeline_system",
        if execution_result {
            "pipeline_succeeded"
        } else {
            "pipeline_failed"
        },
        None,
        execution_result,
    );

    observer.emit(
        "trial_executor",
        "trial_completed",
        None,
        execution_result,
    );

    // ------------------------------------------------------------
    // CLI Cognitive Inspection (Single Inspection Surface)
    // ------------------------------------------------------------

    inspect_context(&cognitive_context, 20);

    println!(
        "\nTotal cognitive events recorded: {}",
        cognitive_context.total_events()
    );
}
