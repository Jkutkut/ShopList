-- ____________ Tables ____________
DROP TABLE IF EXISTS basic_login;
DROP TABLE IF EXISTS credentials;
DROP TABLE IF EXISTS superusers;
DROP TABLE IF EXISTS user_roles;
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

CREATE TABLE user_roles (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id uuid NOT NULL,
  role TEXT NOT NULL,
  team_id uuid NOT NULL,

  CONSTRAINT user_roles_unique UNIQUE (user_id, team_id),
  CONSTRAINT valid_role CHECK (role IN ('admin', 'member')),

  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
  FOREIGN KEY (team_id) REFERENCES teams (id) ON DELETE CASCADE
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
  category_id uuid,
  product_id uuid NOT NULL,
  index integer, -- TODO
  amount float,
  unit text,
  created_at timestamp DEFAULT now() NOT NULL,
  created_by uuid,
  updated_at timestamp DEFAULT now() NOT NULL,
  updated_by uuid,

  CONSTRAINT list_products_unique UNIQUE (list_id, product_id),
  CONSTRAINT list_products_index_positive CHECK (index >= 0),
  CONSTRAINT list_products_amount_positive CHECK (amount >= 0),
  FOREIGN KEY (list_id) REFERENCES lists (id) ON DELETE CASCADE,
  FOREIGN KEY (category_id) REFERENCES list_categories (id) ON DELETE SET NULL,
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
    usr_id credentials.user_id%TYPE,
    tkn credentials.token%TYPE,
    expires_at credentials.expires_at%TYPE
) RETURNS credentials.id%TYPE
AS $$
DECLARE
    credential_id credentials.id%TYPE;
BEGIN
    SELECT id INTO credential_id
    FROM credentials
    WHERE usr_id = user_id AND tkn = token
    LIMIT 1;
    IF FOUND THEN
        RETURN credential_id;
    END IF;
    INSERT INTO credentials (user_id, token, expires_at) VALUES (usr_id, tkn, expires_at) RETURNING id INTO credential_id;
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

DROP FUNCTION IF EXISTS new_team(users.id%TYPE, teams.name%TYPE, teams.description%TYPE, teams.image%TYPE);

CREATE FUNCTION new_team(
    creator users.id%TYPE,
    name teams.name%TYPE,
    description teams.description%TYPE,
    image teams.image%TYPE
) RETURNS teams.id%TYPE
AS $$
DECLARE
    team_id teams.id%TYPE;
BEGIN
    INSERT INTO teams (name, description, image, created_by, updated_by)
        VALUES (name, description, image, creator, creator)
        RETURNING id INTO team_id;
    RETURN team_id;
END $$ LANGUAGE plpgsql;

DROP FUNCTION IF EXISTS team_roles(users.id%TYPE);

CREATE FUNCTION team_roles(
    usr_id users.id%TYPE
) RETURNS TABLE (
    role_name user_roles.role%TYPE,
    team_id teams.id%TYPE,
    team_name teams.name%TYPE,
    team_description teams.description%TYPE,
    team_image teams.image%TYPE,
    team_created_at teams.created_at%TYPE,
    team_created_by teams.created_by%TYPE,
    team_updated_at teams.updated_at%TYPE,
    team_updated_by teams.updated_by%TYPE
)
AS $$
DECLARE
    user_exists BOOLEAN;
BEGIN
    SELECT EXISTS(SELECT 1 FROM users WHERE id = usr_id) INTO user_exists;
    IF NOT user_exists THEN
        RAISE EXCEPTION 'User does not exist';
    END IF;
    RETURN QUERY
    SELECT
        ur.role,
        t.id, t.name,
        t.description, t.image,
        t.created_at, t.created_by,
        t.updated_at, t.updated_by
    FROM user_roles ur
        JOIN teams t ON ur.team_id = t.id
    WHERE ur.user_id = usr_id;
END $$ LANGUAGE plpgsql;
