-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed TINYINT(1) DEFAULT 0, -- 0 for not completed, 1 for completed
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
) CHARSET=utf8mb4;

-- Add your `down` migration SQL here
DROP TABLE IF EXISTS todos;

