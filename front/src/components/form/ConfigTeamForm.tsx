import { useState, type FormEvent } from "react";
import useForm from "../../hooks/useForm";
import type { Team } from "../../types";
import { TextField, TextFieldType } from "./textField";
import teamService from "../../api/versions/v1/teamService";
import ValidationFeedback from "./ValidationFeedback";

interface Props {
    team: Team;
};

const ConfigTeamForm = ({
    team,
}: Props) => {
    const { name, displayName, description, img, onChange } = useForm({
        name: team.name,
        displayName: team.display_name,
        description: team.description,
        img: team.image,
    });
    const [feedback, setFeedback] = useState<string>("");

    const onDelete = async (e: FormEvent) => {
        e.preventDefault();
        const r = await teamService.deleteTeam(team.id);
        if (r.isErr()) {
            console.error("Unable to delete team.", r.unwrapErr());
            setFeedback("Unable to delete team: " + r.unwrapErr().detail.message);
            return;
        }
        setFeedback("Team deleted successfully.");
        window.location.href = "/teams";
    };
    return <section className="full-screen-form">
        <form className="col gap">
            <h1>Configure team</h1>
            <TextField
                name="name"
                label="Name"
                type={TextFieldType.TEXT}
                initialValue={name}
                onChange={onChange}
            />
            <TextField
                name="display_name"
                label="Display name"
                type={TextFieldType.TEXT}
                initialValue={displayName}
                onChange={onChange}
            />
            <TextField
                name="description"
                label="Description"
                type={TextFieldType.TEXT}
                initialValue={description}
                onChange={onChange}
            />
            {/* TODO img */}
            <button
                className="btn btn-primary"
                type="submit"
                disabled
            >
                Submit
            </button>
            <button
                className="btn btn-danger"
                type="submit"
                onClick={onDelete}
            >
                Delete team
            </button>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>
    </section>;
};

export default ConfigTeamForm;
