use super::binding::Binding;

#[derive(Debug)]
pub struct WasmModule {
    pub path: String,
    pub filename: String,
    pub binding: String,
}

impl WasmModule {
    pub fn filename(&self) -> String {
        self.filename.to_string()
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }

    // `name` corresponds to the binding used in the worker js
    // `part` corresponds to the name given to the file in the upload form
    pub fn binding(&self) -> Binding {
        let name = &self.binding;
        let part = &self.filename();

        Binding::new_wasm_module(name, part)
    }
}
