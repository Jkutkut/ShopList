import { useEffect, useState } from "react";
import { TextField, TextFieldType } from "../form/textField";
import { ACTION } from "../../mockup";
import useListContext from "../../hooks/useListContext";
import { ListActionType } from "../../context/listReducer";

const SUGGEST_AFTER = 300;

interface Props {
    categoryId: string;
    addPosition: 0 | -1;
    suggestAfter?: number;
}

const AddProduct = ({
    categoryId,
    addPosition,
    suggestAfter = SUGGEST_AFTER
}: Props) => {
    const { listProducts, searchProductsByQuery, dispatch } = useListContext();
    const [query, setQuery] = useState<string>("");
    const [suggestions, setSuggestions] = useState<any[]>([]);
    useEffect(() => {
        const timer = setTimeout(() => {
            suggestProducts(query);
        }, suggestAfter);
        return () => clearTimeout(timer);
    }, [query]);
    const onChange = (e: React.ChangeEvent<HTMLInputElement>) => setQuery(e.target.value);
    const onFocus = () => suggestProducts(query);
    const onBlur = () => setTimeout(() => setSuggestions([]), 50);
    const suggestProducts = (query: string) => {
        let newSuggestions: any[] = [];
        if (query.length >= 3) {
            newSuggestions = searchProductsByQuery(query).filter((p) => !listProducts.find(
                (lp) => lp.categoryId === categoryId && lp.productId === p.id
            ));
        }
        setSuggestions(newSuggestions);
    };
    const addSuggestion = (suggestion: any) => { // TODO type product
        dispatch({
            type: ListActionType.ADD_PRODUCT_TO_CATEGORY_LIST,
            payload: {
                categoryId,
                productId: suggestion.id,
                index: addPosition
            }
        });
        setQuery("");
        setSuggestions([]);
    };
    return <>
        <div
            className="addProduct row center"
            onFocus={onFocus}
        >
            <div className="col full-w margin">
                <TextField
                    name="new-product"
                    type={TextFieldType.TEXT}
                    autocomplete="off"
                    initialValue={query}
                    placeholder="Add a product"
                    onChange={onChange}
                    onInputFocus={onFocus}
                    onInputBlur={onBlur}
                />
                {suggestions.length > 0 &&
                    <div
                        className="product-suggestions col"
                    >
                        {suggestions.map((suggestion) => (
                            <a
                                key={suggestion.name}
                                className="suggestion padding with-border margin"
                                onClick={() => addSuggestion(suggestion)}
                            >
                                <div className="row gap">
                                    <span className="no-wrap">
                                        {suggestion.name}
                                    </span>
                                    <span className="overflow-ellipsis">
                                        {suggestion.description}
                                    </span>
                                </div>
                            </a>
                        ))}
                    </div>
                }
            </div>
            <a className="btn btn-primary margin" onClick={ACTION("click add product")}>+</a>
        </div>
    </>;
};

export default AddProduct;
