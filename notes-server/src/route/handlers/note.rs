use crate::cache::update_cache_if_needed;
use crate::entity::{Note, NoteForm};
use crate::utility::get_file_path;
use handlebars::Handlebars;
use rand::seq::SliceRandom;
use std::fs;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::{http::Response, reject, reply::Reply, Rejection};

pub async fn get_note_handler() -> Result<impl Reply, Rejection> {
    let mut handlebars = Handlebars::new();
    if handlebars
        .register_template_file("note", get_file_path("templates/note.hbs"))
        .is_err()
    {
        return Err(reject::not_found());
    }
    let handlebars = Arc::new(handlebars);

    let cache = update_cache_if_needed().await;
    let cache = cache.lock().await;
    let cached_notes = cache.as_ref().unwrap();

    let note = cached_notes.notes.choose(&mut rand::thread_rng());
    let rendered = match handlebars.render("note", &note) {
        Ok(rendered) => rendered,
        Err(_) => return Err(reject::not_found()),
    };

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(rendered)
        .unwrap())
}

pub async fn get_note_by_id_handler(id: usize) -> Result<impl Reply, Rejection> {
    let mut handlebars = Handlebars::new();
    if handlebars
        .register_template_file("note", get_file_path("templates/note.hbs"))
        .is_err()
    {
        return Err(reject::not_found());
    }
    let handlebars = Arc::new(handlebars);

    let cache = update_cache_if_needed().await;
    let cache = cache.lock().await;
    let cached_notes = cache.as_ref().unwrap();

    let note = cached_notes.notes.iter().find(|note| note.id == id);
    let rendered = match handlebars.render("note", &note) {
        Ok(rendered) => rendered,
        Err(_) => return Err(reject::not_found()),
    };

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(rendered)
        .unwrap())
}

#[derive(Debug)]
struct NoteError {
    pub message: String,
}
impl Reject for NoteError {}

pub async fn add_note_handler(note_data: NoteForm) -> Result<impl Reply, Rejection> {
    let path = get_file_path("notes.json");
    let notes_file_content = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&notes_file_content).unwrap_or_else(|_| vec![]);
    let new_note = Note {
        id: notes.len() + 1,
        title: note_data.title,
        body: note_data.body,
        publisher: note_data.publisher,
        author: note_data.author,
        media_type: note_data.media_type,
        year: note_data.year,
        month: note_data.month,
        day: note_data.day,
        externals: Option::from(note_data.externals),
    };

    notes.push(new_note);

    let path = get_file_path("notes.json");
    match fs::write(path, serde_json::to_string(&notes).unwrap()) {
        Ok(_) => Ok(warp::reply::with_status(
            "Note successfully added!",
            StatusCode::CREATED,
        )),
        Err(_) => Err(warp::reject::custom(NoteError {
            message: "Failed to write to file".to_string(),
        })),
    }
}

pub async fn delete_note_handler(id: usize) -> Result<impl Reply, Rejection> {
    let path = get_file_path("notes.json");
    let notes_file_content = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    let mut notes: Vec<Note> = serde_json::from_str(&notes_file_content).unwrap_or_else(|_| vec![]);
    let note_index = notes.iter().position(|note| note.id == id);

    let path = get_file_path("notes.json");
    match note_index {
        Some(index) => {
            notes.remove(index);
            match fs::write(path, serde_json::to_string(&notes).unwrap()) {
                Ok(_) => Ok(warp::reply::with_status(
                    "Note successfully deleted!",
                    StatusCode::OK,
                )),
                Err(_) => Err(warp::reject::custom(NoteError {
                    message: "Failed to write to file".to_string(),
                })),
            }
        }
        None => Err(warp::reject::custom(NoteError {
            message: "Note not found".to_string(),
        })),
    }
}
