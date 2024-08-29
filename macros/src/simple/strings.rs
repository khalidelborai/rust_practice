// Repeat a string n times
#[macro_export]
macro_rules! repeat {
    ($count:expr, $expr:expr) => {{
        let mut vec = Vec::new();
        for _ in 0..$count {
            vec.push($expr);
        }
        vec
    }};
}

#[macro_export]
macro_rules! trim_and_split {
    ($s:expr, $delim:expr) => {
        $s.trim().split($delim).collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! trim_and_split_on {
    ($s:expr, $trim_char:expr ,$delim:expr) => {
        $s.trim_matches($trim_char)
            .split($delim)
            .collect::<Vec<_>>()
    };
}
