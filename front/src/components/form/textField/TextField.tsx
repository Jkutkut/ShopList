import "../../../styles/form.css";
import { useEffect, useState } from "react";
import { DEFAULT_VALIDATION_TIMEOUT, FormValidationState, INFINITE_VALIDATION_TIMEOUT, TextFieldType } from ".";
import ValidationFeedback from "../ValidationFeedback";

interface Props {
    name: string;
    label?: string;
    placeholder?: string;
    type?: TextFieldType;
    autocomplete?: string;
    initialValue?: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onInputFocus?: () => void;
    onInputBlur?: () => void;
    validate?: (value: string) => FormValidationState;
    okMessage?: string;
    errorMessage?: string;
    okMessageTimeout?: number;
    errorMessageTimeout?: number;
    className?: string;
}

const TextField = ({
    name,
    label,
    placeholder = "",
    type = TextFieldType.TEXT,
    autocomplete,
    initialValue = "",
    onChange,
    onInputFocus,
    onInputBlur,
    validate,
    okMessage,
    errorMessage,
    okMessageTimeout = DEFAULT_VALIDATION_TIMEOUT,
    errorMessageTimeout = INFINITE_VALIDATION_TIMEOUT,
    className = "",
}: Props) => {
    const [validationState, setValidationState] = useState<FormValidationState>(FormValidationState.NONE);
    const [value, setValue] = useState(initialValue);

    const onChangeListener = (e: React.ChangeEvent<HTMLInputElement>) => {
        setValue(e.target.value);
        if (validate) {
            const result = validate(e.target.value);
            setValidationState(result);
            if (result !== FormValidationState.SUCCESS) {
                return;
            }
        }
        onChange(e);
    }
    className = `input-field ${className}`.trim();
    return <div className={className}>
        {label && <span className="input-field-text">{label}</span>}
        <input
            type={type}
            name={name}
            autoComplete={autocomplete}
            placeholder={placeholder}
            value={value}
            onChange={onChangeListener}
            onFocus={onInputFocus}
            onBlur={onInputBlur}
        />
        <ValidationFeedback
            isOn={validationState !== FormValidationState.NONE}
            type={validationState === FormValidationState.SUCCESS ? "valid" : "invalid"}
            message={validationState === FormValidationState.SUCCESS ? okMessage : errorMessage}
            time={validationState === FormValidationState.SUCCESS ? okMessageTimeout : errorMessageTimeout}
        />
    </div>;
};

export default TextField;
