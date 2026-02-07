import { useState, type ChangeEvent } from "react";
import type { Uuid } from "../../types";
import RoleSelector from "./RoleSelector";

interface Props {
    currentUserId: Uuid;
    teamUserRole: any;
    iAmAdmin: boolean;
    onRemove: (userId: Uuid) => void;
    onChangeRole: (userId: Uuid, newRole: string) => void;
}

const UserTeamRole = ({
    currentUserId,
    teamUserRole,
    iAmAdmin,
    onRemove,
    onChangeRole
}: Props) => {
    const [value, setValue] = useState(teamUserRole.role);
    const onChange = (e: ChangeEvent<HTMLSelectElement>) => {
        const newRole = e.target.value;
        if (newRole === value) {
            return;
        }
        setValue(newRole);
        onChangeRole(teamUserRole.user.id, newRole);
    };
    const onRemoveClick = () => {
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
