use serde::Deserialize;


#[derive(Deserialize)]
pub struct CommentForm {
    pub body: String
}