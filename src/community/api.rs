use std::time::Duration;

use axum::{Extension, Form, response::Redirect, Router, middleware, routing::{post, get}, extract::Path};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{Pool, MySql, types::time::OffsetDateTime};

use crate::{app_state::AppState, auth::{structs::UserJWT, middleware::logged_in}, component::{structs::Referer, cookie::create_cookie}};

use super::structs::{Community, CommunityData, Tag, CommunityBody, FollowedCommunityData, Follow, TagBody, TagBodyWithName, CommunityBodyEdit, User, AdminsCount};

async fn community_admins_count(
    db: &Pool<MySql>,
    id: &String
) -> Option<i64> {
    let query_result = sqlx::query_as::<_,AdminsCount>(
        "SELECT COUNT(comunidade_id) AS count FROM inscricoes WHERE comunidade_id = ? AND admin = TRUE")
        .bind(id)
        .fetch_one(db)
        .await;
    match query_result {
        Ok(result) => Some(result.count),
        Err(_) => None,
    }
}

pub async fn edit(
    Extension(state): Extension<AppState>, 
    Extension(referer): Extension<Referer>, 
    Path(nome): Path<String>,
    jar: CookieJar,
    Form(body): Form<CommunityBodyEdit>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    let query_result = sqlx::query("UPDATE comunidades SET `desc` = ? WHERE nome = ?")
        .bind(body.desc)
        .bind(&nome)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let path = format!("/f/{}", nome);
            let jar = jar.add(create_cookie("success_msg", "Comunidade editada com sucesso.", path.clone()));
            Ok((jar, Redirect::to(&path)))
        },
        Err(_) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao editar comunidade.", url));
            Err(
                (jar,
                Redirect::to(referer))
            )
        },
    }   
}

pub async fn edit_tag(
    Extension(state): Extension<AppState>, 
    Extension(referer): Extension<Referer>, 
    Path((_post_id, id)): Path<(String,String)>,
    jar: CookieJar,
    Form(body): Form<TagBodyWithName>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    let query_result = sqlx::query("UPDATE tags SET nome = ? WHERE id = ?")
        .bind(body.nome)
        .bind(&id)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let path = format!("/f/{}", body.nome_comunidade);
            let jar = jar.add(create_cookie("success_msg", "Tag editada com sucesso.", path.clone()));
            Ok((jar, Redirect::to(&path)))
        },
        Err(_) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao editar tag.", url));
            Err(
                (jar,
                Redirect::to(referer))
            )
        },
    }   
}

pub async fn get_tag_data(db: &Pool<MySql>, id: &String) -> Option<Tag> {
    let query_result = sqlx::query_as::<_, Tag>("SELECT id, nome, status FROM tags WHERE id=?")
    .bind(id)
    .fetch_one(db)
    .await;
    match query_result {
        Ok(result) => {
            Some(result)
        }
        Err(_) => {
            None
        },
    }
}

