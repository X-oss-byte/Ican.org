#![feature(lint_reasons)]
#![feature(iter_intersperse)]
#![feature(arbitrary_self_types)]
#![feature(async_fn_in_trait)]

pub(crate) mod chunking_context;
pub(crate) mod ecmascript;
pub(crate) mod minify;

pub use chunking_context::{BuildChunkingContext, BuildChunkingContextBuilder, MinifyType};

pub fn register() {
    turbo_tasks::register();
    turbo_tasks_fs::register();
    turbopack_core::register();
    turbopack_ecmascript::register();
    turbopack_ecmascript_runtime::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
