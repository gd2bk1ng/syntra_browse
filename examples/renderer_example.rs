// renderer backend exanmole
// file: examples/renderer_example.rs

use syntra::renderer::{
    Renderer, NullRenderer, RenderBackend, start_renderer,
    graph::{RenderGraph, RenderNode},
};
use syntra::renderer::backend_example::ExampleBackend;

fn main() {
    // Initialize and start renderer subsystem (stub)
    start_renderer();

    // Create and use NullRenderer
    let mut null_renderer = NullRenderer::default();
    null_renderer.resize(1280, 720);
    null_renderer.render();
    println!("NullRenderer state after resize: {:?}", null_renderer);

    // Create a render graph with nodes
    let mut graph = RenderGraph::new();
    graph.add_node(RenderNode::new("Opaque Pass"));
    graph.add_node(RenderNode::new("Transparent Pass"));

    // Create example GPU backend and submit the graph
    let mut backend = ExampleBackend::new();
    backend.submit(&graph);
}