pub async fn get_community_users(db: &Pool<MySql>, id: i64) -> Vec<User> {
    let query_result = sqlx::query_as::<_, User>("SELECT id, nome, i.admin FROM usuarios u
    JOIN inscricoes i ON u.id = i.usuario_id
    WHERE i.comunidade_id = ? AND u.nome!='[Removido]'
    ORDER BY i.admin DESC, u.nome")
    .bind(id)
    .fetch_all(db)
    .await;
    match query_result {
        Ok(result) => result,
        Err(_) => [].to_vec()
    }
}

pub async fn get_community_data(db: &Pool<MySql>, name: &String) -> Option<Community> {
    let query_result = sqlx::query_as::<_, CommunityData>("SELECT id, nome, `desc` FROM comunidades WHERE nome=?")
    .bind(name)
    .fetch_one(db)
    .await;
    match query_result {
        Ok(result) => {
            let tags_query = sqlx::query_as::<_, Tag>(
                r#"
                SELECT id, nome, status
                FROM tags
                WHERE comunidade_id = ?
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

pub async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Form(body): Form<CommunityBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(5);

    let query_result = sqlx::query("INSERT INTO comunidades (comunidades.nome, comunidades.desc) VALUES (?, ?)")
        .bind(body.nome)
        .bind(body.desc)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(com) => {
            let _ = sqlx::query("INSERT INTO inscricoes (usuario_id, comunidade_id, admin) VALUES (?, ?, TRUE)")
            .bind(user.id)
            .bind(com.last_insert_id())
            .execute(&state.db)
            .await;
            let mut cookie_ob = Cookie::new("success_msg", "Comunidade criada com sucesso.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            Ok((jar, Redirect::to(referer.url.as_str())))
        },
        Err(_err) => {
            let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar comunidade.");
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

pub async fn create_tag(
    Extension(state): Extension<AppState>,
    jar: CookieJar, 
    Extension(referer): Extension<Referer>,
    Path(id): Path<String>,
    Form(body): Form<TagBody>, ) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    if body.nome=="[Removido]" {
        let jar = jar.add(create_cookie("error_msg", "Erro ao criar tag. Tag não pode ser \"[Removido]\".", url));
        return Err((jar,Redirect::to(referer)))
    }

    if body.nome.len()<2 {
        let jar = jar.add(create_cookie("error_msg", "Erro ao criar tag. Tag deve conter pelo menos 3 caracteres.", url));
        return Err((jar,Redirect::to(referer)))
    }

    let query_result = sqlx::query("INSERT INTO tags (nome, comunidade_id) VALUES (?, ?)")
        .bind(body.nome)
        .bind(id)
        .execute(&state.db)
        .await;
    match query_result {
        Ok(_) => {
            let cookie = jar.add(create_cookie("success_msg", "Categoria cadastrada com sucesso.", url));
            Ok((cookie, Redirect::to(referer)))
        },
        Err(_err) => {
            let cookie = jar.add(create_cookie("error_msg", "Falha ao cadastrar categoria.", url));
            Err((cookie, Redirect::to(referer)))},
    }
}

pub async fn get_user_followed_communities(db: &Pool<MySql>, user_id: i64) -> Vec<FollowedCommunityData> {
    let query_result = sqlx::query_as::<_, FollowedCommunityData>("SELECT comunidades.id, comunidades.nome, comunidades.desc, inscricoes.admin FROM inscricoes JOIN comunidades ON comunidades.id = inscricoes.comunidade_id WHERE usuario_id=?")
    .bind(user_id)
    .fetch_all(db)
    .await;
    match query_result {
        Ok(vec) => vec,
        Err(_) => [].to_vec(),
    }
} 

pub async fn get_if_follows(user_id: i64, community_id: &String, db: &Pool<MySql>) -> Option<Follow> {
    let query_result = sqlx::query_as::<_, Follow>("SELECT usuario_id, comunidade_id, admin FROM inscricoes WHERE usuario_id=? AND comunidade_id=?")
    .bind(user_id)
    .bind(community_id)
    .fetch_optional(db)
    .await;

    match query_result {
        Ok(follow) => follow,
        Err(_) => None
    }
}

async fn delete_tag(
    Extension(state): Extension<AppState>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path((_, tag_id, tipo)): Path<(String, String, String)>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    
    let tipo = tipo=="ativar";

    let query_result = sqlx::query(
    "UPDATE tags SET status = ? WHERE id = ?")
    .bind(tipo)
    .bind(tag_id)
    .execute(&state.db)
    .await;

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    match query_result {
        Ok(_) => {
            let jar = jar.add(create_cookie("success_msg", "Status da tag atualizado com sucesso.", url));
            Ok((jar, Redirect::to(referer)))
        },
        Err(_) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao atualizar status tag.", url));
            Err((jar, Redirect::to(referer)))
        },
    }

}

pub async fn inscrever(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>,
    Extension(referer): Extension<Referer>,
    jar: CookieJar,
    Path(id): Path<String>
) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    let follow = get_if_follows(user.id, &id, &state.db).await;

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;

    match follow {
        Some(result) => {
            if result.admin {
                let count = community_admins_count(&state.db, &id).await;
                match count {
                    Some(count) => {
                        if count<2 {
                            let cookie = jar.add(create_cookie("error_msg", "Erro ao deixar de seguir comunidade. Você é o único admin.", url));
                            return Err((cookie, Redirect::to(referer)))
                        }else{
                            let delete_result = sqlx::query("DELETE FROM inscricoes WHERE usuario_id=? AND comunidade_id=?")
                                .bind(user.id)
                                .bind(id)
                                .execute(&state.db)
                                .await;   
                                
                                match delete_result {
                                    Ok(_) => {
                                        let cookie = jar.add(create_cookie("success_msg", "Você deixou de seguir essa comunidade.", url));
                                        return Ok((cookie, Redirect::to(referer)))
                                    },
                                    Err(_) => {
                                        let cookie = jar.add(create_cookie("error_msg", "Erro ao deixar de seguir comunidade.", url));
                                        return Err((cookie, Redirect::to(referer)))
                                    },
                                }
                        }
                    },
                    None => {
                        let cookie = jar.add(create_cookie("error_msg", "Erro ao deixar de seguir comunidade.", url));
                        Err((cookie, Redirect::to(referer)))
                    }
                }
            }else{
                let delete_result = sqlx::query("DELETE FROM inscricoes WHERE usuario_id=? AND comunidade_id=?")
                .bind(user.id)
                .bind(id)
                .execute(&state.db)
                .await;   
                
                match delete_result {
                    Ok(_) => {
                        let cookie = jar.add(create_cookie("success_msg", "Você deixou de seguir essa comunidade.", url));
                        Ok((cookie, Redirect::to(referer)))
                    },
                    Err(_) => {
                        let cookie = jar.add(create_cookie("error_msg", "Erro ao deixar de seguir comunidade.", url));
                        Err((cookie, Redirect::to(referer)))
                    },
                }
            }
        },
        None => {
            let _ = sqlx::query("INSERT INTO inscricoes (usuario_id, comunidade_id) VALUES (?, ?)")
            .bind(user.id)
            .bind(id)
            .execute(&state.db)
            .await;

            let cookie = jar.add(create_cookie("success_msg", "Comunidade seguida com sucesso.", url));
            Ok((cookie, Redirect::to(referer)))
        },
    }
}

async fn admin(Extension(state): Extension<AppState>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path((id, user_id, tipo)): Path<(String, String, String)>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    
    let tipo = if tipo=="add"{
        true
    }else{
        false
    };

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;

    if !tipo {
        let count = community_admins_count(&state.db, &id).await;
        if let Some(count) = count {
            if count<2 {
                let cookie = jar.add(create_cookie("error_msg", "Erro ao realizar ação. Este é o único admin.", url));
                return Err((cookie, Redirect::to(referer)))
            }
        }
    }

    let query_result = sqlx::query(
        "UPDATE inscricoes SET admin = ? WHERE comunidade_id = ? AND usuario_id = ?")
        .bind(tipo)
        .bind(id)
        .bind(user_id)
        .execute(&state.db)
        .await;
    match query_result {
        Ok(_) => {
            let jar = jar.add(create_cookie("success_msg", "Ação realizada com sucesso.", url));
            Ok((jar, Redirect::to(referer)))
        },
        Err(_) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao realizar ação.", url));
            Err((jar, Redirect::to(referer)))
        },
    }
}

pub fn create_community_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/:id", post(edit))
        .route("/:id/admin/:id/:tipo", get(admin))
        .route("/:id/tag", post(create_tag))
        .route("/:id/seguir", get(inscrever))
        .route("/:id/tag/:tag_id", post(edit_tag))
        .route("/:id/tag/:tag_id/:tipo", get(delete_tag))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}