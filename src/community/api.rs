use sqlx::{Pool, MySql};

use super::structs::{Community, CommunityData, Tag};


pub async fn get_community_data(db: &Pool<MySql>, name: &String) -> Option<Community> {
    let query_result = sqlx::query_as::<_, CommunityData>("SELECT id, nome, `desc` FROM comunidades WHERE nome=?")
    .bind(name)
    .fetch_one(db)
    .await;
    match query_result {
        Ok(result) => {
            let tags_query = sqlx::query_as::<_, Tag>(
                r#"
                SELECT id, nome
                FROM tags
                WHERE comunidade_id = ? AND status = TRUE
                "#,
            )
            .bind(result.id)
            .fetch_all(db)
            .await;
            
            let tags: Vec<Tag> = match tags_query {
                Ok(vec) => vec,
                Err(_) => [].to_vec(),
            };

            Some(Community {
                id: result.id,
                nome: result.nome,
                desc: result.desc,
                tags
            })
        }
        Err(_) => {
            None
        },
    }
}
