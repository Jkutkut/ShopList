import TextField from "./TextField";

export { TextField };

enum TextFieldType {
    TEXT = 'text',
    EMAIL = 'email',
    PASSWORD = 'password'
}

enum FormValidationState {
    NONE = 'none',
    ERROR = 'error',
    SUCCESS = 'success'
}

const DEFAULT_VALIDATION_TIMEOUT = 3000;
const INFINITE_VALIDATION_TIMEOUT = -1;

export { TextFieldType, FormValidationState };
export { DEFAULT_VALIDATION_TIMEOUT, INFINITE_VALIDATION_TIMEOUT };
