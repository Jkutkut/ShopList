type Uuid = string;

type User = {
  id: Uuid;
  name: string;
  created_at: string;
  updated_at: string;
  image: string;
};

type Role = string; // TODO

type Team = {
  id: Uuid;
  name: string;
  display_name?: string;
  description?: string;
  image?: string;
  created_by: string;
  updated_by: string;
  created_at: string;
  updated_at: string;
};

type TeamRole = {
  team: Team,
  role: Role
};

type UserRole = {
  user: User,
  role: Role
};

export type {
  Uuid,
  User,
  Role,
  Team,
  TeamRole,
  UserRole
};
