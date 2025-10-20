import { useState } from "react";
import { ROLES } from "../../mockup";

interface Props {
    teamUserRole: any;
    iAmAdmin: boolean;
}

const UserTeamRole = ({
    teamUserRole,
    iAmAdmin
}: Props) => {
    const [value, setValue] = useState(teamUserRole.role);
    return (
        <div className="member-card row gap space-between padding with-border">
            <h3>{teamUserRole.user.name}</h3>
            <p>{teamUserRole.user.email}</p>
            <select
                value={value}
                onChange={setValue}
                disabled={!iAmAdmin}
            >
                {ROLES.map((role: string) => (
                    <option
                        key={role}
                        value={role}
                    >
                        {role}
                    </option>
                ))}
            </select>
        </div>
    );
};

export default UserTeamRole;
