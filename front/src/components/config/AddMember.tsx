import { useState } from "react";
import type { Team, UserRole, Uuid } from "../../types";
import { FormValidationState, TextField } from "../form/textField";
import useForm from "../../hooks/useForm";
import ValidationFeedback from "../form/ValidationFeedback";
import RoleSelector from "./RoleSelector";
import teamService from "../../api/versions/v1/teamService";
import type { UpdateRoleProps } from "./MembersAdministration";
import { UUID_V4_REGEX } from "../../utils";

interface Props {
    team: Team;
    teamRoles: UserRole[];
    onSubmitted: (data: UpdateRoleProps) => void;
}

const AddMember = ({
    team,
    teamRoles,
    onSubmitted,
}: Props) => {
    const [expanded, setExpanded] = useState(false);
    const [ feedback, setFeedback ] = useState<string>("");
    const { userId, role, onChange } = useForm({
        userId: "",
        role: "member"
    });

    const toggleExpanded = () => setExpanded(!expanded);
    const userIdValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        if (!UUID_V4_REGEX.test(value)) {
            return FormValidationState.ERROR;
        }
        if (teamRoles.some((teamRole: any) => teamRole.user.id === value)) {
            return FormValidationState.ERROR;
        }
        return FormValidationState.SUCCESS;
    };
    const roleValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        if (!["admin", "member"].includes(value)) {
            return FormValidationState.ERROR;
        }
        return FormValidationState.SUCCESS;
    };
    const isFormValid = [
        userIdValidator(userId),
        roleValidator(role),
    ].every((v) => v === FormValidationState.SUCCESS);
    const onSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!isFormValid) {
            return;
        }
        if (feedback !== "") {
            setFeedback("");
        }
        const r = await teamService.updateTeamMember(team.id, {
            user_id: userId,
            role,
        });
        if (r.isErr()) {
            console.error("Error adding member to team", r.unwrapErr());
            setFeedback("Unable to add member to the team: " + r.unwrapErr().detail.message);
            return;
        }
        onSubmitted({
            userId,
            role,
            isDeletion: false
        });
    };

    console.log("AddMember", { team, teamRoles });
    console.log("AddMember form", { userId, role, isFormValid });

    return <>
        {!expanded && (
            <button
                className="btn btn-primary"
                onClick={toggleExpanded}
            >
                Add Member
            </button>
        ) ||
        <form className="col gap">
            <h3>Add Member:</h3>
            <TextField
                name="userId"
                label="User ID"
                autocomplete="off"
                placeholder="Enter the ID of the user"
                validate={userIdValidator}
                okMessage="User ID looks good"
                errorMessage="Enter a valid user ID (UUID v4) that is not already a member of the team. They can share it by copying it from the profile page."
                initialValue={userId}
                onChange={onChange}
            />
            <RoleSelector
                name={"new-member"}
                value={role}
                onChange={(e) => onChange({
                    target: {
                        name: "role",
                        value: e.target.value
                    }
                })}
            />
            <div className="row gap full-w">
                <button
                    className="btn btn-primary full-w"
                    type="submit"
                    disabled={!isFormValid}
                    onClick={onSubmit}
                >
                    Add
                </button>
                <button
                    className="btn btn-primary full-w"
                    onClick={toggleExpanded}
                >
                    Cancel
                </button>
            </div>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>}
    </>;
};

export default AddMember;
