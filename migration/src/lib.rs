pub use sea_orm_migration::prelude::*;


pub mod data;
mod m20220101_000001_create_table;
mod m20250107_204426_create_site_stat_table;
mod m20250108_120158_create_auds_table;
mod m20250108_120630_populate_auds_table;
mod m20250108_130408_create_select_aud_table;
mod m20250108_212444_create_start_way_table;
mod m20250109_160455_create_plan_table;
mod m20250109_160938_populate_plans_table;
mod m20250109_162921_create_change_plan_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250107_204426_create_site_stat_table::Migration),
            Box::new(m20250108_120158_create_auds_table::Migration),
            Box::new(m20250108_120630_populate_auds_table::Migration),
            Box::new(m20250108_130408_create_select_aud_table::Migration),
            Box::new(m20250108_212444_create_start_way_table::Migration),
            Box::new(m20250109_160455_create_plan_table::Migration),
            Box::new(m20250109_160938_populate_plans_table::Migration),
            Box::new(m20250109_162921_create_change_plan_table::Migration),
        ]
    }
}
