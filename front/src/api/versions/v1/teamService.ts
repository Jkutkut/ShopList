import type { Team, UserRole } from "../../../types";
import type {
  TeamRequest,
  NothingResponse,
  Uuid,
  UserRoleRequest,
} from "./types";
import { HttpClient } from "../../client";
import { APIVersion } from "../../types";

const client = HttpClient.create({ version: APIVersion.V1});

const teamService = {
  createTeam(payload: TeamRequest) {
    return client.post<TeamRequest, Uuid>(`/team`, payload);
  },
  getTeam(team_id: Uuid) {
    return client.get<Team>(`/team/${team_id}`);
  },
  updateTeam(team_id: Uuid, payload: TeamRequest) {
    return client.put<TeamRequest, Team>(`/team/${team_id}`, payload);
  },
  deleteTeam(team_id: Uuid) {
    return client.delete<NothingResponse>(`/team/${team_id}`);
  },
  teamMembers(team_id: Uuid) {
    return client.get<UserRole[]>(`/team/${team_id}/members`);
  },
  updateTeamMember(team_id: Uuid, payload: UserRoleRequest) {
    return client.put<UserRoleRequest, NothingResponse>(`/team/${team_id}/members`, payload);
  },
  deleteTeamMember(team_id: Uuid, user_id: Uuid) {
    return client.delete(`/team/${team_id}/members/${user_id}`);
  },
};

export default teamService;
export { client };
