import useForm from "../../hooks/useForm";
import { TextField, FormValidationState, TextFieldType } from "./textField";

const LoginForm = () => {
    const { username, password, onChange } = useForm({
        username: "",
        password: ""
    });
    return <section>
        <form className="col gap">
            <h1>Login</h1>
            <TextField
                name="username"
                label="Username or Email"
                type={TextFieldType.EMAIL}
                autocomplete="username email nickname"
                initialValue={username}
                onChange={onChange}
            />
            <TextField
                name="password"
                label="Password"
                type={TextFieldType.PASSWORD}
                autocomplete="current-password"
                initialValue={password}
                onChange={onChange}
            />
            <a
                href="/register"
                className="link"
                aria-label="Go to the register page"
            >
                Don't have an account? Register instead
            </a>
            <button className="btn btn-primary" type="submit">Login</button>
        </form>
    </section>;
};

export default LoginForm;
