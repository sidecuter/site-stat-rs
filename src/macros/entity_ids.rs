#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! create_entity_with_id {
    ($(($name:ident, $id:expr)),* $(,)?) => {
        $(
            pub struct $name;

            #[allow(dead_code)]
            impl crate::traits::EntityId for $name {
                const ID: i32 = $id;
            }
        )*
    };
}
