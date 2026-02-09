import { useState } from "react";
import type { Product, Team, Uuid } from "../../types";
import AddProduct from "./AddProduct";
import ModifyProduct from "./ModifyProduct";
import type { ProductRequest } from "../../api/versions/v1/types";
import productService from "../../api/versions/v1/productService";

interface Props {
    team: Team;
    products: Product[];
}

const ACTION_OK = null;

const ProductManager = ({
    team,
    products: initialProducts,
}: Props) => {
    const [products, setProducts] = useState<Product[]>(initialProducts);

    const addProduct = async (newProduct: ProductRequest) => {
        console.log("Adding product:", newProduct);
        const result = await productService.createProduct(team.id, newProduct);
        if (result.isErr()) {
            const error = result.unwrapErr();
            console.error("Error creating product", error.detail.message);
            return error.detail.message;
        }
        const product = result.unwrap().data;
        setProducts([...products, product]
            .sort((a, b) => a.name.localeCompare(b.name))
        );
        return ACTION_OK;
    };
    const updateProduct = async (product: Product, productRequest: ProductRequest) => {
        console.log("Updating product:", product, productRequest);
        const result = await productService.updateProduct(team.id, product.id, productRequest);
        if (result.isErr()) {
            const error = result.unwrapErr();
            console.error("Error updating product", error.detail.message);
            return error.detail.message;
        }
        const newProduct = {
            ...product,
            ...productRequest,
        };
        setProducts(products
            .map(p => p.id === product.id ? newProduct : p)
            .sort((a, b) => a.name.localeCompare(b.name))
        );
        return ACTION_OK;
    };
    const deleteProduct = async (product: Product) => {
        console.log("Deleting product:", product);
        const result = await productService.deleteProduct(team.id, product.id);
        if (result.isErr()) {
            const error = result.unwrapErr();
            console.error("Error deleting product", error.detail.message);
            return error.detail.message;
        }
        setProducts(products.filter(p => p.id !== product.id));
        return ACTION_OK;
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
