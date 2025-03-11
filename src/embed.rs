// use crate::tera_lib::versioning::version;
use actix_web::{HttpRequest, HttpResponse};
use rust_embed::RustEmbed;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "templates"]
#[include = "*.html"]
struct TemplateFiles;

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticFiles;

// load templates
pub fn load_templates() -> tera::Result<Tera> {
    let mut tera = Tera::default();
    // tera.register_function("version", version);

    // Iterate over the embedded templates
    for entry in TemplateFiles::iter() {
        let template_name = entry.as_ref();
        let template_content = TemplateFiles::get(template_name).unwrap(); // This should not return None if the template exists
        let template_content = String::from_utf8(template_content.data.to_vec()).unwrap();

        // Add the loaded template content to Tera
        tera.add_raw_template(template_name, &template_content)?;
    }

    Ok(tera)
}

// Define a custom handler to serve embedded static files
pub async fn serve_static_file(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    // Extract the path requested by the client
    let path = req.match_info().query("filename").to_string();

    // Use the RustEmbed trait to access and serve the embedded files
    if let Some(content) = StaticFiles::get(&path) {
        // Create an HttpResponse with the content from the embedded file
        let content_type = match path.split('.').last().unwrap() {
            "css" => "text/css",
            "js" => "text/javascript",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "html" => "text/html",
            _ => "text/plain",
        };
        let data = content.data;
        Ok(HttpResponse::Ok()
            // .append_header((
            //     "Cache-Control",
            //     if env!("VERGEN_CARGO_DEBUG") == "true" {
            //         "no-cache"
            //     } else {
            //         "private, max-age=2592000, s-maxage=2592000, immutable"
            //     },
            // ))
            .append_header(("Service-Worker-Allowed", "/"))
            .content_type(content_type) // Set the appropriate content type
            .body(data))
    } else {
        // Return a 404 response if the file was not found
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}
