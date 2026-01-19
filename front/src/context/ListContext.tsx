import React, { createContext, useEffect, useReducer } from "react";
import { ListActionType, listReducer, type ListAction } from "./listReducer";
import { CATEGORIES, LIST_PRODUCTS, PRODUCTS } from "../mockup";
import useCachedValue, { NO_EXPIRATION } from "../hooks/useCachedValue";

interface ListContextType {
    id: string;
    categories: any[]; // TODO type
    listProducts: any[]; // TODO type
    products: any[]; // TODO type
    dispatch: React.Dispatch<ListAction>;

    events: {
        inCategoryDnd: boolean;
        inProductDnd: boolean;
        dndId?: string;
    }
}

const createListContext: (id: string) => ListContextType = (id) => {
    return {
        id,
        categories: [],
        listProducts: [],
        products: [],
        dispatch: () => {},
        events: {
            inCategoryDnd: false,
            inProductDnd: false,
        }
    };
};

const ListContext = createContext<ListContextType>(createListContext(""));

interface ListContextProviderProps {
    id: string;
    children: React.ReactNode;
};

const ListContextProvider = ({ id, children }: ListContextProviderProps) => {
    const [state, dispatch] = useReducer(listReducer, createListContext(id));

    // TODO replace mockup with real fetch functions
    const {
        value: categories,
        isLoading: isLoadingCategories
    } = useCachedValue<any[]>({
        key: `list-${id}-categories`,
        fetchFunc: async () => {
            console.debug(`Fetching categories for list ${id} from mockup`);
            return CATEGORIES.filter((c) => c.listId === id);
        },
        expiration: NO_EXPIRATION,
    });
    const {
        value: listProducts,
        isLoading: isLoadingListProducts
    } = useCachedValue<any[]>({
        key: `list-${id}-list-products`,
        fetchFunc: async () => {
            console.debug(`Fetching list products for list ${id} from mockup`);
            return LIST_PRODUCTS[id as keyof typeof LIST_PRODUCTS] || [];
        },
        expiration: NO_EXPIRATION,
    });
    const {
        value: products,
        isLoading: isLoadingProducts
    } = useCachedValue<any[]>({
        key: `list-${id}-products`,
        fetchFunc: async () => {
            console.debug(`Fetching products for list ${id} from mockup`);
            return PRODUCTS;
        },
        expiration: NO_EXPIRATION,
    });

    useEffect(() => {
        if (!isLoadingCategories && categories) {
            dispatch({
                type: ListActionType.SET_CATEGORIES,
                payload: categories
            });
        }
    }, [isLoadingCategories, categories]);
    useEffect(() => {
        if (!isLoadingListProducts && listProducts) {
            dispatch({
                type: ListActionType.SET_LIST_PRODUCTS,
                payload: listProducts
            });
        }
    }, [isLoadingListProducts, listProducts]);
    useEffect(() => {
        if (!isLoadingProducts && products) {
            dispatch({
                type: ListActionType.SET_PRODUCTS,
                payload: products
            });
        }
    }, [isLoadingProducts, products]);

    // TODO handle live updates with server
    useEffect(() => {
        const bc = new BroadcastChannel(`list-${id}`);
        bc.onmessage = (event) => dispatch(JSON.parse(event.data));

        return () => bc.close();
    }, [id]);

    return <ListContext.Provider value={{...state, dispatch}}>
        {children}
    </ListContext.Provider>;
}

export { createListContext, ListContext, ListContextProvider };
export type { ListContextType };
