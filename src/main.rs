use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Expr {
    Int(u32),
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>)
}

#[derive(Clone, Debug)]
enum Value {
    VInt(u32),
    VClosure(HashMap<String, Value>, String, Box<Expr>)
}

fn eval(expr: Expr, context: HashMap<String, Value>) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(Value::VInt(n)),
        Expr::Var(name) => match context.get(&name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Variable {} not found", name))
        },
        Expr::Abs(param, body) => Ok(Value::VClosure(context, param, body)),
        Expr::App(f, arg) => match eval(*f, context.clone()) {
            Ok(f_value) => match eval(*arg, context.clone()) {
                Ok(arg_value) => match f_value {
                    Value::VClosure(ctx, param, body) => {
                        let mut new_ctx = ctx;

                        new_ctx.insert(param, arg_value);

                        eval(*body, new_ctx)
                    }
                    Value::VInt(_) => Err("Int is not a function (JS Feelings)".into())
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

fn main() {
    let my_id = Expr::Abs("x".into(), Box::new(Expr::Var("x".into())));

    let call_id = Expr::App(Box::new(my_id), Box::new(Expr::Int(1)));

    println!("{:?}", eval(call_id, HashMap::new()))
}
