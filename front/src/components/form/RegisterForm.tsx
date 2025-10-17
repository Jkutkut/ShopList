import useForm from "../../hooks/useForm";
import { TextField, FormValidationState, TextFieldType, INFINITE_VALIDATION_TIMEOUT } from "./textField";

const RegisterForm = () => {
    const { username, email, password, onChange } = useForm({
        username: "",
        email: "",
        password: ""
    });
    return <section className="full-screen-form">
        <form className="col gap">
            <h1>Register</h1>
            <TextField
                name="username"
                label="Username"
                type={TextFieldType.TEXT}
                autocomplete="nickname username"
                initialValue={username}
                onChange={onChange}
                validate={() => FormValidationState.SUCCESS}
                okMessage="Username available"
                errorMessage="Invalid username"
            />
            <TextField
                name="email"
                label="Email"
                type={TextFieldType.EMAIL}
                autocomplete="email"
                initialValue={email}
                onChange={onChange}
                validate={() => FormValidationState.ERROR}
                errorMessageTimeout={INFINITE_VALIDATION_TIMEOUT}
                okMessage="Valid email!"
                errorMessage="Invalid email"
            />
            <TextField
                name="password"
                label="Password"
                type={TextFieldType.PASSWORD}
                autocomplete="new-password"
                initialValue={password}
                onChange={onChange}
                validate={() => FormValidationState.SUCCESS}
                okMessage="Good enough password!"
                errorMessage="Weak password"
            />
            <a
                href="/login"
                className="link"
                aria-label="Go to the login page"
            >
                Already have an account? Login instead
            </a>
            <button className="btn btn-primary" type="submit">Register</button>
        </form>
    </section>;
};

export default RegisterForm;
