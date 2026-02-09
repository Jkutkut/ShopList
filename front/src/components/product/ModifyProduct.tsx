import type { ProductRequest } from "../../api/versions/v1/types";
import type { Product } from "../../types";
import DateLabel from "../date/DateLabel";
import type { Props as ManagerProps } from "./ProductManager";

interface Props extends ManagerProps {
    product: Product;
    onUpdate: (product: Product, productRequest: ProductRequest) => void;
    onDelete: (product: Product) => void;
}

const ModifyProduct = ({
    product,
    team,
    products,
    onUpdate,
    onDelete
}: Props) => {
    return <div key={product.name} className="product-card padding with-border col">
        <div className="row wrap space-between">
            {product.name &&
                <h2 className="no-wrap">{product.name}</h2>
            }
            {product.image &&
                <img src={product.image} alt={product.name} />
            }
        </div>
        <div className="col gap">
            {product.description &&
                <p>{product.description}</p>
            }
            <div className="row space-between wrap gap">
                <span className="">
                Created by {product.created_by} <DateLabel date={product.created_at} />
                </span>
                <span className="">
                Updated by {product.updated_by} <DateLabel date={product.updated_at} />
                </span>
            </div>
            <a className="btn btn-primary btn-small no-animation">✏️</a>
        </div>
    </div>;
};

export default ModifyProduct;
