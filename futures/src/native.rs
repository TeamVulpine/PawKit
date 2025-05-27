use std::{future::Future, sync::LazyLock};

use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

pub fn block_on<T>(fut: T) -> <T as Future>::Output where T : Future {
    return RUNTIME.block_on(fut);
}

pub fn spawn<T>(fut: T) where T : Future + Send + 'static, <T as Future>::Output : Send + 'static {
    RUNTIME.spawn(fut);
}

pub use tokio::select;
