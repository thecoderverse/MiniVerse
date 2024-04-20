use crate::utility::get_file_path;
use warp::{http::Response, reply::Reply, Rejection};
use handlebars::Handlebars;
use warp::reject;
use std::sync::Arc;
use crate::cache::update_cache_if_needed;
use std::collections::HashMap;

pub async fn index_handler() -> Result<impl Reply, Rejection> {
    let mut handlebars = Handlebars::new();
    if handlebars
        .register_template_file("index", get_file_path("templates/index.hbs"))
        .is_err()
    {
        return Err(reject::not_found());
    }
    let handlebars = Arc::new(handlebars);

    let cache = update_cache_if_needed().await;
    let cache = cache.lock().await;
    let cached_notes = cache.as_ref().unwrap();

    let mut notes_html = String::new();

    for note in cached_notes.notes.iter() {
        notes_html.push_str(&format!(
            "<a href=\"/note/{}\">{}</a> by {} on {}/{}/{}<br>",
            note.id, note.title, note.author, note.year, note.month, note.day
        ));
    }

    let mut data = HashMap::new();
    data.insert("notes_html", notes_html);

    let rendered = match handlebars.render("index", &data) {
        Ok(rendered) => rendered,
        Err(_) => return Err(reject::not_found()),
    };

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(rendered)
        .unwrap())
}