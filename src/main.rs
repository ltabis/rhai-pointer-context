use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    // Define external function
    fn compute_something(x: i64) -> bool {
        (x % 40) == 0
    }

    // Create scripting engine
    let mut engine = Engine::new();

    // Register external function as 'compute'
    engine.register_fn("compute", compute_something);
    engine.register_fn(
        "run",
        |ncc: rhai::NativeCallContext,
         callbacks: rhai::Array|
         -> Result<(), Box<rhai::EvalAltResult>> {
            let callbacks = callbacks
                .into_iter()
                .map(|cb| cb.cast::<rhai::FnPtr>())
                .collect::<Vec<_>>();

            for cb in callbacks {
                let _ = cb
                    .call_within_context::<rhai::Dynamic>(&ncc, ())
                    .expect("failed to run pointer");
            }

            Ok(())
        },
    );

    // Evaluate the script, expecting a 'bool' result
    let result: bool = engine.eval_file("./script.rhai".into())?;

    assert!(result);

    Ok(())
}
