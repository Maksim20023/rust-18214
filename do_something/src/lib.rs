#[macro_export]
macro_rules! do_something {
    (
        $(
            $field:ident : $typ:ty
        ),* $(,)?
    ) => {
        pub struct Data {
            $(
                pub $field: $typ,
            )*
        }

        impl Data {
            pub fn new($($field: $typ),*) -> Self {
                Data {
                    $($field),*
                }
            }
            pub fn print(&self) {
                $(
                    let _ = &self.$field;
                )*
            }
        }
    }
}
