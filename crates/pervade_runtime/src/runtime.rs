use std::mem::ManuallyDrop;

use rquickjs::{
    context::{intrinsic, Intrinsic},
    Context, Module, Runtime as JsRuntime,
};

use crate::config::{Config, JSIntrinsics};
use crate::error::Error;

pub struct Runtime {
    context: ManuallyDrop<Context>,
    inner: ManuallyDrop<JsRuntime>,
}

impl Runtime {
    pub fn new(config: Config) -> Result<Self, Error> {
        let Config { intrinsics, gc_threshold, memory_limit, max_stack_size } = config;

        let rt = JsRuntime::new()?;

        rt.set_gc_threshold(gc_threshold);
        rt.set_memory_limit(memory_limit);
        rt.set_max_stack_size(max_stack_size);

        let context = Context::custom::<()>(&rt)?;

        context.with(|ctx| {
            let raw_ctx = ctx.as_raw();

            unsafe {
                if intrinsics.contains(JSIntrinsics::DATE) {
                    intrinsic::Date::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::EVAL) {
                    intrinsic::Eval::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::REGEXP_COMPILER) {
                    intrinsic::RegExpCompiler::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::REGEXP) {
                    intrinsic::RegExp::add_intrinsic(raw_ctx);
                }

                if intrinsics.contains(JSIntrinsics::JSON) {
                    intrinsic::Json::add_intrinsic(raw_ctx);
                }

                if intrinsics.contains(JSIntrinsics::PROXY) {
                    intrinsic::Proxy::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::MAP_SET) {
                    intrinsic::MapSet::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::TYPED_ARRAY) {
                    intrinsic::TypedArrays::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::PROMISE) {
                    intrinsic::Promise::add_intrinsic(raw_ctx)
                }

                if intrinsics.contains(JSIntrinsics::BIG_INT) {
                    intrinsic::BigInt::add_intrinsic(raw_ctx)
                }
            }
        });

        Ok(Self {
            inner: ManuallyDrop::new(rt),
            context: ManuallyDrop::new(context),
        })
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn join(&self) -> Result<(), Error> {
        if self.inner.is_job_pending() {
            loop {
                let result = self.inner.execute_pending_job();
                if let Ok(false) = result {
                    break;
                }

                if let Err(e) = result {
                    return Err(Error::JobException(e.to_string()));
                }
            }
        }

        Ok(())
    }

    pub fn has_pending_jobs(&self) -> bool {
        self.inner.is_job_pending()
    }

    pub fn compile_to_bytecode(&self, name: &str, contents: &str) -> Result<Vec<u8>, Error> {
        Ok(self.context()
            .with(|this| Module::declare(this.clone(), name, contents)?.write_le())?)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(Config::default()).unwrap()
    }
}
