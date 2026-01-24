type User = {
  id: string;
  name: string;
  email: string;
  created_at: string;
  updated_at: string;
};

type Role = string; // TODO

type Team = {
  id: string;
  name: string;
  description: string;
  image: string;
  created_by: string;
  updated_by: string;
  created_at: string;
  updated_at: string;
};

type TeamRole = {
  team: Team,
  role: Role
};

export type {
  User,
  Role,
  Team,
  TeamRole
};
