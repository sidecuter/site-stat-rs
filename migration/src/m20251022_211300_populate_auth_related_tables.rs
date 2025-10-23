use super::m20251018_061409_add_user_auth_related_models::{Goal, Right, Role, RoleRightGoal};
use crate::data::{GOALS, RIGHTS, ROLES, ROLE_RIGHT_GOALS};
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let insert_role = {
            let mut insert = Query::insert()
                .into_table(Role::Table)
                .columns([Role::Id, Role::Name])
                .to_owned();
            for (name, id) in ROLES {
                insert = insert.values_panic([id.into(), name.into()]).to_owned();
            }
            insert
        };
        let insert_right = {
            let mut insert = Query::insert()
                .into_table(Right::Table)
                .columns([Right::Id, Right::Name])
                .to_owned();
            for (name, id) in RIGHTS {
                insert = insert
                    .values_panic([id.into(), name.into()])
                    .to_owned();
            }
            insert
        };
        let insert_goal = {
            let mut insert = Query::insert()
                .into_table(Goal::Table)
                .columns([Goal::Id, Goal::Name])
                .to_owned();
            for (name, id) in GOALS {
                insert = insert
                    .values_panic([id.into(), name.into()])
                    .to_owned();
            }
            insert
        };
        let insert_role_right_goal = {
            let mut insert = Query::insert()
                .into_table(RoleRightGoal::Table)
                .columns([RoleRightGoal::RoleId, RoleRightGoal::RightId, RoleRightGoal::GoalId])
                .to_owned();
            for (role_id, right_id, goal_id) in ROLE_RIGHT_GOALS {
                insert = insert
                    .values_panic([role_id.into(), right_id.into(), goal_id.into()])
                    .to_owned();
            }
            insert
        };
        manager.exec_stmt(insert_role).await?;
        manager.exec_stmt(insert_right).await?;
        manager.exec_stmt(insert_goal).await?;
        manager.exec_stmt(insert_role_right_goal).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let role_query = Query::delete().from_table(Role::Table).to_owned();
        let right_query = Query::delete().from_table(Right::Table).to_owned();
        let goal_query = Query::delete().from_table(Goal::Table).to_owned();
        let role_right_goal_query = Query::delete().from_table(RoleRightGoal::Table).to_owned();
        manager.exec_stmt(role_right_goal_query).await?;
        manager.exec_stmt(goal_query).await?;
        manager.exec_stmt(right_query).await?;
        manager.exec_stmt(role_query).await?;
        Ok(())
    }
}
