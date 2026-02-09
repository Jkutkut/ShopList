import { useState } from "react";
import type { Product, Team, Uuid } from "../../types";
import DateLabel from "../date/DateLabel";
import AddProduct from "./AddProduct";
import ModifyProduct from "./ModifyProduct";
import type { ProductRequest } from "../../api/versions/v1/types";

interface Props {
    team: Team;
    products: Product[];
}

const ProductManager = ({
    team,
    products: initialProducts,
}: Props) => {
    const [products, setProducts] = useState<Product[]>(initialProducts);

    const addProduct = (newProduct: ProductRequest) => {
        console.log("Adding product:", newProduct);
    };
    const updateProduct = (product: Product, productRequest: ProductRequest) => {
        console.log("Updating product:", product, productRequest);
    };
    const deleteProduct = (product: Product) => {
        console.log("Deleting product:", product);
    };

    console.debug("Rendering ProductManager");
    return <div className="col gap">
        {products.length > 3 && <div>
            <AddProduct team={team} products={products} onAdd={addProduct} />
        </div>}
        <div className="col gap">
            {products.map(product => (
                <ModifyProduct
                    key={product.id}
                    product={product}
                    team={team}
                    products={products}
                    onUpdate={updateProduct}
                    onDelete={deleteProduct}
                />
            ))}
        </div>
        <div>
            <AddProduct team={team} products={products} onAdd={addProduct} />
        </div>
    </div>;
};

export default ProductManager;
export type { Props };
