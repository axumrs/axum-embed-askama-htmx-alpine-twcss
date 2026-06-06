use askama::Template;
use axum::response::Html;

use crate::{Result, tpl};

pub async fn index() -> Result<Html<String>> {
    let tpl = tpl::index::IndexTemplate {};
    let html = tpl.render()?;
    Ok(Html(html))
}
