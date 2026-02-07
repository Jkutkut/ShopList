import UserTeamRole from "./UserTeamRole";

interface Props {
    team: any;
    teamRoles: any[];
    user: any;
}

const MembersAdministration = ({
    team,
    teamRoles,
    user
}: Props) => {
    const iAmAdmin = teamRoles.some((teamRole: any) => teamRole.user.id === user.id && teamRole.role === 'admin');
    const admins = teamRoles.filter((teamRole: any) => teamRole.role === 'admin');
    const members = teamRoles.filter((teamRole: any) => teamRole.role === 'member');
    return <>
        <h2>Admins</h2>
        <div className="col gap">
        {admins.map((teamRole: any) => (
            <UserTeamRole
                teamUserRole={teamRole}
                iAmAdmin={iAmAdmin && teamRole.user.id !== user.id}
            />
        ))}
        </div>
        <h2>Members</h2>
        <div className="col gap">
        {members.map((teamRole: any) => (
            <UserTeamRole
                teamUserRole={teamRole}
                iAmAdmin={iAmAdmin}
            />
        ))}
        </div>
    </>;
};

export default MembersAdministration;
