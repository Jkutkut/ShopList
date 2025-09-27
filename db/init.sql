-- ____________ Tables ____________
DROP TABLE IF EXISTS basic_login;
DROP TABLE IF EXISTS credentials;
DROP TABLE IF EXISTS superusers;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS product_identifiers;
DROP TABLE IF EXISTS generic_products;
DROP TABLE IF EXISTS product_tags;
DROP TABLE IF EXISTS list_products;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS list_categories;
DROP TABLE IF EXISTS lists;
DROP TABLE IF EXISTS teams;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name text NOT NULL UNIQUE,
  image text,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp DEFAULT now() NOT NULL
);

CREATE TABLE superusers (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id uuid NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
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

CREATE TABLE roles (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name text NOT NULL UNIQUE
);

CREATE TABLE user_roles (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id uuid NOT NULL,
  role_id uuid NOT NULL,
  -- team_id uuid NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
  FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
  -- FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE
);

CREATE TABLE teams (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name text NOT NULL UNIQUE,
  description text,
  image text,
  created_at timestamp DEFAULT now() NOT NULL,
  created_by uuid,
  updated_at timestamp DEFAULT now() NOT NULL,
  updated_by uuid,
  FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE SET NULL,
  FOREIGN KEY (updated_by) REFERENCES users (id) ON DELETE SET NULL
);

CREATE TABLE products (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  team_id uuid NOT NULL,
  name text NOT NULL,
  description text,
  image text,
  created_at timestamp DEFAULT now() NOT NULL,
  created_by uuid,
  updated_at timestamp DEFAULT now() NOT NULL,
  updated_by uuid,

  FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE,
  FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE SET NULL,
  FOREIGN KEY (updated_by) REFERENCES users (id) ON DELETE SET NULL
);

CREATE TABLE product_identifiers (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  product_id uuid NOT NULL,
  identifier text NOT NULL,

  CONSTRAINT product_identifiers_unique UNIQUE (product_id, identifier),
  FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE
);

CREATE TABLE generic_products (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  brand_product_id uuid NOT NULL,
  generic_product_id uuid NOT NULL,

  CONSTRAINT generic_products_unique UNIQUE (brand_product_id, generic_product_id),
  FOREIGN KEY (brand_product_id) REFERENCES products (id) ON DELETE CASCADE,
  FOREIGN KEY (generic_product_id) REFERENCES products (id) ON DELETE CASCADE
);

CREATE TABLE tags (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name text NOT NULL,
  team_id uuid NOT NULL,

  CONSTRAINT tags_unique UNIQUE (name, team_id),
  FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE
);

CREATE TABLE product_tags (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  product_id uuid NOT NULL,
  tag_id uuid NOT NULL,

  CONSTRAINT product_tags_unique UNIQUE (product_id, tag_id),
  FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

CREATE TABLE lists (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  team_id uuid NOT NULL,
  name text NOT NULL,
  description text,
  image text,
  created_at timestamp DEFAULT now() NOT NULL,
  created_by uuid,
  updated_at timestamp DEFAULT now() NOT NULL,
  updated_by uuid,

  CONSTRAINT lists_unique UNIQUE (team_id, name),
  FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE,
  FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE SET NULL,
  FOREIGN KEY (updated_by) REFERENCES users (id) ON DELETE SET NULL
);

CREATE TABLE list_categories (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  list_id uuid NOT NULL,
  name text NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  created_by uuid,
  updated_at timestamp DEFAULT now() NOT NULL,
  updated_by uuid,

  CONSTRAINT list_categories_unique UNIQUE (list_id, name),
  FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE,
  FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE SET NULL,
  FOREIGN KEY (updated_by) REFERENCES users (id) ON DELETE SET NULL
);

CREATE TABLE list_products (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  list_id uuid NOT NULL,
  product_id uuid NOT NULL,
  index integer, -- TODO
  quantity integer,
  unit text,
  created_at timestamp DEFAULT now() NOT NULL,
  created_by uuid,
  updated_at timestamp DEFAULT now() NOT NULL,
  updated_by uuid,

  CONSTRAINT list_products_unique UNIQUE (list_id, product_id),
  CONSTRAINT list_products_index_positive CHECK (index >= 0),
  CONSTRAINT list_products_quantity_positive CHECK (quantity > 0),
  FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE,
  FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE,
  FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE SET NULL,
  FOREIGN KEY (updated_by) REFERENCES users (id) ON DELETE SET NULL
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

DROP FUNCTION IF EXISTS set_superuser(users.id%TYPE);

CREATE FUNCTION set_superuser(user_id users.id%TYPE) RETURNS void
AS $$
BEGIN
    INSERT INTO superusers (user_id) VALUES (user_id);
END $$ LANGUAGE plpgsql;

DROP FUNCTION IF EXISTS create_credentials(credentials.user_id%TYPE, credentials.token%TYPE, credentials.expires_at%TYPE);

CREATE FUNCTION create_credentials(
    user_id credentials.user_id%TYPE,
    token credentials.token%TYPE,
    expires_at credentials.expires_at%TYPE
) RETURNS credentials.id%TYPE
AS $$
DECLARE
    credential_id credentials.id%TYPE;
BEGIN
    INSERT INTO credentials (user_id, token, expires_at) VALUES (user_id, token, expires_at) RETURNING id INTO credential_id;
    RETURN credential_id;
END $$ LANGUAGE plpgsql;

DROP FUNCTION IF EXISTS logout_everyone();

CREATE FUNCTION logout_everyone() RETURNS void
AS $$
BEGIN
    DELETE FROM credentials;
END $$ LANGUAGE plpgsql;

DROP FUNCTION IF EXISTS logout_user_everywhere(credentials.user_id%TYPE);

CREATE FUNCTION logout_user_everywhere(
    usr_id credentials.user_id%TYPE
) RETURNS void
AS $$
BEGIN
    DELETE FROM credentials WHERE credentials.user_id = usr_id;
END $$ LANGUAGE plpgsql;
