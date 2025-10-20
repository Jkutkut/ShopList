import useForm from "../../hooks/useForm";
import { TextField, TextFieldType } from "./textField";

interface Props {
    team: any;
};

const ConfigTeamForm = ({
    team,
}: Props) => {
    const { name, description, img, onChange } = useForm({
        name: team.name,
        description: team.description,
        img: "",
    });
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
                name="description"
                label="Description"
                type={TextFieldType.TEXT}
                initialValue={description}
                onChange={onChange}
            />
            {/* TODO img */}
            <button className="btn btn-primary" type="submit">Submit</button>
            <button className="btn btn-danger" type="submit">Delete team</button>
        </form>
    </section>;
};

export default ConfigTeamForm;
