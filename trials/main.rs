/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
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
   ================================================================================================ */

use syntra::cli::inspect::inspect_context;

use syntra::cognition::context::CognitiveContext;
use syntra::cognition::observation::ObservationEmitter;

use syntra::intent::intent::Intent;
use syntra::intent::pipeline_map::resolve_pipeline;

use syntra::pipeline::executor::execute_pipeline_with_actor;

fn main() {
    env_logger::init();

    println!("Syntra Trials — Axiom Three");
    println!("Executable intent with actor-backed pipelines");

    // ------------------------------------------------------------
    // Cognitive context initialization
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
    // Failure-aware routing (non-adaptive)
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
    // CLI Cognitive Inspection (single inspection surface)
    // ------------------------------------------------------------

    inspect_context(&cognitive_context, 20);

    println!(
        "Total cognitive events recorded: {}",
        cognitive_context.total_events()
    );
}
