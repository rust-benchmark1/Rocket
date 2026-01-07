/// Loop engine for processing iteration operations
pub fn execute_loop_operation(iteration_count: usize) -> Result<String, String> {
    let mut count = 0;

    std::iter::repeat_with(|| "item")
    //CWE 606
    //SINK
        .take(iteration_count)
        .for_each(|_| {
            count += 1;
            std::env::set_var("CURRENT_OFFSET", count.to_string());
        });

    Ok(format!("Loop executed {} iterations", count))
}
