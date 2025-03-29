//! Module for handling embedded static files and templates.
//! This module uses the `rust-embed` crate to embed static files and templates into the binary.

use actix_web::{HttpRequest, HttpResponse};
use rust_embed::RustEmbed;
use tera::Tera;

/// Embedded template files from the templates directory
#[derive(RustEmbed)]
#[folder = "templates"]
#[include = "*.html"]
struct TemplateFiles;

/// Embedded static files from the static directory
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticFiles;

/// Loads and initializes the Tera template engine with embedded templates
/// 
/// This function:
/// 1. Creates a new Tera instance
/// 2. Iterates over all embedded template files
/// 3. Loads each template into the Tera engine
/// 
/// # Returns
/// * `Result<Tera, tera::Error>` - The initialized Tera engine or an error if template loading fails
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

/// Serves embedded static files with appropriate content types
/// 
/// This handler:
/// 1. Extracts the requested file path
/// 2. Looks up the file in the embedded static files
/// 3. Determines the appropriate content type based on file extension
/// 4. Returns the file with proper headers or a 404 if not found
/// 
/// # Arguments
/// * `req` - The HTTP request containing the requested file path
/// 
/// # Returns
/// * `Result<HttpResponse, actix_web::Error>` - The response containing the file or an error
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
            .append_header(("Service-Worker-Allowed", "/"))
            .content_type(content_type) // Set the appropriate content type
            .body(data))
    } else {
        // Return a 404 response if the file was not found
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}
