import type { Team, User, UserRole, Uuid } from "../../types";
import UserTeamRole from "./UserTeamRole";

interface Props {
    team: Team;
    teamRoles: UserRole[];
    user: User;
}

const MembersAdministration = ({
    team,
    teamRoles,
    user
}: Props) => {
    const iAmAdmin = teamRoles.some((teamRole: any) => teamRole.user.id === user.id && teamRole.role === 'admin');
    const admins = teamRoles.filter((teamRole: any) => teamRole.role === 'admin');
    const members = teamRoles.filter((teamRole: any) => teamRole.role === 'member');

    console.log("user", user);
    console.log("team", team);
    console.log("teamRoles", teamRoles);

    console.log("iAmAdmin", iAmAdmin);
    console.log("admins", admins);
    console.log("members", members);

    const onChangeRole = (userId: Uuid, newRole: string) => {
        console.info("Change role of user with id", userId, "to", newRole);
        console.error("Not implemented yet");
    };
    const onRemove = (userId: Uuid) => {
        console.info("Remove user with id", userId);
        console.error("Not implemented yet");
    };

    return <>
        <h2>Admins</h2>
        <div className="col gap">
            {admins.map((teamRole: any, idx) => (
                <UserTeamRole
                    key={idx}
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
                    currentUserId={user.id}
                    teamUserRole={teamRole}
                    iAmAdmin={iAmAdmin}
                    onChangeRole={onChangeRole}
                    onRemove={onRemove}
                />
            ))}
        </div>
        <button
            className="btn btn-primary"
            disabled={!iAmAdmin}
        >
            Add Member
        </button>
    </>;
};

export default MembersAdministration;
