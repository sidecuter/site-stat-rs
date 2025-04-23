#[macro_export]
macro_rules! filter_visit {
    ($self:expr, $start:expr, $end:expr; {
        $($variant:ident => $entity:ident),+
        $(,)?
    }) => {
        match $self {
            $(
                Self::$variant(q) => Self::$variant(
                    q.filter($entity::Column::VisitDate.between($start, $end))
                ),
            )+
        }
    };
}

#[macro_export]
macro_rules! build_query {
    ($input:expr => $output_enum:path {
        $($variant:ident => $entity:ident),+
        $(,)?
    }) => {
        match $input {
            $(
                Target::$variant => <$output_enum>::$variant(
                    $entity::Entity::find()
                        .select_only()
                        .column($entity::Column::UserId)
                ),
            )+
        }
    };
}
