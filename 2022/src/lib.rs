/// Measures the `Duration` it take to run the given expression.
/// ```
/// # #[macro_use] extern crate aoc;
/// let (result, elapsed) = bench!(1+2);
/// println!("Expression took {} seconds to run", elapsed.as_secs());
///
/// bench!(1+2, result, elapsed);
/// println!("Expression took {} seconds to run", elapsed.as_secs());
/// ```
#[macro_export]
macro_rules! bench {
    ($x:expr) => {{
        let start = std::time::Instant::now();
        let result = $x;
        let elapsed = start.elapsed();
        (result, elapsed)
    }};
    ($x:expr, $result:ident, $elapsed:ident) => {
        let ($result, $elapsed) = {
            let start = std::time::Instant::now();
            let result = $x;
            let elapsed = start.elapsed();
            (result, elapsed)
        };
    };
}
