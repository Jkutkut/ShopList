import { IMAGES, PRODUCTS, TEXT } from "../../mockup";
import { TextField, TextFieldType } from "../form/textField";

interface Props {
    id: string
}

const ListLayout = ({
    id,
}: Props) => {
    return <section className="list-layout col gap">
        <a href="" className="btn btn-primary">Create new category</a>
        <div className="categories with-border">
            <div className="category col">
                <div className="header row space-between padding">
                    <h2>Category name</h2>
                    <a href="" className="btn btn-primary">✏️</a>
                </div>
                <div className="products">
                    <div className="product col with-border margin">
                        <div className="header row space-between padding">
                            <div className="row gap center">
                                <a href="" className="btn btn-primary">❌</a>
                                <h3>Product name</h3>
                            </div>
                            <div className="row half-gap center">
                                {/* <span>12 L</span> */}
                                <TextField
                                    name="amount"
                                    type={TextFieldType.TEXT}
                                    placeholder="Amount"
                                    onChange={(...args) => console.log("onChange", args)}
                                    className="product-amount"
                                />
                                <TextField
                                    name="unit"
                                    type={TextFieldType.TEXT}
                                    placeholder="Unit"
                                    onChange={(...args) => console.log("onChange", args)}
                                    className="product-unit"
                                />
                                <a href="" className="btn btn-primary">⬆</a>
                                <a href="" className="btn btn-primary">⬇</a>
                            </div>
                        </div>
                        <div className="content col gap padding">
                            <div className="row space-between">
                                <a href="" className="btn btn-primary">✏️</a>
                                <div className="col">
                                    <span className="">
                                        Added by {PRODUCTS[0].createdBy} on <span className="date2format" data-date={new Date().toISOString()}></span>
                                    </span>
                                    <span className="">
                                        Updated by: {PRODUCTS[0].updatedBy} on <span className="date2format" data-date={new Date().toISOString()}></span>
                                    </span>
                                </div>
                            </div>
                            <p>{TEXT['3']}</p>
                            <div className="imgs row gap wrap center">
                                <img src={IMAGES['1:1']} alt="" />
                                <img src={IMAGES['1:1']} alt="" />
                                <img src={IMAGES['2:1']} alt="" />
                                <img src={IMAGES['1:1']} alt="" />
                                <img src={IMAGES['2:1']} alt="" />
                                <img src={IMAGES['2:1']} alt="" />
                                <img src={IMAGES['1:1']} alt="" />
                                <img src={IMAGES['2:1']} alt="" />
                                <img src={IMAGES['1:1']} alt="" />
                            </div>
                        </div>
                    </div>
                    <div className="product col with-border margin">
                        <div className="header row space-between padding">
                            <div className="row gap center">
                                <a href="" className="btn btn-primary">❌</a>
                                <h3>Product name</h3>
                            </div>
                            <div className="row half-gap center">
                                <span>12 L</span>
                                <a href="" className="btn btn-primary">⬆</a>
                                <a href="" className="btn btn-primary">⬇</a>
                            </div>
                        </div>
                    </div>
                    <div className="product col with-border margin">
                        <div className="header row space-between padding">
                            <div className="row gap center">
                                <a href="" className="btn btn-primary">❌</a>
                                <h3>Product name</h3>
                            </div>
                            <div className="row half-gap center">
                                <a href="" className="btn btn-primary">⬆</a>
                                <a href="" className="btn btn-primary">⬇</a>
                            </div>
                        </div>
                    </div>
                    <div className="product col with-border margin">
                        <div className="header row space-between padding">
                            <div className="row gap center">
                                <a href="" className="btn btn-primary">❌</a>
                                <h3>Product name</h3>
                            </div>
                            <div className="row half-gap center">
                                <span>12 L</span>
                                <a href="" className="btn btn-primary">⬆</a>
                                <a href="" className="btn btn-primary">⬇</a>
                            </div>
                        </div>
                    </div>
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
            </div>
        </div>
        <a href="" className="btn btn-primary">Create new category</a>
    </section>;
};

export default ListLayout;
