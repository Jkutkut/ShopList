import React, { createContext, useEffect, useReducer } from "react";
import { ListActionType, listReducer, type ListAction } from "./listReducer";
import { CATEGORIES, LIST_PRODUCTS, PRODUCTS } from "../mockup";

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

    useEffect(() => {
        // TODO Simulate loading
        setTimeout(() => {
            dispatch({
                type: ListActionType.SET_CATEGORIES,
                payload: CATEGORIES.filter((c) => c.listId === id)
            });
        }, 100);
        setTimeout(() => {
            dispatch({
                type: ListActionType.SET_LIST_PRODUCTS,
                payload: LIST_PRODUCTS[id as keyof typeof LIST_PRODUCTS] || []
            })
        }, 100);
        setTimeout(() => {
            dispatch({
                type: ListActionType.SET_PRODUCTS,
                payload: PRODUCTS,
            })
        }, 100);
    }, [id]);

    return <ListContext.Provider value={{...state, dispatch}}>
        {children}
    </ListContext.Provider>;
}

export { createListContext, ListContext, ListContextProvider };
export type { ListContextType };
