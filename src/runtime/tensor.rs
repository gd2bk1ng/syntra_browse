/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/runtime/tensor.rs
   Module:      Tensor Engine Skeleton
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal tensor abstraction for future GPU-accelerated operations in Syntra.

   Notes:
     - Axiom Three defines the shape, not the backend.
   ================================================================================================ */

#[derive(Debug, Clone)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub data: Vec<f32>,
}

impl Tensor {
    pub fn zeros(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        Tensor {
            shape: shape.to_vec(),
            data: vec![0.0; size],
        }
    }
}
