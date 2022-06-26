use super::FileProvider;

pub struct DummyProvider {}

impl DummyProvider {
    pub fn new() -> Self {
        DummyProvider {}
    }
}

impl FileProvider for DummyProvider {
    fn provide_file(&self, path: &String) -> Vec<u8> {
        println!("{}", path);
        vec![0; 10]
    }
}