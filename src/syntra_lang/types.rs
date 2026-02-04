// ================================================================================================
//   SYNTRA LANGUAGE — TYPE SYSTEM (V0.1 SKELETON)
// ------------------------------------------------------------------------------------------------
//   File:        src/syntra_lang/types.rs
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Initial Syntra type system representation. Hybrid, linear-aware, and ready to be
//       extended with capabilities, symbolic wrappers, and differentiable types.
// ================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntraType {
    Unit,
    Bool,
    Int,
    Float,
    Str,
    Tensor {
        elem: Box<SyntraType>,
        dims: Vec<usize>,
    },
    // Future:
    // Ref(Box<SyntraType>),
    // Mut(Box<SyntraType>),
    // Cap(Box<SyntraType>),
    // Symbolic(Box<SyntraType>),
    // Grad(Box<SyntraType>),
    Unknown,
}

impl SyntraType {
    pub fn tensor(elem: SyntraType, dims: Vec<usize>) -> Self {
        SyntraType::Tensor {
            elem: Box::new(elem),
            dims,
        }
    }
}
