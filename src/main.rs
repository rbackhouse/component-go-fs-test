use anyhow::Result;

wasmtime::component::bindgen!({
    world: "testfs",
    path: "./wit",
    async: true
});

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{
    WasiCtxBuilder, Table, WasiCtx, WasiView, command::add_to_linker, DirPerms, FilePerms
};

#[tokio::main] 
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let preopen_dir = wasmtime_wasi::Dir::open_ambient_dir("./", wasmtime_wasi::ambient_authority())?;

    let builder = WasiCtxBuilder::new()
    .push_preopened_dir(
        preopen_dir,
        DirPerms::all(), 
        FilePerms::all(),
        "/",
    )
    .inherit_stdio();

    let mut table = Table::new();
    let wasi = builder.build(&mut table)?;

    struct Ctx {
        table: Table,
        wasi: WasiCtx,
    }

    impl WasiView for Ctx {
        fn table(&self) -> &Table {
            &self.table
        }
        fn table_mut(&mut self) -> &mut Table {
            &mut self.table
        }
        fn ctx(&self) -> &WasiCtx {
            &self.wasi
        }
        fn ctx_mut(&mut self) -> &mut WasiCtx {
            &mut self.wasi
        }
    }

    let mut linker: Linker<Ctx> = Linker::new(&engine);

    let component = Component::from_file(&engine, "testfs.component.wasm").unwrap();

    add_to_linker(&mut linker)?;

    let mut store = Store::new(
        &engine,
        Ctx {
            table,
            wasi,
        },
    );
    
    let instance = Testfs::instantiate_async(&mut store, &component, &linker).await?;
    instance.0.call_openfile(&mut store).await?;

    Ok(())
}
