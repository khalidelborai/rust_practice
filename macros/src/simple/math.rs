#[macro_export]
macro_rules! add {
    // matches a single expression
    ($x:expr) => {
        $x
    };

    // matches two expressions
    ($x:expr, $y:expr) => {
        $x + $y
    };

    // matches multiple expressions using the $(...),+ syntax
    (
        $($x:expr),+ // 1 or more expressions
    ) => {
        // the code block that follows the pattern
        // the $x is the expression that will be added to sum
        // sum is initialized to 0
        // doesn't get back ti 0 because it's not a recursive macro
        // it's a pattern matching macro
        {
            let mut sum = 0;
            $(
                sum += $x;
            )+
            sum
        }
    };
}
