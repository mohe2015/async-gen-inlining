#![feature(gen_blocks, async_iterator)]

pub gen fn test() -> &'static str {
    yield "Hi";
}
