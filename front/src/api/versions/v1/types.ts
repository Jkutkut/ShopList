import type { TeamRole, User, UserRole, Uuid } from "../../../types";

type RegisterBasicRequest = {
  name: string;
  email: string;
  password: string;
};

type LoginBasicRequest = {
  email: string;
  password: string;
};

type ChangeBasicPasswordRequest = {
  user_id: Uuid;
  new_password: string;
};

type TokenResponse = {
  token: string;
};
type NothingResponse = {};
type RegisterBasicResponse = TokenResponse;
type LoginBasicResponse = TokenResponse;
type UserResponse = User;
type TeamRolesResponse = {
  team_roles: TeamRole[];
};

type TeamRequest = {
  name: string;
  display_name?: string;
  description?: string;
  img?: string;
};

type UserRoleRequest = {
  user_id: Uuid;
  role: string;
};

export type {
  RegisterBasicRequest,
  LoginBasicRequest,
  ChangeBasicPasswordRequest,
  TokenResponse,
  NothingResponse,
  RegisterBasicResponse,
  LoginBasicResponse,
  UserResponse,
  TeamRolesResponse,
  TeamRequest,
  Uuid,
  UserRoleRequest,
};
