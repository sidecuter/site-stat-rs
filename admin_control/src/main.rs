use anyhow::anyhow;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use clap::Parser;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use stat_api::entity::{user, user_role};

#[derive(Parser, Debug)]
#[command(author, version, about = "Добавляет нового пользователя в базу данных", long_about = None)]
struct Args {
    /// Логин нового пользователя
    #[arg(short, long)]
    login: String,

    /// Пароль нового пользователя
    #[arg(short, long)]
    password: String,

    /// Строка подключения к бд
    #[arg(short, long)]
    database_url: String,
}

fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("{}", e))?;
    Ok(password_hash.to_string())
}

async fn add_user_with_admin_role(
    db_conn: &DatabaseConnection,
    login: &str,
    password: &str,
) -> anyhow::Result<()> {
    let existing_user = user::Entity::find()
        .filter(user::Column::Login.eq(login))
        .one(db_conn)
        .await?;

    if existing_user.is_some() {
        return Err(anyhow!(
            "Пользователь с логином '{}' уже существует.",
            login
        ));
    }

    let hashed_password = hash_password(password)?;

    let new_user = user::ActiveModel {
        login: Set(login.to_string()),
        hash: Set(hashed_password),
        is_active: Set(true),
        ..Default::default()
    };

    let inserted_user = new_user.insert(db_conn).await?;
    println!(
        "Пользователь '{}' (ID: {}) успешно добавлен.",
        login, inserted_user.id
    );

    let admin_role_id = 1;

    let user_role_link = user_role::ActiveModel {
        user_id: Set(inserted_user.id),
        role_id: Set(admin_role_id),
    };
    user_role_link.insert(db_conn).await?;

    println!(
        "Пользователю '{}' назначена роль администратора (ID: {}).",
        login, admin_role_id
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let db_conn = Database::connect(&args.database_url).await?;

    add_user_with_admin_role(&db_conn, &args.login, &args.password).await?;

    println!("Операция завершена успешно.");

    Ok(())
}
