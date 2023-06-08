-- Sample fixtures for the javascript_page table
INSERT INTO javascript_page (endpoint_id, run_id, endpoint, url, status, alert_triggered, body)
VALUES
    (1, 'run001', 'endpoint1', 'https://example.com/page1', 'new', FALSE, 'Sample body 1'),
    (1, 'run002', 'endpoint2', 'https://example.com/page2', 'processing', TRUE, 'Sample body 2'),
    (1, 'run003', 'endpoint3', 'https://example.com/page3', 'completed', FALSE, 'Sample body 3');
