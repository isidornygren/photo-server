use actix_web::{
    get,
    http::header::ContentType,
    web::{Data, Query},
    HttpRequest, HttpResponse, Responder,
};
use image::image_dimensions;

use crate::{
    image::{load_image, ImageTransformOptions},
    WebContext,
};

const VALID_IMAGE_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "bmp"];

#[get("/")]
pub async fn hello(context: Data<WebContext>) -> impl Responder {
    HttpResponse::Ok().body(context.index.clone())
}

#[get("/{image_path:.*}")]
pub async fn images(
    request: HttpRequest,
    query: Query<ImageTransformOptions>,
    context: Data<WebContext>,
) -> impl Responder {
    let path: std::path::PathBuf = request.match_info().query("image_path").parse().unwrap();
    let built_path = context.path.join(&path);

    if !built_path.exists() {
        return HttpResponse::NotFound().body("Not found");
    }

    if image_dimensions(&built_path).is_err() {
        return HttpResponse::BadRequest().body("File not an image");
    }

    let maybe_buffer = load_image(&built_path, query.into_inner()).unwrap();

    return HttpResponse::Ok()
        .insert_header(ContentType::png())
        .body(maybe_buffer);
}

#[get("/random")]
pub async fn random(
    query: Query<ImageTransformOptions>,
    context: Data<WebContext>,
) -> impl Responder {
    use rand::prelude::IteratorRandom;
    use walkdir::WalkDir;

    let mut rng = rand::thread_rng();

    let files = WalkDir::new(&context.path).into_iter().filter_map(|e| {
        if let Ok(entry) = e {
            if let Some(extension) = entry.path().extension() {
                if let Some(e_unwrapped) = extension.to_str() {
                    if VALID_IMAGE_EXTENSIONS.contains(&e_unwrapped) {
                        return Some(entry);
                    }
                }
            }
        }
        return None;
    });

    if let Some(file) = files.choose(&mut rng) {
        let maybe_buffer = load_image(file.path(), query.into_inner()).unwrap();

        return HttpResponse::Ok()
            .insert_header(ContentType::png())
            .body(maybe_buffer);
    }
    return HttpResponse::NotFound().body("Not found");
}
