import React, { createContext, useEffect, useReducer } from "react";
import { ListActionType, listReducer, type ListAction } from "./listReducer";
import { CATEGORIES, LIST_PRODUCTS, PRODUCTS } from "../mockup";
import usePersistedValue, { NO_EXPIRATION } from "../hooks/usePersistedValue";

interface ListContextType {
    id: string;
    categories: any[]; // TODO type
    listProducts: any[]; // TODO type
    products: any[]; // TODO type

    setCategories: (categories: any[]) => void; // TODO type
    setListProducts: (listProducts: any[]) => void; // TODO type
    setProducts: (products: any[]) => void; // TODO type

    dispatch: React.Dispatch<ListAction>;

    events: {
        inCategoryDnd: boolean;
        inProductDnd: boolean;
        dndId?: string;
        dndOverId?: string;
    }
}

const createListContext: (id: string) => ListContextType = (id) => {
    return {
        id,
        categories: [],
        listProducts: [],
        products: [],
        setCategories: () => {},
        setListProducts: () => {},
        setProducts: () => {},
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
    // TODO replace mockup with real fetch functions
    const {
        value: categories,
        isLoading: isLoadingCategories,
        setValue: setCategories
    } = usePersistedValue<any[]>({
        key: `list-${id}-categories`,
        fetchFunc: async () => {
            console.debug(`Fetching categories for list ${id} from mockup`);
            return CATEGORIES.filter((c) => c.listId === id);
        },
        expiration: NO_EXPIRATION,
    });
    const {
        value: listProducts,
        isLoading: isLoadingListProducts,
        setValue: setListProducts
    } = usePersistedValue<any[]>({
        key: `list-${id}-list-products`,
        fetchFunc: async () => {
            console.debug(`Fetching list products for list ${id} from mockup`);
            return LIST_PRODUCTS[id as keyof typeof LIST_PRODUCTS] || [];
        },
        expiration: NO_EXPIRATION,
    });
    const {
        value: products,
        isLoading: isLoadingProducts,
        setValue: setProducts
    } = usePersistedValue<any[]>({
        key: `list-${id}-products`,
        fetchFunc: async () => {
            console.debug(`Fetching products for list ${id} from mockup`);
            return PRODUCTS;
        },
        expiration: NO_EXPIRATION,
    });

    const [state, dispatch] = useReducer(
      listReducer,
      {
        ...createListContext(id),
        setCategories,
        setListProducts,
        setProducts,
      }
    );

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
