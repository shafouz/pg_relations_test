fn main() {}

#[cfg(test)]
mod tests {
    use sqlx::{postgres::PgRow, FromRow, PgPool, Row, Type};

    #[derive(Debug, Clone, sqlx::Decode, FromRow, serde::Deserialize, serde::Serialize)]
    #[sqlx(type_name = "javascript_page")]
    pub struct JavascriptPage {
        pub id: Option<i32>,
        pub endpoint: Option<String>,
    }

    #[derive(Debug, Clone, FromRow, serde::Deserialize, serde::Serialize)]
    #[sqlx(type_name = "endpoint")]
    pub struct Endpoint {
        pub id: Option<i32>,
        pub endpoint: Option<String>,
        pub headers: Option<String>,
        pub response_body: Option<String>,
        pub old_response_body: Option<String>,
        pub status_code: Option<i16>,
        pub response_filters: Option<String>,
        pub current_run_id: Option<String>,
        pub javascript_page: Option<Vec<JavascriptPage>>,
    }

    #[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
    #[sqlx(type_name = "consumable")]
    pub struct Consumable {
        pub id: String,
        pub name: Option<String>,
        pub servings: Option<Serving>,
    }

    #[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
    #[sqlx(type_name = "serving")]
    pub struct Serving {
        pub id: String,
        pub amount: i32,
        pub kcal: i32,
    }

    #[derive(Clone, Debug)]
    pub struct MediaFile {
        pub id: i32,
        pub name: String,
        pub media_language: Lang,
        pub media_speaker: Vec<Speaker>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Lang {
        pub id: i32,
        pub name: String,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Speaker {
        pub id: i32,
        pub name: String,
    }

    impl FromRow<'_, PgRow> for MediaFile {
        fn from_row(row: &PgRow) -> sqlx::Result<Self> {
            let lang = Lang {
                id: row.get::<(i32, String), &str>("language").0,
                name: row.get::<(i32, String), &str>("language").1,
            };

            let mut speaker = vec![];

            for s in row
                .get::<Vec<Option<(i32, String)>>, &str>("speaker")
                .into_iter()
                .flatten()
            {
                speaker.push(Speaker { id: s.0, name: s.1 })
            }

            Ok(Self {
                id: row.get("id"),
                name: row.get("name"),
                media_language: lang,
                media_speaker: speaker,
            })
        }
    }

    #[sqlx::test(fixtures("endpoint", "javascript_page"))]
    fn one_to_one_pg(_pool: PgPool) {
        // let q = sqlx::query!("SELECT * from javascript_page")
        //     .fetch_all(&pool)
        //     .await
        //     .unwrap();

        // eprintln!("DEBUGPRINT[1]: main.rs:38: q={:#?}", q);
    }

    #[sqlx::test(fixtures("endpoint", "javascript_page"))]
    fn one_to_many_pg(pool: PgPool) {
        let q = sqlx::query_as!(
            Endpoint,
            r#"SELECT e.id AS "id: i32",
            e.endpoint AS "endpoint: String",
            e.headers AS "headers: String",
            e.response_body AS "response_body: String",
            e.old_response_body AS "old_response_body: String",
            e.status_code AS "status_code: i16",
            e.response_filters AS "response_filters: String",
            e.current_run_id AS "current_run_id: String",
            (
                jp.id,
                jp.endpoint
            ) AS "javascript_page: Vec<JavascriptPage>"
            FROM endpoint e
            LEFT JOIN javascript_page jp ON e.id = jp.endpoint_id"#
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        eprintln!("DEBUGPRINT[1]: main.rs:122: q={:#?}", q);
    }

    #[sqlx::test]
    fn working_as_row(pool: PgPool) {
        let ordering = "id ASC";
        let query = format!(
            r#"SELECT mf.*,
            (ml.id, ml."name") AS "language",
            array_agg((case when ms.id > 0 then (ms.id, ms."name") end)) AS "speaker"
            FROM media_files mf
            INNER JOIN media_languages ml ON ml.id = mf.lang_id
            LEFT JOIN media_file_speakers mfs ON mfs.media_file_id = mf.id
            LEFT JOIN media_speakers ms ON ms.id = mfs.media_speaker_id
            GROUP BY mf.id, ml.id
            ORDER BY {}"#,
            ordering
        );

        let data: Vec<MediaFile> = sqlx::query_as(&query).fetch_all(&pool).await.unwrap();
    }
}
