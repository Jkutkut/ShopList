import useExpanded from "../../hooks/useExpanded";
import { ACTION } from "../../mockup";
import DateLabel from "../date/DateLabel";
import { TextField, TextFieldType } from "../form/textField";
import arrowUp from "../../assets/arrow-up.svg";
import arrowDown from "../../assets/arrow-down.svg";
import xMark from "../../assets/x-mark.svg";
import Droppable from "../dnd/Droppable";
import Draggable from "../dnd/Draggable";
import useListContext from "../../hooks/useListContext";
import { DndType, ListActionType } from "../../context/listReducer";
import { useEffect, useState } from "react";
import { set } from "astro:schema";

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
    const [amount, setAmount] = useState<string>(productList.amount);
    const [unit, setUnit] = useState<string>(productList.unit);
    const { dispatch } = useListContext();

    const updateAmount = () => productList.amount !== amount && dispatch({
        type: ListActionType.UPDATE_PRODUCT_LIST_AMOUNT,
        payload: { id: productList.id, amount },
    });
    const updateUnit = () => productList.unit !== unit && dispatch({
        type: ListActionType.UPDATE_PRODUCT_LIST_UNIT,
        payload: { id: productList.id, unit },
    });

    useEffect(() => {
        const timer = setTimeout(updateAmount, 1000);
        return () => clearTimeout(timer);
    }, [amount]);
    useEffect(() => {
        const timer = setTimeout(updateUnit, 1000);
        return () => clearTimeout(timer);
    }, [unit]);

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
                        initialValue={amount}
                        placeholder="Amount"
                        onChange={(e) => setAmount(e.target.value)}
                        onInputBlur={updateAmount}
                        className="product-amount"
                    />
                    <TextField
                        name="unit"
                        type={TextFieldType.TEXT}
                        initialValue={unit}
                        placeholder="Unit"
                        onChange={(e) => setUnit(e.target.value)}
                        onInputBlur={updateUnit}
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
    id: string;
    usingDnd: boolean;
};

const ProductDnDHandle = ({
    id,
    usingDnd
}: ProductDndHandleProps) => {
    if (usingDnd) {
        return Draggable({
            id: `drag-${DndType.PRODUCT}_${id}`,
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
    const { getProductById, dispatch } = useListContext();
    const { isExpanded, toggleIsExpanded } = useExpanded(false);
    const product = getProductById(productList.productId); // TODO handle undefined
    const {
        node: draggable,
        style: dragStyle,
    } = ProductDnDHandle({
        id: productList.id,
        usingDnd: true,
    });
    const deleteProduct = () => {
        dispatch({
            type: ListActionType.REMOVE_PRODUCT_FROM_CATEGORY_LIST,
            payload: {
                productListId: productList.id
            },
        });
    }
    return <Droppable
        id={`drop-${DndType.PRODUCT}_${productList.id}`}
        className="product col with-border margin"
        style={dragStyle}
    >
        <div className="header row half-padding half-gap space-between">
            <a className="btn btn-small no-animation" onClick={deleteProduct}>
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
