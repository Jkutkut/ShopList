import type { TeamRole, User } from "../../../types";
import { HttpClient } from "../../client";
import { APIVersion } from "../../types";

const client = HttpClient.create({ version: APIVersion.V1});

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
  user_id: string;
  new_password: string;
};

type TokenResponse = {
  token: string;
};
type NothingResponse = {};
type RegisterBasicResponse = TokenResponse;
type LoginBasicResponse = TokenResponse;
type UserResponse = User;
type TeamRolesResponse = TeamRole[];

const userService = {
  userInfo(user_id: string) {
    return client.get<UserResponse>(`/user/${user_id}`);
  },
  deleteUser(user_id: string) {
    return client.delete<NothingResponse>(`/user/${user_id}`);
  },
  registerBasic(payload: RegisterBasicRequest) {
    return client.post<RegisterBasicRequest, RegisterBasicResponse>("/user/register/basic", payload);
  },
  loginBasic(payload: LoginBasicRequest) {
    return client.post<LoginBasicRequest, LoginBasicResponse>("/user/login/basic", payload);
  },
  changeBasicPassword(payload: ChangeBasicPasswordRequest) {
    return client.post<ChangeBasicPasswordRequest, NothingResponse>("/user/basic/password", payload);
  },
  me() {
    return client.get<UserResponse>("/user/me");
  },
  refreshToken() {
    return client.post<null, TokenResponse>("/user/me/token");
  },
  logout() {
    return client.post<null, NothingResponse>("/user/logout");
  },
  logoutUser(userId: string) {
    return client.post<null, NothingResponse>(`/user/logout/${userId}`);
  },
  logoutEveryone() {
    return client.post<null, NothingResponse>("/user/logout/everyone");
  },
  setAsSuperuser(userId: string) {
    return client.post<null, NothingResponse>(`/user/superuser/${userId}`);
  },
  removeAsSuperuser(userId: string) {
    return client.delete<NothingResponse>(`/user/superuser/${userId}`);
  },
  teamRoles() {
    return client.get<TeamRolesResponse>("/team/roles");
  }
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
};

export default userService;
export { client };
