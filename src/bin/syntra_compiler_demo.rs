/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/bin/syntra_compiler_demo.rs
   Module:      Syntra Refrence Compiler Demo
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Standalone demo binary showcasing the Syntra Language front-end:
                  • Lexer
                  • Parser
                  • AST generation
                  • Type checking
                  • Intent dataset example
                  • Syntra Browser architecture skeleton

   Notes:
     - Axiom Three keeps this demo isolated from the main browser runtime.
     - Use this as a playground for Syntra Language evolution.
   ================================================================================================ */

use syntra::lexer::Lexer;
use syntra::parser::Parser;
use syntra::type_checker::TypeContext;
use syntra::dataset::IntentDataset;
use syntra::tokens::TokenKind;

fn main() {
    println!("🧪 Syntra Refrence Compiler Demo");
    println!("================================\n");

    let source = r#"
        intent syntra_core {
            motive "performance"
            action "optimize"
        }

        fn add(x: Int<64>, y: Int<64>) -> Int<64> {
            let z = x;
            return z;
        }

        rule simple_rule(a: Int<64>) -> Int<64> {
            return a;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().expect("Lexer returned None unexpectedly");
        let kind = token.kind.clone();
        tokens.push(token);
        if kind == TokenKind::Eof {
            break;
        }
    }

    println!("=== Tokens ===");
    for token in &tokens {
        println!("{:?} at {:?}", token.kind, token.span);
    }

    let mut parser = Parser::new(&tokens);
    let ast_nodes = parser.parse_program();

    println!("\n=== AST Nodes ===");
    for node in &ast_nodes {
        println!("{:#?}", node);
    }

    if !parser.errors().is_empty() {
        println!("\n=== Parser Errors ===");
        for e in parser.errors() {
            println!("{:?}", e);
        }
    }

    let mut ctx = TypeContext::new();
    for node in &ast_nodes {
        match node {
            syntra::ast::AstNode::Function(f) => {
                if let Err(e) = ctx.check_function(f) {
                    println!("\nType error in function '{}': {:?}", f.name, e);
                }
            }
            syntra::ast::AstNode::Rule(r) => {
                if let Err(e) = ctx.check_rule(r) {
                    println!("\nType error in rule '{}': {:?}", r.name, e);
                }
            }
            syntra::ast::AstNode::Intent(i) => {
                println!("\nIntent parsed: {:?}", i);
            }
        }
    }

    let mut dataset = IntentDataset::new();
    dataset.add_intent("Parenting", Some(101), "Monitor child's screen time usage");
    dataset.add_intent("Career Development", Some(201), "Update resume with recent experience");

    let json = serde_json::to_string_pretty(&dataset).unwrap();
    println!("\n=== Serialized Intent Dataset JSON ===\n{}", json);

    println!("\n=== Syntra Browser Skeleton ===");
    syntra::browser::print_syntra_browser_overview();
}
