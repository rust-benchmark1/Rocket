use rhai::{Engine, Scope};

/// Script engine for processing code execution
pub fn execute_script(script: String) -> Result<String, String> {
    let engine = Engine::new();
    let mut scope = Scope::new();
    scope.push("base", 40_i64);

    let ast = match engine.compile_expression(&script) {
        Ok(a) => a,
        Err(e) => return Err(e.to_string()),
    };

    //CWE 94
    //SINK
    match engine.run_ast_with_scope(&mut scope, &ast) {
        Ok(_) => {
            std::env::set_var("SCRIPT_OUTPUT", "success");
            Ok("Script executed".to_string())
        },
        Err(e) => Err(format!("Script error: {}", e)),
    }
}
