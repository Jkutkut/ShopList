import { DndContext } from "@dnd-kit/core";
import { ACTION, CATEGORIES, LIST_PRODUCTS } from "../../mockup";
import Category from "./Category";

interface Props {
    id: keyof typeof LIST_PRODUCTS
}

const ListLayout = ({
    id,
}: Props) => {
    const categories = CATEGORIES.filter((c) => c.listId === id);
    const listProducts = LIST_PRODUCTS[id];

    return <section className="list-layout col gap">
        <DndContext
            onDragStart={(e) => console.log(`Drag start: ${e.active.id}`)}
            onDragEnd={(e) => console.log(`Drag end: ${e.active.id} over ${e.over?.id}`)}
            onDragAbort={() => console.log("Drag aborted")}
            onDragCancel={() => console.log("Drag cancelled")}
            onDragOver={(e) => console.log(`Drag over: ${e.over?.id}`)}
            onDragMove={(e) => console.log(`Drag move: ${e.active.id}`)}
            onDragPending={() => console.log("Drag pending")}
        >
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
        </DndContext>
    </section>;
};

export default ListLayout;
