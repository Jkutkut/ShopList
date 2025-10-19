import useExpanded from "../../hooks/useExpanded";
import { IMAGES } from "../../mockup";
import DateLabel from "../date/DateLabel";
import { TextField, TextFieldType } from "../form/textField";
import arrowUp from "../../assets/arrow-up.svg";
import arrowDown from "../../assets/arrow-down.svg";
import xMark from "../../assets/x-mark.svg";

interface Props {
    productList: any
    product: any
};

const Product = ({
    productList,
    product,
}: Props) => {
    const { expanded, toggleExpanded } = useExpanded(false);
    console.log("product", product);
    return <div className="product col with-border margin">
        <div className="header row space-between half-padding" onClick={toggleExpanded}>
            <div className="row gap center">
                <a href="" className="btn btn-small no-animation">
                    <img src={xMark.src} alt="" width={10} />
                </a>
                <h3>{product.name}</h3>
            </div>
            <div className="row half-gap center">
                {!expanded &&
                    <span>{productList.amount} {productList.unit}</span>
                }
                <a href="" className="btn btn-primary btn-small no-animation">
                    <img src={arrowUp.src} alt="" width={10} />
                </a>
                <a href="" className="btn btn-primary btn-small no-animation">
                    <img src={arrowDown.src} alt="" width={10} />
                </a>
            </div>
        </div>
        {expanded &&
            <div className="content col gap half-padding">
                <div className="row gap space-between wrap">
                    <div className="row gap space-between">
                        <a href="" className="btn btn-primary btn-small no-animation">✏️</a>
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
                        <img src={IMAGES['1:1']} alt="" />
                    </div>
                }
            </div>
        }
    </div>;
};

export default Product;
