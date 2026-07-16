use crate::error::VibeError;
use wasmtime::{Config, Engine, Module, Store, Linker};

pub struct WasmSandbox {
    engine: Engine,
}

impl WasmSandbox {
    pub fn new() -> Result<Self, VibeError> {
        let mut config = Config::new();
        config.async_support(true);
        // Note: we can restrict memory and CPU time using wasmtime features here later.
        
        let engine = Engine::new(&config).map_err(|e| VibeError::Plugin(e.to_string()))?;
        
        Ok(Self { engine })
    }

    /// Validates a WebAssembly module without executing it.
    pub fn validate_module(&self, wasm_bytes: &[u8]) -> Result<(), VibeError> {
        Module::validate(&self.engine, wasm_bytes).map_err(|e| VibeError::Plugin(format!("Invalid WASM module: {}", e)))
    }

    /// Loads and executes a WASM plugin in a fresh, isolated store.
    pub async fn run_plugin(&self, wasm_bytes: &[u8]) -> Result<String, VibeError> {
        let module = Module::new(&self.engine, wasm_bytes).map_err(|e| VibeError::Plugin(e.to_string()))?;
        let mut store = Store::new(&self.engine, ());
        let linker = Linker::new(&self.engine);
        
        // In a full implementation, we'd define host functions in `linker` here 
        // to expose CNTRL intents or filesystem selectively based on manifest permissions.
        
        let instance = linker
            .instantiate_async(&mut store, &module)
            .await
            .map_err(|e| VibeError::Plugin(e.to_string()))?;
            
        let run_func = instance
            .get_typed_func::<(), ()>(&mut store, "run")
            .map_err(|e| VibeError::Plugin(format!("Missing exported 'run' function: {}", e)))?;
            
        run_func
            .call_async(&mut store, ())
            .await
            .map_err(|e| VibeError::Plugin(format!("Plugin execution failed: {}", e)))?;
            
        Ok("Plugin executed successfully".into())
    }
}
