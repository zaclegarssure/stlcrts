pub mod evaluator;
pub mod term;
pub mod typechecker;

// Re-export the main public API
pub use evaluator::*;
pub use term::*;
pub use typechecker::*;
