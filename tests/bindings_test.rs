

#[cfg(test)]
mod tests {
    use qjs_rs::{JSContext, JSRuntime, JSValue};

    #[test]
    pub fn runtime_test(){
        if let Ok(rt) = JSRuntime::new() {
            println!("{:?}",rt);
        }
    }



    #[test]
    pub fn context_test(){
        if let Ok(rt) = JSRuntime::new() {
            if let Ok(ctx) = JSContext::new(&rt) {
                println!("{:?}",ctx);
            }
        }
    }

    #[test]
    pub fn eval_test(){
        if let Ok(rt) = JSRuntime::new() {
            if let Ok(ctx) = JSContext::new(&rt) {
                println!("{:?}",ctx);

                // int
                if let Ok(res) = ctx.eval("1+10"){
                    if let JSValue::Int(_) = res {
                        let value:i32 = res.try_into().unwrap();
                        println!("Number: {}",value);
                    }
                };

                // bool
                if let Ok(res) = ctx.eval("false"){
                    if let JSValue::Bool(_) = res{
                        let value:bool = res.try_into().unwrap();
                        println!("Bool: {}",value);
                    }
                }

                // null
                if let Ok(res) = ctx.eval("null"){
                    if let JSValue::Null(_) = res{
                        let value:Option<()> = res.try_into().unwrap();
                        println!("Null: {:?}",value);
                    }
                }

                


                // f64
                if let Ok(res) = ctx.eval("1.6"){
                    if let JSValue::Float64(_) = res{
                        let value:f64 = res.try_into().unwrap();
                        println!("F64: {}",value);
                    }
                }

                // String
                if let Ok(res) = ctx.eval("'Hello.World'"){
                    if let JSValue::String(_) = res{
                        let value:String = res.try_into().unwrap();
                        println!("String: {}",value);
                    }
                }

            }
        }
    }

}