pub fn build_mongo_uri(
    host: &str,
    port: u16,
    db: &str,
    username: Option<&str>,
    password: Option<&str>
) -> String {
    if let (Some(user), Some(pass)) = (username, password) {
        if !user.is_empty() && !pass.is_empty() {
            return format!(
                "mongodb://{}:{}@{}:{}/{}",
                user, pass, host, port, db
            );
        }
    }
    format!("mongodb://{}:{}/{}", host, port, db)
}