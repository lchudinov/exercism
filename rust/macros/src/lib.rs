#[macro_export]
macro_rules! hashmap {
    () => (
        ::std::collections::HashMap::new()
    );
    ($($key:expr => $val:expr),+) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $val);
            )*
            m
        }
    };
    ($($key:expr => $val:expr,)*) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $val);
            )*
            m
        }
    };
}
