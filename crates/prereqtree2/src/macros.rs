/// Convenience routine for prereqtree testing
#[macro_export]
macro_rules! t2 {
    () => {
        crate::Tree2::None
    };
    (and, $($module:expr),*) => {
        crate::Tree2::And ( vec![ $(($module),)*] )
    };
    (or, $($module:expr),*) => {
        crate::Tree2::Or ( vec![ $(($module),)*] )
    };
    ($t:ident, $($n:expr),*) => {
        crate::Tree2::M(stringify!($t).to_string(), vec![$($n,)*])
    };
}
