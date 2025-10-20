import useForm from "../../hooks/useForm";
import { TextField, TextFieldType } from "./textField";

const NewTeamForm = () => {
    const { name, description, img, onChange } = useForm({
        name: "",
        description: "",
        img: "",
    });
    return <section className="full-screen-form">
        <form className="col gap">
            <h1>New Team</h1>
            <TextField
                name="name"
                label="Name"
                type={TextFieldType.EMAIL}
                initialValue={name}
                onChange={onChange}
            />
            <TextField
                name="description"
                label="Description"
                type={TextFieldType.PASSWORD}
                initialValue={description}
                onChange={onChange}
            />
            {/* TODO img */}
            <button className="btn btn-primary" type="submit">Submit</button>
        </form>
    </section>;
};

export default NewTeamForm;
