-- Your SQL goes here
ALTER TABLE role_permissions
DROP CONSTRAINT fk_role;


ALTER TABLE role_permissions
ADD CONSTRAINT fk_role
FOREIGN KEY (role_id) REFERENCES roles(id)
ON DELETE CASCADE;