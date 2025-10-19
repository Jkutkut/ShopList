import { ACTION, CATEGORIES, LIST_PRODUCTS, LISTS } from "../../mockup";
import Category from "./Category";

interface Props {
    id: keyof typeof LIST_PRODUCTS
}

const ListLayout = ({
    id,
}: Props) => {
    // const list = LISTS.find((l) => l.id === id);
    const categories = CATEGORIES.filter((c) => c.listId === id);
    const listProducts = LIST_PRODUCTS[id];

    return <section className="list-layout col gap">
        <a className="btn btn-primary" onClick={ACTION("new category")}>Create new category</a>
        <div className="categories col gap">
            {categories.map((c) => (
                <Category
                    key={c.id}
                    category={c}
                    productsList={listProducts.filter((p) => p.categoryId === c.id)}
                />
            ))}
            <Category
                category={undefined}
                productsList={listProducts.filter((p) => p.categoryId === undefined)}
            />
        </div>
        <a className="btn btn-primary" onClick={ACTION("new category")}>Create new category</a>
    </section>;
};

export default ListLayout;
