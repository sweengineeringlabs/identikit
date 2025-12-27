//! Face Embedding Providers
//!
//! Implementations of the FaceEmbedder trait.

mod dlib_embedder;

pub use dlib_embedder::DlibEmbedder;
