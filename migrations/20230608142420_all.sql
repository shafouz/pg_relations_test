CREATE TABLE IF NOT EXISTS endpoint
(
    id SERIAL PRIMARY KEY,
    endpoint TEXT NOT NULL UNIQUE,
    response_body TEXT NOT NULL DEFAULT '',
    old_response_body TEXT NOT NULL DEFAULT '',
    status_code SMALLINT NOT NULL DEFAULT 0,
    response_filters TEXT NOT NULL DEFAULT '',
    headers TEXT NOT NULL DEFAULT '',
    callback TEXT NOT NULL DEFAULT '',
    schedule TEXT CHECK(schedule IN ('hourly', '8 hours', 'daily', 'weekly', '3 days')) NOT NULL DEFAULT '8 hours',
    current_run_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS javascript_page
(
    id SERIAL PRIMARY KEY NOT NULL,
    run_id TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'new',
    alert_triggered BOOLEAN NOT NULL DEFAULT FALSE,
    body TEXT NOT NULL,

    endpoint_id INT NOT NULL,
    CONSTRAINT fk_endpoint FOREIGN KEY(endpoint_id) REFERENCES endpoint(id)
);

