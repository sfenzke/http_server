pub mod dummy_provider;

pub trait FileProvider {
    fn provide_file(&self, path: &str) -> Vec<u8>;
}

pub use crate::file::provider::dummy_provider::DummyProvider;
