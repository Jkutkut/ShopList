import { PRODUCTS } from "../../mockup";
import Product from "./Product";
import { TextField, TextFieldType } from "../form/textField";
import useExpanded from "../../hooks/useExpanded";

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
            <div className="header row space-between padding" onClick={toggleExpanded}>
                <h2>{category.name}</h2>
                <a href="" className="btn btn-primary no-animation">✏️</a>
            </div>
        ||
            <div className="header padding" onClick={toggleExpanded}>
                <h2>Uncategorized</h2>
            </div>
        }
        {expanded && <>
            {productsList.length > 0 &&
                <div className="row center">
                    <TextField
                        name="new-product"
                        type={TextFieldType.TEXT}
                        placeholder="Add a product"
                        onChange={(...args) => console.log("onChange", args)}
                        className="full-w margin"
                    />
                    <a href="" className="btn btn-primary margin">+</a>
                </div>
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
            <div className="row center">
                <TextField
                    name="new-product"
                    type={TextFieldType.TEXT}
                    placeholder="Add a product"
                    onChange={(...args) => console.log("onChange", args)}
                    className="full-w margin"
                />
                <a href="" className="btn btn-primary margin">+</a>
            </div>
        </>}
    </div>;
};

export default Category;
