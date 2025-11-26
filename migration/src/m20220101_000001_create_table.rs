use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // สร้าง users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("uuid_generate_v4()".into())),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .to_owned(),
            )
            .await?;

        // สร้าง todos table
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(SimpleExpr::Custom("uuid_generate_v4()".into())),
                    )
                    .col(ColumnDef::new(Todos::UserId).uuid().not_null())
                    .col(ColumnDef::new(Todos::Title).string().not_null())
                    .col(ColumnDef::new(Todos::Description).text())
                    .col(
                        ColumnDef::new(Todos::Completed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Todos::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .col(
                        ColumnDef::new(Todos::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".into())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todos::Table, Todos::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // สร้าง index
        manager
            .create_index(
                Index::create()
                    .name("idx_todos_user_id")
                    .table(Todos::Table)
                    .col(Todos::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todos::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Todos {
    Table,
    Id,
    UserId,
    Title,
    Description,
    Completed,
    CreatedAt,
    UpdatedAt,
}
