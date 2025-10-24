use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Goal::Table)
                    .if_not_exists()
                    .col(pk_auto(Goal::Id))
                    .col(string(Goal::Name))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Right::Table)
                    .if_not_exists()
                    .col(pk_auto(Right::Id))
                    .col(string(Right::Name))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(string(Role::Name))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(RoleRightGoal::Table)
                    .if_not_exists()
                    .col(integer(RoleRightGoal::RoleId).not_null())
                    .col(integer(RoleRightGoal::RightId).not_null())
                    .col(integer(RoleRightGoal::GoalId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .to(Role::Table, Role::Id)
                            .from(RoleRightGoal::Table, RoleRightGoal::RoleId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(Right::Table, Right::Id)
                            .from(RoleRightGoal::Table, RoleRightGoal::RightId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(Goal::Table, Goal::Id)
                            .from(RoleRightGoal::Table, RoleRightGoal::GoalId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(RoleRightGoal::RoleId)
                            .col(RoleRightGoal::RightId)
                            .col(RoleRightGoal::GoalId),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Login))
                    .col(string(User::Hash))
                    .col(ColumnDef::new(User::Token).string().null())
                    .col(boolean(User::IsActive))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(integer(UserRole::UserId).not_null())
                    .col(integer(UserRole::RoleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .to(Role::Table, Role::Id)
                            .from(UserRole::Table, UserRole::RoleId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(User::Table, User::Id)
                            .from(UserRole::Table, UserRole::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(Index::create().col(UserRole::UserId).col(UserRole::RoleId))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RoleRightGoal::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Goal::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Right::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Goal {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Right {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum RoleRightGoal {
    Table,
    RoleId,
    RightId,
    GoalId,
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Login,
    Hash,
    Token,
    IsActive,
}

#[derive(DeriveIden)]
pub enum UserRole {
    Table,
    UserId,
    RoleId,
}
