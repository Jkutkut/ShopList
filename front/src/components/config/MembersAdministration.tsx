import { useState } from "react";
import type { Team, User, UserRole, Uuid } from "../../types";
import AddMember from "./AddMember";
import UserTeamRole from "./UserTeamRole";
import userService from "../../api/versions/v1/userService";

interface Props {
    team: Team;
    teamRoles: UserRole[];
    user: User;
}

interface UpdateRoleProps {
    userId: Uuid;
    role: string;
    isDeletion?: boolean;
}

const MembersAdministration = ({
    team,
    teamRoles,
    user
}: Props) => {
    const [ roles, setRoles ] = useState<UserRole[]>(teamRoles);
    const iAmAdmin = roles.some((teamRole: any) => teamRole.user.id === user.id && teamRole.role === 'admin');
    const admins = roles.filter((teamRole: any) => teamRole.role === 'admin');
    const members = roles.filter((teamRole: any) => teamRole.role === 'member');

    const updateRole = async ({
        userId,
        role: newRole,
        isDeletion = false
    }: UpdateRoleProps) => {
        console.info("Update role of user with id", userId, "to", newRole, "isDeletion:", isDeletion);
        let found = false;
        const updatedRoles = roles.map((teamRole: any) => {
            if (teamRole.user.id === userId) {
                found = true;
                if (isDeletion) {
                    return null;
                }
                return {
                    ...teamRole,
                    role: newRole
                };
            }
            return teamRole;
        }).filter(Boolean) as UserRole[];
        if (!found && !isDeletion) {
            // const user = (await userService.userInfo(userId)).unwrap().data; // Already validated
            // updatedRoles.push({
            //     user,
            //     role: newRole
            // });
            // TODO load user
            window.location.href = window.location.href;
        }
        setRoles(updatedRoles);
    };
    const onChangeRole = (userId: Uuid, newRole: string) => {
        console.info("Change role of user with id", userId, "to", newRole);
        updateRole({
            userId,
            role: newRole,
            isDeletion: false
        });
    };
    const onRemove = (userId: Uuid) => {
        console.info("Remove user with id", userId);
        updateRole({
            userId,
            role: "member",
            isDeletion: true
        });
    };

    return <>
        <h2>Admins</h2>
        <div className="col gap">
            {admins.map((teamRole: any, idx) => (
                <UserTeamRole
                    key={idx}
                    team={team}
                    currentUserId={user.id}
                    teamUserRole={teamRole}
                    iAmAdmin={iAmAdmin && teamRole.user.id !== user.id}
                    onChangeRole={onChangeRole}
                    onRemove={onRemove}
                />
            ))}
        </div>
        <h2>Members</h2>
        <div className="col gap">
            {members.map((teamRole: any, idx) => (
                <UserTeamRole
                    key={idx}
                    team={team}
                    currentUserId={user.id}
                    teamUserRole={teamRole}
                    iAmAdmin={iAmAdmin}
                    onChangeRole={onChangeRole}
                    onRemove={onRemove}
                />
            ))}
        </div>
        {iAmAdmin &&
            <AddMember
                team={team}
                teamRoles={roles}
                onSubmitted={updateRole}
            />
        }
    </>;
};

export default MembersAdministration;
export type { UpdateRoleProps };
