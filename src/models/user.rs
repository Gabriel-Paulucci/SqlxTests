use uuid::Uuid;

use crate::database::get_db;

pub struct User {
    pub id: Uuid,
    pub name: String,
}

pub async fn register_user(name: &str) {
    let db = get_db();

    let uuid = Uuid::new_v4();

    query!(
        r#"INSERT INTO "User" ( id, name ) VALUES ( $1, $2 )"#,
        uuid,
        name
    )
    .fetch_one(db)
    .await
    .unwrap();
}

pub async fn get_user(id: &Uuid) -> User {
    let db = get_db();

    let user = query_as!(
        User,
        r#"SELECT * FROM "User"
        WHERE id = $1"#,
        id
    )
    .fetch_one(db)
    .await
    .unwrap();

    user
}
