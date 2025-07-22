-- ____________ Tables ____________
DROP TABLE IF EXISTS basic_login;
DROP TABLE IF EXISTS credentials;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name text NOT NULL,
  image text,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp DEFAULT now() NOT NULL
);

CREATE TABLE credentials (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id uuid NOT NULL,
  token text NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  expires_at timestamp NOT NULL,

  CONSTRAINT credentials_token_unique UNIQUE (token),
  CONSTRAINT valid_dates CHECK (expires_at > created_at),

  -- Remove credentials when expires
  CHECK (expires_at > now()),

  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE basic_login (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id uuid NOT NULL,
  email text NOT NULL,
  password text NOT NULL,

  CONSTRAINT basic_login_email_unique UNIQUE (email),
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- ____________ Functions ____________
DROP FUNCTION IF EXISTS create_user_basic_credentials(users.name%TYPE, basic_login.email%TYPE, basic_login.password%TYPE);

CREATE FUNCTION create_user_basic_credentials(
    username users.name%TYPE,
    email basic_login.email%TYPE,
    password basic_login.password%TYPE
) RETURNS users.id%TYPE
AS $$
DECLARE
    user_id users.id%TYPE;
BEGIN
    INSERT INTO users (name) VALUES (username) RETURNING id INTO user_id;
    INSERT INTO basic_login (user_id, email, password) VALUES (user_id, email, password);
    RETURN user_id;
END $$ LANGUAGE plpgsql;
