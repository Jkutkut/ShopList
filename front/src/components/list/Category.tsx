import { ACTION } from "../../mockup";
import Product from "./Product";
import useExpanded from "../../hooks/useExpanded";
import AddProduct from "./AddProduct";
import { DndContext } from "@dnd-kit/core";
import Droppable from "../dnd/Droppable";
import Draggable from "../dnd/Draggable";
import { useContext } from "react";
import { ListContext } from "../../context/ListContext";
import { DndType, ListActionType } from "../../context/listReducer";
import useListContext from "../../hooks/useListContext";

interface Props {
    category?: any;
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
    const { dispatch } = useContext(ListContext);
    return <>
        {productsList.length > 0 && <AddProduct /> }
        <DndContext
            onDragStart={(e) => dispatch({
                type: ListActionType.DND_START,
                payload: { type: DndType.PRODUCT, id: `${e.active.id}` },
            })}
            onDragEnd={(e) => dispatch({
                type: ListActionType.DND_STOP,
                payload: { type: DndType.PRODUCT, id: `${e.active.id}` },
            })}
            onDragOver={(e) => dispatch({
                type: ListActionType.DND_OVER,
                payload: { type: DndType.PRODUCT, id: `${e.over?.id}` },
            })}
            onDragAbort={() => console.log("Drag aborted category")}
            onDragCancel={() => console.log("Drag cancelled category")}
        >
            <div className="products">
                {productsList.map((p, idx) => (
                    <Product
                        key={idx}
                        productList={p}
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
}: Props) => {
    const { getListProductsByCategoryId } = useListContext();
    const { isExpanded, toggleIsExpanded } = useExpanded(true);

    const categoryId = category ? category.id : undefined;
    const categoryIdLiteral = category ? category.id : "uncategorized";
    const productsList = getListProductsByCategoryId(categoryId);
    // TODO handle undefined category
    const {
        node: dndHandle,
        style: dndStyle,
    } = Draggable({
        id: `drag-cat-${categoryIdLiteral}`,
        className: "btn btn-primary no-animation margin center",
        style: {
            exportStyles: true,
            xTranslate: false,
            yTranslate: true,
        },
        children: "⠿",
    });
    return <Droppable
        id={`drop-${categoryIdLiteral}`}
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
