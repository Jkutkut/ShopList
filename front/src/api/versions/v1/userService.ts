import type {
  RegisterBasicRequest,
  LoginBasicRequest,
  ChangeBasicPasswordRequest,
  TokenResponse,
  NothingResponse,
  RegisterBasicResponse,
  LoginBasicResponse,
  UserResponse,
  TeamRolesResponse,
  Uuid,
} from "./types";
import { HttpClient } from "../../client";
import { APIVersion } from "../../types";

const client = HttpClient.create({ version: APIVersion.V1});

const userService = {
  userInfo(user_id: Uuid) {
    return client.get<UserResponse>(`/user/${user_id}`);
  },
  deleteUser(user_id: Uuid) {
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
  logoutUser(userId: Uuid) {
    return client.post<null, NothingResponse>(`/user/logout/${userId}`);
  },
  logoutEveryone() {
    return client.post<null, NothingResponse>("/user/logout/everyone");
  },
  setAsSuperuser(userId: Uuid) {
    return client.post<null, NothingResponse>(`/user/superuser/${userId}`);
  },
  removeAsSuperuser(userId: Uuid) {
    return client.delete<NothingResponse>(`/user/superuser/${userId}`);
  },
  teamRoles() {
    return client.get<TeamRolesResponse>("/team/roles");
  }
};

export default userService;
export { client };
