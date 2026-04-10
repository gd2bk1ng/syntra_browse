// Authour: Alexandr Roussinov
#[derive(Debug, Clone)]
pub struct Relation { pub from: String, pub to: String, pub label: String }

#[derive(Debug, Clone, Default)]
pub struct KnowledgeGraph { pub relations: Vec<Relation> }
