import { useState, type FormEvent } from "react";
import useForm from "../../hooks/useForm";
import { FormValidationState, TextField, TextFieldType } from "./textField";
import ValidationFeedback from "./ValidationFeedback";
import teamService from "../../api/versions/v1/teamService";

const NewTeamForm = () => {
    const { name, description, img, onChange } = useForm({
        name: "",
        description: "",
        img: "",
    });
    const [ feedback, setFeedback ] = useState<string>("");

    const nameValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        return FormValidationState.SUCCESS;
    };
    const descriptionValidator = (_: string) => {
        // TODO
        return FormValidationState.SUCCESS;
    };
    const isFormValid = [
        nameValidator(name),
        descriptionValidator(description),
    ].every((v) => v === FormValidationState.SUCCESS);

    const onInputChange = (e: FormEvent) => {
        if (feedback !== "") {
            setFeedback("");
        }
        onChange(e);
    };
    const onSubmit = async (e: FormEvent) => {
        e.preventDefault();
        if (!isFormValid) {
            return;
        }
        const payload = {
            name,
            description,
            // TODO img
        };
        const r = await teamService.createTeam(payload);
        if (r.isErr()) {
            const error = r.unwrapErr();
            console.error("Create team error.", error);
            setFeedback(`Unable to create team: ${error.detail.message}`);
            return;
        }
        const team_id = r.unwrap();
        window.location.href = `/${team_id}`;
    };

    return <section className="full-screen-form">
        <form className="col gap">
            <h1>New Team</h1>
            <TextField
                name="name"
                label="Name"
                type={TextFieldType.TEXT}
                initialValue={name}
                onChange={onInputChange}
            />
            <TextField
                name="description"
                label="Description"
                type={TextFieldType.TEXT}
                initialValue={description}
                onChange={onInputChange}
            />
            {/* TODO img */}
            <button
                className="btn btn-primary"
                type="submit"
                onClick={onSubmit}
                disabled={!isFormValid}
            >
                Submit
            </button>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>
    </section>;
};

export default NewTeamForm;
