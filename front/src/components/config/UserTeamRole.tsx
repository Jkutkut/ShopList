import { useState, type ChangeEvent } from "react";
import type { Team, UserRole, Uuid } from "../../types";
import RoleSelector from "./RoleSelector";
import teamService from "../../api/versions/v1/teamService";

interface Props {
    team: Team;
    currentUserId: Uuid;
    teamUserRole: UserRole;
    iAmAdmin: boolean;
    onRemove: (userId: Uuid) => void;
    onChangeRole: (userId: Uuid, newRole: string) => void;
}

const UserTeamRole = ({
    team,
    currentUserId,
    teamUserRole,
    iAmAdmin,
    onRemove,
    onChangeRole
}: Props) => {
    const [value, setValue] = useState(teamUserRole.role);
    const onChange = async (e: ChangeEvent<HTMLSelectElement>) => {
        const newRole = e.target.value;
        if (newRole === value) {
            return;
        }
        const r = await teamService.updateTeamMember(team.id, {
            user_id: teamUserRole.user.id,
            role: newRole
        });
        if (r.isErr()) {
            console.error("Error updating team member role", r.unwrapErr().detail.message);
            setValue(teamUserRole.role);
            return;
        }
        setValue(newRole);
        onChangeRole(teamUserRole.user.id, newRole);
    };
    const onRemoveClick = async () => {
        const r = await teamService.deleteTeamMember(team.id, teamUserRole.user.id)
        if (r.isErr()) {
            console.error("Error removing team member", r.unwrapErr().detail.message);
            return;
        }
        onRemove(teamUserRole.user.id);
    };

    const canRemove = iAmAdmin && currentUserId !== teamUserRole.user.id;
    const itsMe = currentUserId === teamUserRole.user.id;
    return (
        <div className="member-card row gap space-between padding with-border center">
            <h3>{teamUserRole.user.name}</h3>
            {itsMe && <span>(you)</span>}
            <div className="row gap">
                <RoleSelector
                    name={teamUserRole.user.id}
                    onChange={onChange}
                    value={value}
                    disabled={!iAmAdmin}
                />
                {canRemove && <button className="btn btn-danger" onClick={onRemoveClick}>Remove</button>}
            </div>
        </div>
    );
};

export default UserTeamRole;
