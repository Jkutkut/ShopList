import { useEffect, useState } from "react";
import { INFINITE_VALIDATION_TIMEOUT } from "./textField";

interface ValidationFeedbackProps {
    isOn: boolean;
    type: "valid" | "invalid";
    message?: string;
    time?: number;
}

const ValidationFeedback = ({
    isOn,
    type,
    message,
    time = INFINITE_VALIDATION_TIMEOUT
}: ValidationFeedbackProps) => {
    const [isVisible, setIsVisible] = useState<boolean>(isOn && message !== undefined);

    useEffect(() => {
        const shouldShow = isOn && message !== undefined;
        setIsVisible(shouldShow);
        if (time === INFINITE_VALIDATION_TIMEOUT || !shouldShow) {
            return;
        }
        let timeout = setTimeout(() => setIsVisible(false), time);
        return () => clearTimeout(timeout);
    }, [isOn]);

    return isVisible && message && <div className={`${type}-feedback`}>
        {message}
    </div>;
};

export default ValidationFeedback;
