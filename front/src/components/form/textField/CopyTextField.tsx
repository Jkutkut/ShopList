import "../../../styles/form.css";
import { useEffect, useState } from "react";
import { DEFAULT_VALIDATION_TIMEOUT, FormValidationState, TextFieldType } from ".";
import ValidationFeedback from "../ValidationFeedback";

interface Props {
    name: string;
    label?: string;
    readonly?: boolean;
    type?: TextFieldType;
    initialValue: string;
    okMessage?: string;
    okMessageTimeout?: number;
    className?: string;
}

const CopyTextField = ({
    name,
    label,
    readonly = false,
    type = TextFieldType.TEXT,
    initialValue,
    okMessage,
    okMessageTimeout = DEFAULT_VALIDATION_TIMEOUT,
    className = "",
}: Props) => {
    const [validationState, setValidationState] = useState<FormValidationState>(FormValidationState.NONE);
    const [value, setValue] = useState(initialValue);

    const onFocus = () => {
        setValidationState(FormValidationState.SUCCESS);
        navigator.clipboard.writeText(value).then(() => {
            console.debug("Value copied to clipboard");
        }).catch(err => {
            console.error("Unable to copy value to clipboard:", err);
        });
    };

    useEffect(() => {
        setValue(initialValue);
    }, [initialValue]);

    className = `input-field ${className} row gap`.trim();
    return <div className={className}>
        {label && <span className="input-field-text">{label}</span>}
        <div className="col full-w">
            <input
                type={type}
                name={name}
                readOnly={readonly}
                value={value}
                onFocus={onFocus}
                onClick={onFocus}
            />
            <ValidationFeedback
                isOn={validationState !== FormValidationState.NONE}
                type={"valid"}
                message={validationState === FormValidationState.SUCCESS ? okMessage : undefined}
                time={validationState === FormValidationState.SUCCESS ? okMessageTimeout : 0}
            />
        </div>
    </div>;
};

export default CopyTextField;
