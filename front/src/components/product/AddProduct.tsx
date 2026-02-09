import { useState, type FormEvent } from "react";
import type { ProductRequest } from "../../api/versions/v1/types";
import useExpanded from "../../hooks/useExpanded";
import type { Props as ManagerProps } from "./ProductManager";
import useForm from "../../hooks/useForm";
import { FormValidationState, TextField, TextFieldType } from "../form/textField";
import ValidationFeedback from "../form/ValidationFeedback";

interface Props extends ManagerProps {
    onAdd: (product: ProductRequest) => void
}

const AddProduct = ({
    team,
    products,
    onAdd,
}: Props) => {
    const { isExpanded, toggleIsExpanded } = useExpanded(false);
    const [ feedback, setFeedback ] = useState<string>("");
    const { name, description, img, onChange } = useForm({
        name: "",
        description: "",
        img: "",
    });
    
    if (!isExpanded) {
        return <button
            className="btn btn-primary no-animation"
            onClick={toggleIsExpanded}
        >
            Add a product
        </button>;
    }

    const nameValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        for (const product of products) {
            if (product.name === value) {
                return FormValidationState.ERROR;
            }
        }
        return FormValidationState.SUCCESS;
    };
    const descriptionValidator = (_: string) => {
        return FormValidationState.SUCCESS;
    };
    const isFormValid = [
        nameValidator(name),
        descriptionValidator(description),
    ].every((v) => v === FormValidationState.SUCCESS);

    const onInputChange = (e: FormEvent) => {
        if (feedback !== "") {
            setFeedback("");
        }
        onChange(e);
    };
    const onSubmit = async (e: FormEvent) => {
        e.preventDefault();
        if (!isFormValid) {
            return;
        }
        onAdd({
            name,
            description,
            // TODO img
        });
    };

    return <div className="product-card padding with-border col">
        <div className="row wrap space-between">
            <h2>Add a product</h2>
            <button
                className="btn btn-primary no-animation"
                onClick={toggleIsExpanded}
            >
                Cancel
            </button>
        </div>
        <form className="col gap">
            <TextField
                name="name"
                label="Name"
                type={TextFieldType.TEXT}
                onChange={onInputChange}
                validate={nameValidator}
                errorMessage="The product name already exists"
                okMessage="The product name is valid"
            />
            <TextField
                name="description"
                label="Description"
                type={TextFieldType.TEXT}
                initialValue={description}
                onChange={onInputChange}
            />
            {/* TODO img */}
            <button
                className="btn btn-primary"
                type="submit"
                onClick={onSubmit}
                disabled={!isFormValid}
            >
                Save
            </button>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>
    </div>;
};

export default AddProduct;
