import { useState, type FormEvent } from "react";
import useForm from "../../hooks/useForm";
import { FormValidationState, TextField, TextFieldType } from "./textField";
import userService from "../../api/versions/v1/userService";
import type { FetchTransportError } from "../../api/transport/fetchTransport";
import type { TransportError } from "../../api/types";
import ValidationFeedback from "./ValidationFeedback";

const LoginForm = () => {
    const { email, password, onChange } = useForm({
        email: "",
        password: ""
    });
    const [ feedback, setFeedback ] = useState<string>("");

    const emailValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        const regex = /^[a-z][a-z1-9._-]*@[a-z]+\.[a-z]{1,3}$/;
        if (!regex.test(value)) {
            return FormValidationState.ERROR;
        }
        return FormValidationState.SUCCESS;
    };
    const passwordValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        return FormValidationState.SUCCESS;
    };
    const isFormValid = [
        emailValidator(email),
        passwordValidator(password)
    ].every((v) => v === FormValidationState.SUCCESS);

    const onInputChange = (e: FormEvent) => {
        if (feedback !== "") {
            setFeedback("");
        }
        onChange(e);
    }
    const onSubmit = async (e: FormEvent) => {
        e.preventDefault();
        if (!isFormValid) {
            return;
        }
        const r = await userService.loginBasic({ email, password });
        if (r.isErr()) {
            const error: TransportError<FetchTransportError> = r.unwrapErr();
            console.error("Log in error.", error.detail.message, error);
            onChange({ target: { name: "password", value: "" }});
            if (error.status == 400) {
                setFeedback(`Unable to log in: email or password is incorrect`);
                return;
            }
            else {
                setFeedback(`Not able to log in: ${error.detail.message}`);
                console.error(error);
            }
            return;
        }
        const tokenResponse = r.unwrap().data;
        localStorage.setItem("token", JSON.stringify(tokenResponse)); // TODO change
        window.location.href = "/";
    }

    return <section className="full-screen-form">
        <form className="col gap">
            <h1>Login</h1>
            <TextField
                name="email"
                label="Email"
                type={TextFieldType.EMAIL}
                autocomplete="email"
                initialValue={email}
                validate={emailValidator}
                errorMessage="Enter a valid email"
                onChange={onInputChange}
            />
            <TextField
                name="password"
                label="Password"
                type={TextFieldType.PASSWORD}
                autocomplete="current-password"
                initialValue={password}
                validate={passwordValidator}
                errorMessage="Enter a password"
                onChange={onInputChange}
            />
            <a
                href="/register"
                className="link"
                aria-label="Go to the register page"
            >
                Don't have an account? Register instead
            </a>
            <button type="submit"
                className="btn btn-primary"
                onClick={onSubmit}
                disabled={!isFormValid}
            >
                Login
            </button>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>
    </section>;
};

export default LoginForm;
