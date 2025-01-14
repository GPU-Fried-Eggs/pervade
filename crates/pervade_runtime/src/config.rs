use bitflags::bitflags;

bitflags! {
    pub struct JSIntrinsics: u32  {
        const DATE = 1;
        const EVAL = 1 << 1;
        const REGEXP_COMPILER = 1 << 2;
        const REGEXP = 1 << 3;
        const JSON = 1 << 4;
        const PROXY = 1 << 5;
        const MAP_SET = 1 << 6;
        const TYPED_ARRAY  = 1 << 7;
        const PROMISE  = 1 << 8;
        const BIG_INT = 1 << 9;
    }
}

pub struct Config {
    pub intrinsics: JSIntrinsics,
    pub gc_threshold: usize,
    pub memory_limit: usize,
    pub max_stack_size: usize,
}

impl Config {
    pub fn gc_threshold(&mut self, bytes: usize) -> &mut Self {
        self.gc_threshold = bytes;
        self
    }

    pub fn memory_limit(&mut self, bytes: usize) -> &mut Self {
        self.memory_limit = bytes;
        self
    }

    pub fn max_stack_size(&mut self, bytes: usize) -> &mut Self {
        self.max_stack_size = bytes;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            intrinsics: JSIntrinsics::all(),
            gc_threshold: usize::MAX,
            memory_limit: usize::MAX,
            max_stack_size: 256 * 1024,
        }
    }
}
