import useExpanded from "../../hooks/useExpanded";
import { ACTION } from "../../mockup";
import DateLabel from "../date/DateLabel";
import { TextField, TextFieldType } from "../form/textField";
import arrowUp from "../../assets/arrow-up.svg";
import arrowDown from "../../assets/arrow-down.svg";
import xMark from "../../assets/x-mark.svg";
import Droppable from "../dnd/Droppable";
import Draggable from "../dnd/Draggable";
import { useContext } from "react";
import { ListContext } from "../../context/ListContext";
import useListContext from "../../hooks/useListContext";
import { DndType } from "../../context/listReducer";

interface Props {
    productList: any;
};

interface ProductDetailProps {
    productList: any;
    product: any;
};

const ProductDetail = ({
    productList,
    product,
}: ProductDetailProps) => {
    return <div className="content col gap half-padding">
        <div className="row gap space-between wrap">
            <div className="row gap space-between full-w wrap">
                <a
                    className="btn btn-primary btn-small no-animation"
                    onClick={ACTION("Edit product")}
                >✏️</a>
                <div className="row gap wrap">
                    <TextField
                        name="amount"
                        type={TextFieldType.TEXT}
                        initialValue={productList.amount}
                        placeholder="Amount"
                        onChange={(...args) => console.log("onChange", args)}
                        className="product-amount"
                    />
                    <TextField
                        name="unit"
                        type={TextFieldType.TEXT}
                        initialValue={productList.unit}
                        placeholder="Unit"
                        onChange={(...args) => console.log("onChange", args)}
                        className="product-unit"
                    />
                </div>
            </div>
            <div className="col">
                <span className="">
                    Added by {productList.createdBy} <DateLabel date={productList.createdAt} />
                </span>
                <span className="">
                    Updated by {productList.updatedBy} <DateLabel date={productList.updatedAt} />
                </span>
            </div>
        </div>
        <p>{product.description}</p>
        {product.image &&
            <div className="imgs row gap wrap center">
                <img src={product.image} alt="" />
            </div>
        }
    </div>;
};

interface ProductDndHandleProps {
    productId: string;
    usingDnd: boolean;
};

const ProductDnDHandle = ({
    productId,
    usingDnd
}: ProductDndHandleProps) => {
    if (usingDnd) {
        return Draggable({
            id: `drag-${DndType.PRODUCT}_${productId}`,
            className: "btn btn-primary btn-small no-animation center",
            style: {
                exportStyles: true,
                xTranslate: false,
                yTranslate: true,
            },
            children: "⠿",
        });
    }
    return {
        node: <div className="row half-gap">
            <a
                key="up"
                className="btn btn-primary btn-small no-animation"
                onClick={ACTION("Up product")}
            >
                <img src={arrowUp.src} alt="Move product up" width={10} />
            </a>
            <a
                key="down"
                className="btn btn-primary btn-small no-animation"
                onClick={ACTION("Down product")}
            >
                <img src={arrowDown.src} alt="Move product down" width={10} />
            </a>
        </div>,
        style: undefined,
    };
};

const Product = ({
    productList,
}: Props) => {
    const { getProductById } = useListContext();
    const product = getProductById(productList.productId); // TODO handle undefined
    const {
        node: draggable,
        style: dragStyle,
    } = ProductDnDHandle({
        productId: product.id,
        usingDnd: true,
    });
    const { isExpanded, toggleIsExpanded } = useExpanded(false);
    return <Droppable
        id={`drop-${DndType.PRODUCT}_${product.id}`}
        className="product col with-border margin"
        style={dragStyle}
    >
        <div className="header row half-padding half-gap space-between">
            <a className="btn btn-small no-animation" onClick={ACTION("Delete product")}>
                <img src={xMark.src} alt="" width={10} />
            </a>
            <div className="row center space-between full-w" onClick={toggleIsExpanded}>
                <h3 className="no-wrap">{product.name}</h3>
                {!isExpanded &&
                    <div className="row half-gap center">
                        <span>{productList.amount} {productList.unit}</span>
                    </div>
                }
            </div>
            <div className="row half-gap">
                {draggable}
            </div>
        </div>
        {isExpanded &&
            <ProductDetail {...{productList, product}} />
        }
    </Droppable>;
};

export default Product;
