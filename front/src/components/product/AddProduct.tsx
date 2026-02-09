import type { ProductRequest } from "../../api/versions/v1/types";
import useExpanded from "../../hooks/useExpanded";
import type { Props as ManagerProps } from "./ProductManager";

interface Props extends ManagerProps {
    onAdd: (product: ProductRequest) => void
}

const AddProduct = ({
    team,
    products
}: Props) => {
    const { isExpanded, toggleIsExpanded } = useExpanded(false);
    
    if (!isExpanded) {
        return <button
            className="btn btn-primary no-animation"
            onClick={toggleIsExpanded}
        >
            Add a product
        </button>;
    }
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
    </div>;
};

export default AddProduct;
