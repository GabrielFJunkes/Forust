use std::{time::Duration, collections::HashMap};
use axum::{routing::{post, get}, Router, Extension, response::Redirect, Form, middleware, extract::Path};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{types::time::OffsetDateTime, Pool, MySql, Error};
use crate::{app_state::AppState, auth::{middleware::logged_in, structs::UserJWT}, component::{structs::Referer, middleware::get_referer, cookie::create_cookie}, post::structs::Comment};

use super::structs::{PostPreview, PostBody, Post, CommentSQLData, PostRanking};


pub async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Form(body): Form<PostBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let query_result = sqlx::query("INSERT INTO posts (titulo, body, usuario_id, comunidade_id, tag_id) 
                                        VALUES (?, ?, ?, ?, CASE WHEN ? = 'NULL' THEN NULL ELSE ? END)")
        .bind(body.titulo)
        .bind(body.body)
        .bind(user.id)
        .bind(body.community_id)
        .bind(&body.tag_id)
        .bind(&body.tag_id)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let mut cookie_ob = Cookie::new("success_msg", "Postagem criada com sucesso.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            Ok((jar, Redirect::to(referer.url.as_str())))
        },
        Err(_err) => {
            let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar postagem.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            Err(
                (jar,
                Redirect::to(referer.url.as_str()))
            )
        },
    }   
}

