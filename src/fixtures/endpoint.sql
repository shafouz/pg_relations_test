-- Sample fixtures for the endpoints table
INSERT INTO endpoint (endpoint, response_body, old_response_body, status_code, response_filters, headers, callback, schedule, current_run_id, created_at, updated_at)
VALUES
    ('endpoint1', '', '', 0, '', '', '', '8 hours', NULL, NOW(), NOW()),
    ('endpoint2', '', '', 0, '', '', '', 'hourly', NULL, NOW(), NOW()),
    ('endpoint3', '', '', 0, '', '', '', 'daily', NULL, NOW(), NOW());

