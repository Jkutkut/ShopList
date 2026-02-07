import { ROLES } from "../../mockup";

interface Props {
    name: string;
    onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
    value?: string;
    disabled?: boolean;
};

const RoleSelector = ({
    name,
    onChange,
    value,
    disabled = false
}: Props) => {
    return <select
        name={`role-selector-${name}`}
        value={value || ROLES[0]}
        onChange={onChange}
        disabled={disabled}
    >
        {ROLES.map((role: string) => (
            <option
                key={role}
                value={role}
            >
                {role}
            </option>
        ))}
    </select>
};

export default RoleSelector;
