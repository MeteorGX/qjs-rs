#[cfg(test)]
mod tests {
    use qjs_rs::{JSContext, JSRuntime, JSAny, JSRawType};

    #[test]
    pub fn runtime_test() {
        if let Ok(rt) = JSRuntime::new() {
            println!("Runtime = {:?}", rt);
        }

        if let Ok(rt) = JSRuntime::new() {
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


    #[test]
    pub fn value_test() {
        if let Ok(rt) = JSRuntime::new() {
            if let Ok(ctx) = JSContext::new(&rt) {
                let script = "type.js";
                println!("Context = {:?}", ctx);


                // number
                let res = ctx.eval(script, "1+3");
                assert!(res.is_ok());
                if let JSAny::Int(v) = res.unwrap() {
                    let v: i32 = v.into();
                    println!("Int: {}", v);
                }


                // bool
                let res = ctx.eval(script, "true");
                assert!(res.is_ok());
                if let JSAny::Bool(v) = res.unwrap() {
                    let v: bool = v.into();
                    println!("Bool: {}", v);
                }

                // null
                let res = ctx.eval(script, "null");
                assert!(res.is_ok());
                if let JSAny::Null(v) = res.unwrap() {
                    println!("IsNull: {}", v.is_null());
                }

                // undefined
                let res = ctx.eval(script, "undefined");
                assert!(res.is_ok());
                if let JSAny::Undefined(v) = res.unwrap() {
                    println!("IsUndefined: {}", v.is_undefined());
                }

                // undefined
                let res = ctx.eval(script, "undefined");
                assert!(res.is_ok());
                if let JSAny::Undefined(v) = res.unwrap() {
                    println!("IsUndefined: {}", v.is_undefined());
                }

                // exception
                let res = ctx.eval(script, "this exception!!!\r\n");
                assert!(res.is_ok());
                if let JSAny::Exception(v) = res.unwrap() {
                    println!("IsExcept: {}", v.is_exception());
                }

                // float64
                let res = ctx.eval(script, "3.14");
                assert!(res.is_ok());
                if let JSAny::Float64(v) = res.unwrap() {
                    let v: f64 = v.into();
                    println!("Float: {}", v);
                }


                // string
                let res = ctx.eval(script, "'Hello.world!'");
                assert!(res.is_ok());
                if let JSAny::String(v) = res.unwrap() {
                    if let Ok(s) = v.to_string() {
                        println!("String: {}", s);
                    }
                }

                // memory
                ctx.dump_mem();
            }
        }
    }


    #[test]
    pub fn prop_test() {
        if let Ok(rt) = JSRuntime::new() {
            if let Ok(ctx) = JSContext::new(&rt) {
                let script = "type.js";
                println!("Context = {:?}", ctx);


                let global_name = "_G_INT";
                let res = ctx.global_val(global_name, 111);
                assert!(res.is_ok());

                // call first
                let res = ctx.eval(script, global_name);
                if let JSAny::Int(v) = res.unwrap() {
                    let v: i32 = v.into();
                    println!("G INT[First] = {}", v);
                }

                // call double
                let res = ctx.eval(script, global_name);
                if let JSAny::Int(v) = res.unwrap() {
                    let v: i32 = v.into();
                    println!("G INT[Two] = {}", v);
                }

                // env array
                let vars:Vec<String> = std::env::args().collect();
                let global_name = "__ENV__";
                let res = ctx.global_val(global_name, vars);
                assert!(res.is_ok());

                // print first
                let res = ctx.eval(script, global_name);
                assert!(res.is_ok());
                if let JSAny::Array(arr) = res.unwrap(){
                    println!("ARRAY[FIRST] = {:?}",arr);
                }


                // print two
                let res = ctx.eval(script, global_name);
                assert!(res.is_ok());
                if let JSAny::Array(arr) = res.unwrap(){
                    println!("ARRAY[TWO] = {:?}",arr);
                }



                ctx.dump_mem();
            }

        }
    }
}