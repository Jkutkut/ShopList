import "../../../styles/form.css";
import { useEffect, useState } from "react";
import { DEFAULT_VALIDATION_TIMEOUT, FormValidationState, TextFieldType } from ".";

interface Props {
    name: string;
    label?: string;
    placeholder?: string;
    type?: TextFieldType;
    autocomplete?: string;
    initialValue?: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    validate?: (value: string) => FormValidationState;
    okMessage?: string;
    errorMessage?: string;
    okMessageTimeout?: number;
    errorMessageTimeout?: number;
}

const TextField = ({
    name,
    label,
    placeholder = "",
    type = TextFieldType.TEXT,
    autocomplete,
    initialValue = "",
    onChange,
    validate,
    okMessage,
    errorMessage,
    okMessageTimeout = DEFAULT_VALIDATION_TIMEOUT,
    errorMessageTimeout = DEFAULT_VALIDATION_TIMEOUT,
}: Props) => {
    const [validationState, setValidationState] = useState<FormValidationState>(FormValidationState.NONE);

    useEffect(() => {
        if (!validate) return;

        const result = validate(initialValue);
        console.log(result);
        setValidationState(result);

        let timeout: NodeJS.Timeout;
        if (result === FormValidationState.SUCCESS && okMessage) {
            timeout = setTimeout(() => {
                setValidationState(FormValidationState.NONE);
            }, okMessageTimeout);
        } else if (result === FormValidationState.ERROR && errorMessage) {
            timeout = setTimeout(() => {
                setValidationState(FormValidationState.NONE);
            }, errorMessageTimeout);
        }
        
        return () => {
            if (timeout) clearTimeout(timeout);
        };
    }, [initialValue]);

    return <div className="input-field">
        {label && <span className="input-field-text">{label}</span>}
        <input
            type={type}
            name={name}
            autoComplete={autocomplete}
            placeholder={placeholder}
            value={initialValue}
            onChange={onChange}
        />
        {okMessage && validationState === FormValidationState.SUCCESS &&
            <div className="valid-feedback">
                {okMessage}
            </div>
        }
        {errorMessage && validationState === FormValidationState.ERROR &&
            <div className="invalid-feedback">
                {errorMessage}
            </div>
        }
    </div>;
};

export default TextField;
