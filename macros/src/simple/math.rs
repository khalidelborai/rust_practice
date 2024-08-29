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

#[macro_export]
macro_rules! add_as {
    // matches a single expression and a type
    ($x:expr, $y:ty) => {
        $x as $y
    };

    // matches two expressions and a type
    ($x:expr, $y:expr, $z:ty) => {
        ($x + $y) as $z
    };

    // matches three expressions and a type to remove ambiguity with the previous pattern
    ($x:expr, $y:expr, $z:expr, $w:ty) => {
        ($x + $y + $z) as $w
    };

    // matches multiple expressions and a type
    (
        $($x:expr),+, $z:ty
    ) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )+
            sum as $z
        }
    };
}
