import { ACTION, PRODUCTS } from "../../mockup";
import Product from "./Product";
import useExpanded from "../../hooks/useExpanded";
import AddProduct from "./AddProduct";
import { DndContext } from "@dnd-kit/core";
import Droppable from "../dnd/Droppable";
import Draggable from "../dnd/Draggable";

interface Props {
    category?: any;
    productsList: any[];
};

interface CategoryHeaderProps {
    category?: any;
    isExpanded: boolean;
    toggleExpanded: () => void;
    dndHandle?: React.ReactNode;
};

const CategoryHeader = ({
    category,
    isExpanded, toggleExpanded,
    dndHandle,
}: CategoryHeaderProps) => {
    const categoryLabel = category ? category.name : "Uncategorized";
    return <div className="header row">
        <h2 className="full-w padding no-wrap" onClick={toggleExpanded}>{categoryLabel}</h2>
        {category && isExpanded &&
            <a
                className="btn btn-primary no-animation margin center"
                onClick={ACTION("Edit category")}
            >
                ✏️
            </a>
        }
        {dndHandle}
    </div>
};

interface CategoryDetailProps {
    productsList: any[];
};

const CategoryDetail = ({
    productsList,
}: CategoryDetailProps) => {
    return <>
        {productsList.length > 0 && <AddProduct /> }
        <DndContext
            onDragStart={(e) =>  console.log(`Category ctx: Drag start: ${e.active.id}`)}
            onDragEnd={(e) =>    console.log(`Category ctx: Drag end: ${e.active.id} over ${e.over?.id}`)}
            onDragAbort={() =>   console.log("Category ctx: Drag aborted")}
            onDragCancel={() =>  console.log("Category ctx: Drag cancelled")}
            onDragOver={(e) =>   console.log(`Category ctx: Drag over: ${e.over?.id}`)}
            onDragMove={(e) =>   console.log(`Category ctx: Drag move: ${e.active.id}`)}
            onDragPending={() => console.log("Category ctx: Drag pending")}
        >
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
        </DndContext>
        <AddProduct />
    </>;
};

const Category = ({
    category,
    productsList
}: Props) => {
    const { isExpanded, toggleIsExpanded } = useExpanded(true);
    // console.log("category", category, productsList);
    const categoryId = category ? category.id : "uncategorized";

    const {
        node: dndHandle,
        style: dndStyle,
    } = Draggable({
        id: `drag-cat-${categoryId}`,
        className: "btn btn-primary no-animation margin center",
        style: {
            exportStyles: true,
            xTranslate: false,
            yTranslate: true,
        },
        children: "⠿",
    });
    return <Droppable
        id={`drop-${categoryId}`}
        className="category col with-border"
        style={dndStyle}
    >
        <CategoryHeader
            category={category}
            isExpanded={isExpanded} toggleExpanded={toggleIsExpanded}
            dndHandle={dndHandle}
        />
        {isExpanded && <CategoryDetail productsList={productsList} />}
    </Droppable>;
};

export default Category;
