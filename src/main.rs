fn main() {}

#[cfg(test)]
mod tests {
    use sqlx::{FromRow, PgPool};

    use super::*;

    #[derive(Debug, Clone, FromRow, serde::Deserialize, serde::Serialize)]
    #[sqlx(rename = "javascript_page")]
    pub struct JavascriptPage {
        pub body: String,
        pub url: String,
        pub alert_triggered: bool,
    }

    #[derive(Debug, Clone, FromRow, serde::Deserialize, serde::Serialize)]
    #[sqlx(rename = "endpoint")]
    pub struct Endpoint {
        pub id: i32,
        pub endpoint: String,
        pub headers: String,
        pub response_body: String,
        pub old_response_body: String,
        pub status_code: i16,
        pub response_filters: String,
        pub current_run_id: Option<String>,
        pub javascript_page: Option<JavascriptPage>,
    }

    #[sqlx::test(fixtures("endpoint", "javascript_page"))]
    fn one_to_one_pg(pool: PgPool) {
        let q = sqlx::query!("SELECT * from javascript_page")
            .fetch_all(&pool)
            .await
            .unwrap();

        eprintln!("DEBUGPRINT[1]: main.rs:38: q={:#?}", q);
    }

    #[sqlx::test(fixtures("endpoint", "javascript_page"))]
    fn one_to_many_pg(pool: PgPool) {
        let q = sqlx::query!("SELECT * from endpoint")
            .fetch_all(&pool)
            .await
            .unwrap();

        eprintln!("DEBUGPRINT[1]: main.rs:38: q={:#?}", q);
    }
}
