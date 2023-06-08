-- Sample fixtures for the endpoints table
INSERT INTO endpoint (endpoint, response_body, old_response_body, status_code, response_filters, headers, callback, schedule, current_run_id, created_at, updated_at)
VALUES
    ('endpoint1', 'a', 'a', 1, 'a', 'a', 'a', '8 hours', 'asd', NOW(), NOW()),
    ('endpoint2', 'a', 'a', 1, 'a', 'a', 'a', 'hourly', 'asd', NOW(), NOW()),
    ('endpoint3', 'a', 'a', 1, 'a', 'a', 'a', 'daily', 'asd', NOW(), NOW());

