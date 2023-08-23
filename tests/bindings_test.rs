#[cfg(test)]
mod tests {
    use qjs_rs::{JSContext, JSRuntime};

    #[test]
    pub fn runtime_test() {
        if let Ok(rt) = JSRuntime::new() {
            println!("Runtime = {:?}", rt);
        }

        if let Ok(rt) = JSRuntime::new(){
            rt.memory_limit(100_000);
            rt.dump_mem();
        }
    }


    #[test]
    pub fn context_test() {
        if let Ok(rt) = JSRuntime::new() {
            if let Ok(ctx) = JSContext::new(&rt) {
                println!("Context = {:?}", ctx);
                ctx.dump_mem();
            }
        }
    }
}