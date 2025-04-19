#[macro_export]
macro_rules! impl_paginate {
    ($out_type:ty, $module:ident) => {
        impl Paginate<'_, $module::Entity, $module::Model> for $out_type {
            fn get_query(filter: &Filter) -> Select<$module::Entity> {
                if let Some(user_id) = filter.user_id {
                    $module::Entity::find()
                        .filter($module::Column::UserId.eq(user_id))
                        .order_by_asc($module::Column::Id)
                } else {
                    $module::Entity::find()
                        .order_by_asc($module::Column::Id)
                }
            }
        }
    };
}
