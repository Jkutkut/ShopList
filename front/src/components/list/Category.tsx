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
    categoryId: string;
    productsList: any[];
};

const CategoryDetail = ({
    categoryId,
    productsList,
}: CategoryDetailProps) => {
    return <>
        {productsList.length >= 5 &&
            <AddProduct categoryId={categoryId} addPosition={0} />
        }
        <div className="products">
            {productsList.sort((a, b) => a.index - b.index).map((p, idx) => (
                <Product
                    key={idx}
                    productList={p}
                />
            ))}
            {productsList.length === 0 &&
                <p className="padding">No products</p>
            }
        </div>
        <AddProduct categoryId={categoryId} addPosition={-1} />
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
    const {
        node: dndHandle,
        style: dndStyle,
    } = Draggable({
        id: `drag-${DndType.CATEGORY}_${categoryIdLiteral}`,
        className: "btn btn-primary no-animation margin center",
        style: {
            exportStyles: true,
            xTranslate: false,
            yTranslate: true,
        },
        children: "⠿",
    });
    return <Droppable
        id={`drop-${DndType.CATEGORY}_${categoryIdLiteral}`}
        className="category col with-border"
        style={dndStyle}
    >
        <CategoryHeader
            category={category}
            isExpanded={isExpanded} toggleExpanded={toggleIsExpanded}
            dndHandle={category && dndHandle}
        />
        {isExpanded && <CategoryDetail categoryId={categoryId} productsList={productsList} />}
    </Droppable>;
};

export default Category;
