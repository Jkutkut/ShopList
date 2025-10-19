import { useEffect, useState } from "react";
import { TextField, TextFieldType } from "../form/textField";
import { PRODUCTS } from "../../mockup";

const SUGGEST_AFTER = 300;

interface Props {
    suggestAfter?: number
}

const AddProduct = ({
    suggestAfter = SUGGEST_AFTER
}: Props) => {
    const [query, setQuery] = useState<string>("");
    const [suggestions, setSuggestions] = useState<any[]>([]);

    useEffect(() => {
        const timer = setTimeout(() => {
            suggestProducts(query);
        }, suggestAfter);
        return () => clearTimeout(timer);
    });
    const onChange = (e: React.ChangeEvent<HTMLInputElement>) => setQuery(e.target.value);
    const ofFocus = () => suggestProducts(query);
    const onBlur = () => setSuggestions([]);
    const suggestProducts = (query: string) => {
        let newSuggestions: any[] = [];
        if (query.length >= 3) {
            newSuggestions = PRODUCTS
                .filter((p) =>
                    p.name.toLowerCase().includes(query.toLowerCase()) ||
                    p.description.toLowerCase().includes(query.toLowerCase())
                );
        }
        setSuggestions(newSuggestions);
    };
    return <>
        <div className="addProduct row center">
            <div className="col full-w margin">
                <TextField
                    name="new-product"
                    type={TextFieldType.TEXT}
                    initialValue={query}
                    placeholder="Add a product"
                    onChange={onChange}
                    onInputFocus={ofFocus}
                    onInputBlur={onBlur}
                />
                {suggestions.length > 0 &&
                    <div className="product-suggestions col">
                        {suggestions.map((suggestion) => (
                            <div className="suggestion padding with-border margin" key={suggestion.name}>
                                <div className="row gap">
                                    <span className="no-wrap">
                                        {suggestion.name}
                                    </span>
                                    <span className="overflow-ellipsis">
                                        {suggestion.description}
                                    </span>
                                </div>
                            </div>
                        ))}
                    </div>
                }
            </div>
            <a href="" className="btn btn-primary margin">+</a>
        </div>
    </>;
};

export default AddProduct;