pub async fn get_posts_data(db: &Pool<MySql>, community_id: Option<i64>, user_id: Option<i64>) -> Vec<PostPreview> {
    let query: String;
    if let Some(user_id) = user_id {
        query = format!("SELECT posts.id, posts.titulo, 
            CASE
                WHEN LENGTH(posts.body) <= 100 THEN posts.body
                ELSE CONCAT(SUBSTRING(posts.body, 1, 100), '...')
            END as body, 
            uap.gostou as liked,
            usuarios.nome AS user_name, comunidades.nome as community_name, tags.nome as tag_name, posts.created_at,
            CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking
            FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id 
            JOIN comunidades ON posts.comunidade_id = comunidades.id 
            LEFT JOIN tags ON tags.id = posts.tag_id
            LEFT JOIN (
                SELECT post_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
                FROM usuarios_avaliam_posts
                GROUP BY post_id
            ) as avaliacoes ON posts.id = avaliacoes.post_id
            LEFT JOIN usuarios_avaliam_posts uap ON uap.post_id=posts.id AND uap.usuario_id = {user_id}");
    }else{
        query = String::from("SELECT posts.id, posts.titulo, 
        CASE
            WHEN LENGTH(posts.body) <= 100 THEN posts.body
            ELSE CONCAT(SUBSTRING(posts.body, 1, 100), '...')
        END as body, 
        uap.gostou as liked,
        usuarios.nome AS user_name, comunidades.nome as community_name, tags.nome as tag_name, posts.created_at,
        CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking
        FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id 
        JOIN comunidades ON posts.comunidade_id = comunidades.id 
        LEFT JOIN tags ON tags.id = posts.tag_id
        LEFT JOIN (
            SELECT post_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
            FROM usuarios_avaliam_posts
            GROUP BY post_id
        ) as avaliacoes ON posts.id = avaliacoes.post_id
        LEFT JOIN usuarios_avaliam_posts uap ON uap.post_id=posts.id AND uap.usuario_id = NULL");
    }
    let result: Result<Vec<PostPreview>, Error>;
    if let Some(community_id) = community_id {
        let query_plus = " WHERE posts.comunidade_id = ?";
        result = sqlx::query_as::<_, PostPreview>(
        &(query.to_owned()+&query_plus))
        .bind(community_id)
        .fetch_all(db)
        .await;
    } else {
        result = sqlx::query_as::<_, PostPreview>(
        &query)
        .bind(community_id)
        .fetch_all(db)
        .await;
    }

    match result {
        Ok(vec) => {
            vec
        },
        Err(err) => {
            println!("{}", err);
            [].to_vec()},
    }
}

pub async fn get_post_data(db: &Pool<MySql>, post_id: String, user_id: Option<i64>) -> Option<Post> {
    let query: String;
    if let Some(user_id) = &user_id {
        query = format!("SELECT posts.id, posts.titulo, posts.body, usuarios.nome AS user_name, 
        comunidades.nome as community_name, tags.nome as tag_name, posts.created_at,
        uap.gostou as liked,
        CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking 
        FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id 
        JOIN comunidades ON posts.comunidade_id = comunidades.id 
        LEFT JOIN tags ON tags.id = posts.tag_id 
        LEFT JOIN (
            SELECT post_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
            FROM usuarios_avaliam_posts
            GROUP BY post_id
        ) as avaliacoes ON posts.id = avaliacoes.post_id
        LEFT JOIN usuarios_avaliam_posts uap ON uap.post_id=posts.id AND uap.usuario_id = {user_id}
        WHERE posts.id = ?")
    }else{
        query = "SELECT posts.id, posts.titulo, posts.body, usuarios.nome AS user_name, 
            comunidades.nome as community_name, tags.nome as tag_name, posts.created_at,
            uap.gostou as liked,
            CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking 
            FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id 
            JOIN comunidades ON posts.comunidade_id = comunidades.id 
            LEFT JOIN tags ON tags.id = posts.tag_id 
            LEFT JOIN (
                SELECT post_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
                FROM usuarios_avaliam_posts
                GROUP BY post_id
            ) as avaliacoes ON posts.id = avaliacoes.post_id
            LEFT JOIN usuarios_avaliam_posts uap ON uap.post_id=posts.id AND uap.usuario_id = NULL
            WHERE posts.id = ?".to_owned();
    }
    let result = sqlx::query_as::<_, PostPreview>(
        &query)
        .bind(&post_id)
        .fetch_one(db)
        .await;

    match result {
        Ok(post) => {
            let comments_query: String;
            if let Some(user_id) = &user_id {
                comments_query = format!("SELECT c.id, c.body, usuarios.nome as user_name, c.created_at, 
                uac.gostou as liked,
                (SELECT GROUP_CONCAT(rc.id) 
                    FROM comentarios rc 
                    WHERE rc.comentario_id = c.id) AS answers_string,
                    CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking 
                FROM comentarios c JOIN usuarios ON usuarios.id = c.usuario_id 
                LEFT JOIN (
                    SELECT uac.comentario_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
                    FROM usuarios_avaliam_comentarios uac
                    GROUP BY uac.comentario_id
                ) as avaliacoes ON c.id = avaliacoes.comentario_id
                LEFT JOIN usuarios_avaliam_comentarios uac ON uac.comentario_id=c.id AND uac.usuario_id = {user_id}
                WHERE post_id = ? AND c.comentario_id IS NULL");
            }else{
                comments_query = "SELECT c.id, c.body, usuarios.nome as user_name, c.created_at,
                uac.gostou as liked, 
                (SELECT GROUP_CONCAT(rc.id) 
                    FROM comentarios rc 
                    WHERE rc.comentario_id = c.id) AS answers_string,
                    CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking 
                FROM comentarios c JOIN usuarios ON usuarios.id = c.usuario_id 
                LEFT JOIN (
                    SELECT uac.comentario_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
                    FROM usuarios_avaliam_comentarios uac
                    GROUP BY uac.comentario_id
                ) as avaliacoes ON c.id = avaliacoes.comentario_id
                LEFT JOIN usuarios_avaliam_comentarios uac ON uac.comentario_id=c.id AND uac.usuario_id = NULL
                WHERE post_id = ? AND c.comentario_id IS NULL".to_owned();
            }
            let result = sqlx::query_as::<_, CommentSQLData>(&comments_query)
            .bind(&post_id)
            .fetch_all(db)
            .await;
            let comments = match result {
                Ok(comments) => {
                    comments.into_iter().map(|data| {
                        let answers_id = if let Some(data) = data.answers_string {
                            data
                            .split(',')
                            .filter_map(|s| s.parse::<i64>().ok())
                            .collect::<Vec<i64>>()
                        }else{
                            [].to_vec()
                        };
                        
                        Comment {
                            id: data.id,
                            body: data.body,
                            user_name: data.user_name,
                            created_at: data.created_at,
                            ranking: data.ranking,
                            liked: data.liked,
                            answers_id
                        }
                    }).collect()
                },
                Err(_) => {
                    [].to_vec()},
            };
            let answers_query: String;
            if let Some(user_id) = &user_id {
                answers_query = format!("SELECT c.id, c.body, usuarios.nome as user_name, c.created_at,
                uac.gostou as liked, 
                (SELECT GROUP_CONCAT(rc.id) 
                    FROM comentarios rc 
                    WHERE rc.comentario_id = c.id) AS answers_string,
                    CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking 
                FROM comentarios c JOIN usuarios ON usuarios.id = c.usuario_id 
                LEFT JOIN (
                    SELECT uac.comentario_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
                    FROM usuarios_avaliam_comentarios uac
                    GROUP BY uac.comentario_id
                ) as avaliacoes ON c.id = avaliacoes.comentario_id
                LEFT JOIN usuarios_avaliam_comentarios uac ON uac.comentario_id=c.id AND uac.usuario_id = {user_id}
                WHERE post_id = ? AND c.comentario_id IS NOT NULL");
            }else{
                answers_query = "SELECT c.id, c.body, usuarios.nome as user_name, c.created_at,
                uac.gostou as liked, 
                (SELECT GROUP_CONCAT(rc.id) 
                    FROM comentarios rc 
                    WHERE rc.comentario_id = c.id) AS answers_string,
                    CAST(COALESCE(avaliacoes.count, 0) AS SIGNED) as ranking 
                FROM comentarios c JOIN usuarios ON usuarios.id = c.usuario_id 
                LEFT JOIN (
                    SELECT uac.comentario_id, SUM(CASE WHEN gostou = true THEN 1 ELSE -1 END) as count
                    FROM usuarios_avaliam_comentarios uac
                    GROUP BY uac.comentario_id
                ) as avaliacoes ON c.id = avaliacoes.comentario_id
                LEFT JOIN usuarios_avaliam_comentarios uac ON uac.comentario_id=c.id AND uac.usuario_id = NULL
                WHERE post_id = ? AND c.comentario_id IS NOT NULL".to_owned();
            }
            
            let result = sqlx::query_as::<_, CommentSQLData>(&answers_query)
            .bind(post_id)
            .fetch_all(db)
            .await;
            let answers = match result {
                Ok(comments) => {
                    let answers: HashMap<i64, Comment> = comments.iter().enumerate().map(|(_, x)| {
                        let answers_id = if let Some(data) = &x.answers_string {
                            data
                            .split(',')
                            .filter_map(|s| s.parse::<i64>().ok())
                            .collect::<Vec<i64>>()
                        }else{
                            [].to_vec()
                        };
                        let comment = Comment {
                            id: x.id,
                            body: x.body.clone(),
                            user_name: x.user_name.clone(),
                            created_at: x.created_at,
                            ranking: x.ranking,
                            liked: x.liked,
                            answers_id,
                        };
                        
                        (x.id, comment)
                    }).collect();
                    answers
                }
                Err(err) => {
                    println!("{err}");
                    HashMap::new()
                }
            };

            Some(Post{
                id: post.id,
                titulo: post.titulo,
                body: post.body,
                user_name: post.user_name,
                community_name: post.community_name,
                tag_name: post.tag_name,
                created_at: post.created_at,
                ranking: post.ranking,
                comments,
                answers,
                liked: post.liked
            })
        },
        Err(_) => {
            None},
    }
}

async fn avaliate(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path((id, ranking_type)): Path<(String, String)>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let ranking_type = if ranking_type=="like"{
        true
    }else{
        false
    };

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;

    let query_result = sqlx::query_as::<_, PostRanking>
        ("SELECT * FROM usuarios_avaliam_posts WHERE post_id = ? AND usuario_id = ?")
        .bind(&id)
        .bind(user.id)
        .fetch_one(&state.db)
        .await;

    match query_result {
        Ok(comment) => {
            if comment.gostou==ranking_type {
                let _query_result = sqlx::query(
                    "DELETE FROM usuarios_avaliam_posts WHERE post_id = ? AND usuario_id = ?")
                    .bind(id)
                    .bind(user.id)
                    .execute(&state.db)
                    .await;
                let jar = jar.add(create_cookie("success_msg", "Avaliação apagada com sucesso.", url));
                Ok((jar, Redirect::to(referer)))
            }else{
                let _query_result = sqlx::query(
                    "UPDATE usuarios_avaliam_posts SET gostou = ? WHERE post_id = ? AND usuario_id = ?")
                    .bind(ranking_type)
                    .bind(id)
                    .bind(user.id)
                    .execute(&state.db)
                    .await;
                let jar = jar.add(create_cookie("success_msg", "Postagem avaliada com sucesso.", url));
                Ok((jar, Redirect::to(referer)))
            }
        },
        Err(_) => {
            let query_result = sqlx::query(
                "INSERT INTO usuarios_avaliam_posts (post_id, usuario_id, gostou) VALUES (?, ?, ?)")
                .bind(&id)
                .bind(user.id)
                .bind(ranking_type)
                .execute(&state.db)
                .await;
        
        
            match query_result {
                Ok(_) => {
                    let jar = jar.add(create_cookie("success_msg", "Postagem avaliada com sucesso.", url));
                    Ok((jar, Redirect::to(referer)))
                },
                Err(_) => {
                    let jar = jar.add(create_cookie("error_msg", "Erro ao avaliar postagem.", url));
                    Err(
                        (jar,
                        Redirect::to(referer))
                    )       
                }
            }  
        },
    }

     
}

pub fn create_post_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/:id/avaliar/:ranking_type", get(avaliate))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}
