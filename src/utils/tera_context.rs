use actix_session::Session;

pub fn base_context(session: &Session) -> tera::Context {
    let session_json = session.get::<serde_json::Value>("session").unwrap();
    let mut ctx = tera::Context::new();
    if let Some(data) = session_json {
        ctx.insert("session", &data);
    }
    ctx
}
