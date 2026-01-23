import type { TeamRole, User } from "../../../types";
import { HttpClient } from "../../client";
import { APIVersion } from "../../types";

const client = HttpClient.create({ version: APIVersion.V1});

type LoginBasicRequest = {
  email: string;
  password: string;
};

type TokenResponse = {
  token: string;
};
type LoginBasicResponse = TokenResponse;

const userService = {
  loginBasic(payload: LoginBasicRequest) {
    return client.post<LoginBasicRequest, LoginBasicResponse>("/user/login/basic", payload);
  },
};

export type {
  LoginBasicRequest,
  LoginBasicResponse,
};

export default userService;
