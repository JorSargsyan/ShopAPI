CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL
);

INSERT INTO roles (title)
VALUES
('SuperAdmin'),
('User');

CREATE TABLE credentials (
    id SERIAL PRIMARY KEY,
    password TEXT NOT NULL
);


CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL,
    role_id INT NOT NULL,
    credential_id INT NOT NULL,
    created_at TEXT NOT NULL, 
    CONSTRAINT fk_role FOREIGN KEY(role_id) REFERENCES roles(id),
    CONSTRAINT fk_credential FOREIGN KEY(credential_id) REFERENCES credentials(id)
);


CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE role_permissions (
    id  SERIAL PRIMARY KEY,
    role_id INT,
    permission_id INT,
    CONSTRAINT fk_role FOREIGN KEY(role_id) REFERENCES roles(id),
    CONSTRAINT fk_permission FOREIGN KEY(permission_id) REFERENCES permissions(id)
)