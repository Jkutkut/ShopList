import { useState, type FormEvent } from "react";
import useForm from "../../hooks/useForm";
import { TextField, FormValidationState, TextFieldType, INFINITE_VALIDATION_TIMEOUT } from "./textField";
import ValidationFeedback from "./ValidationFeedback";
import userService from "../../api/versions/v1/userService";
import type { TransportError } from "../../api/types";
import type { FetchTransportError } from "../../api/transport/fetchTransport";

const RegisterForm = () => {
    const { name, email, password, onChange } = useForm({
        name: "",
        email: "",
        password: ""
    });
    const [ feedback, setFeedback ] = useState<string>("");

    const nameValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        return FormValidationState.SUCCESS;
    }
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
            return {
                state: FormValidationState.ERROR,
                message: "Password is required"
            };
        }
        const rules: { test: RegExp, msg: string }[] = [
            // { test: /^.{8,}$/, msg: "at least 8 characters"},
            // { test: /[A-Z].*[A-Z]/, msg: "at least 2 upper case letters"},
            // { test: /\d.*\d/, msg: "at least 2 numbers"},
            // { test: /\W.*\W/, msg: "at least 2 special characters"},
            // { test: /^[A-Za-z0-9]+$/, msg: "Only english alphabet and numbers"},
            // { test: /^(?:([A-Za-z0-9])(?!.*\1))*$/, msg: "No repetitions of characters"},
        ];
        const errors = rules
            .filter((rule) => !rule.test.test(value))
            .map((rule) => rule.msg)
            .join(", ");
        return {
            state: errors ? FormValidationState.ERROR : FormValidationState.SUCCESS,
            message: errors ? `Invalid password: ${errors}.` : undefined
        };
    };
    const isFormValid = [
        nameValidator(name),
        emailValidator(email),
        passwordValidator(password)
    ].every((v) =>
        v === FormValidationState.SUCCESS ||
        (typeof v === "object" && v.state === FormValidationState.SUCCESS)
    );

    const onSubmit = async (e: FormEvent) => {
        e.preventDefault();
        if (!isFormValid) {
            return;
        }
        const r = await userService.registerBasic({ name, email, password });
        if (r.isErr()) {
            const error: TransportError<FetchTransportError> = r.unwrapErr();
            console.error("Register error.", error.detail.message, error);
            setFeedback(`Not able to register: ${error.detail.message}`);
            console.error(error);
            return;
        }
        const tokenResponse = r.unwrap().data;
        localStorage.setItem("token", JSON.stringify(tokenResponse)); // TODO change
        window.location.href = "/";
    }

    return <section className="full-screen-form">
        <form className="col gap">
            <h1>Register</h1>
            <TextField
                name="name"
                label="Username"
                type={TextFieldType.TEXT}
                autocomplete="nickname username"
                initialValue={name}
                onChange={onChange}
                validate={nameValidator}
                errorMessage="Invalid username"
            />
            <TextField
                name="email"
                label="Email"
                type={TextFieldType.EMAIL}
                autocomplete="email"
                initialValue={email}
                onChange={onChange}
                validate={emailValidator}
                errorMessageTimeout={INFINITE_VALIDATION_TIMEOUT}
                errorMessage="Invalid email"
            />
            <TextField
                name="password"
                label="Password"
                type={TextFieldType.PASSWORD}
                autocomplete="new-password"
                initialValue={password}
                onChange={onChange}
                validate={passwordValidator}
                okMessage="Good password!"
            />
            <a
                href="/login"
                className="link"
                aria-label="Go to the login page"
            >
                Already have an account? Login instead
            </a>
            <button
                className="btn btn-primary"
                type="submit"
                onClick={onSubmit}
                disabled={!isFormValid}
            >
                Register
            </button>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>
    </section>;
};

export default RegisterForm;
