use crate::cognition::context::CognitiveContext;

pub fn inspect_context(context: &CognitiveContext, limit: usize) {
    println!("================================================");
    println!(" SYNTRA COGNITIVE INSPECTION");
    println!("================================================");

    let records = context.recent(limit);

    if records.is_empty() {
        println!("No cognitive records available.");
        return;
    }

    for (index, record) in records.iter().enumerate() {
        println!(
            "#{:<3} [{:?}] actor='{}' event='{}' success={}",
            index + 1,
            record.timestamp,
            record.actor,
            record.event,
            record.success
        );
    }

    println!("------------------------------------------------");
    println!("Total recorded events: {}", context.total_events());
    println!("================================================");
}
