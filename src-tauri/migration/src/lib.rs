pub use sea_orm_migration::prelude::*;

pub(crate) mod m20230813_000001_create_table;
mod m20231204_162103_add_ncm_src;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20230813_000001_create_table::Migration),
            Box::new(m20231204_162103_add_ncm_src::Migration),
        ]
  }
}
