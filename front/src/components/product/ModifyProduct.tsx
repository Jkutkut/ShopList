import { useState, type FormEvent } from "react";
import type { ProductRequest } from "../../api/versions/v1/types";
import useExpanded from "../../hooks/useExpanded";
import type { Product } from "../../types";
import type { Props as ManagerProps } from "./ProductManager";
import useForm from "../../hooks/useForm";
import { FormValidationState, TextField, TextFieldType } from "../form/textField";
import ValidationFeedback from "../form/ValidationFeedback";
import { IMAGES } from "../../mockup";
import { Created, Updated } from "../date/Metadata";

interface Props extends ManagerProps {
    product: Product;
    onUpdate: (product: Product, productRequest: ProductRequest) => Promise<string | null>;
    onDelete: (product: Product) => Promise<string | null>;
}

const ModifyProduct = ({
    product,
    products,
    onUpdate,
    onDelete
}: Props) => {
    const { isExpanded, setIsExpanded, toggleIsExpanded } = useExpanded(false);
    const [ feedback, setFeedback ] = useState<string>("");
    const { name, description, img, onChange } = useForm({
        name: product.name,
        description: product.description || "",
        img: "",
    });

    if (!isExpanded) {
        return <div className="product-card padding with-border col gap">
            <div className="row space-between wrap gap full-w">
                <h2 className="no-wrap">{product.name}</h2>
                <div>
                    <button
                        className="btn btn-primary no-animation"
                        onClick={toggleIsExpanded}
                    >
                        ✏️
                    </button>
                </div>
            </div>
            <div className="col gap">
                <div className="image-description row gap space-between">
                    {product.description &&
                        <p>{product.description}</p>
                    }
                    {product.image &&
                        <img src={product.image} alt={product.name} /> ||
                        <img src={IMAGES["1:1"]} alt={product.name} />
                    }
                </div>
                <div className="row space-between wrap gap">
                    <Created
                        author={product.created_by}
                        date={product.created_at}
                    />
                    <Updated
                        author={product.updated_by}
                        date={product.updated_at}
                        createdAt={product.created_at}
                    />
                </div>
            </div>
        </div>;
    }

    const nameValidator = (value: string) => {
        if (!value) {
            return FormValidationState.ERROR;
        }
        for (const p of products) {
            if (p.id !== product.id && p.name === value) {
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
        (
            product.name === name &&
            product.description === description
        ) ? FormValidationState.ERROR : FormValidationState.SUCCESS
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
        const result = await onUpdate(product, {
            name,
            description,
            // TODO img
        });
        if (result === null) {
            setIsExpanded(false);
        }
        else {
            setFeedback(result);
        }
    };
    const onDeleteClick = async (e: FormEvent) => {
        e.preventDefault();
        const result = await onDelete(product);
        if (result === null) {
            setIsExpanded(false);
        }
        else {
            setFeedback(result);
        }
    }

    return <div className="product-card padding with-border col gap">
        <div className="row space-between wrap gap full-w">
            <h2 className="no-wrap">Edit {product.name}</h2>
            <div>
                <button
                    className="btn btn-primary no-animation"
                    onClick={toggleIsExpanded}
                >
                    Cancel
                </button>
            </div>
        </div>
        <form className="col gap">
            <TextField
                name="name"
                label="Name"
                type={TextFieldType.TEXT}
                initialValue={name}
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
            <button
                className="btn btn-danger"
                type="button"
                onClick={onDeleteClick}
            >
                Delete
            </button>
            <ValidationFeedback
                isOn={feedback !== ""}
                type={"invalid"}
                message={feedback}
            />
        </form>
    </div>;
};

export default ModifyProduct;
