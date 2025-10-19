import { ACTION, PRODUCTS } from "../../mockup";
import Product from "./Product";
import useExpanded from "../../hooks/useExpanded";
import AddProduct from "./AddProduct";

interface Props {
    category?: any;
    productsList: any[];
};

const Category = ({
    category,
    productsList
}: Props) => {
    const { expanded, toggleExpanded } = useExpanded(true);

    console.log("category", category, productsList);
    return <div className="category col with-border">
        {category &&
            <div className="header row">
                <h2 className="full-w padding" onClick={toggleExpanded}>{category.name}</h2>
                <a className="btn btn-primary no-animation margin" onClick={ACTION("Edit category")}>✏️</a>
            </div>
        ||
            <div className="header padding" onClick={toggleExpanded}>
                <h2>Uncategorized</h2>
            </div>
        }
        {expanded && <>
            {productsList.length > 0 &&
                <AddProduct />
            }
            <div className="products">
                {productsList.map((p, idx) => (
                    <Product
                        key={idx}
                        productList={p}
                        product={PRODUCTS.find((product) => product.id === p.productId)}
                    />
                ))}
                {productsList.length === 0 &&
                    <p className="padding">No products</p>
                }
            </div>
            <AddProduct />
        </>}
    </div>;
};

export default Category;
