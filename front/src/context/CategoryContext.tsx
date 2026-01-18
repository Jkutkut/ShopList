import React, { createContext, useReducer } from "react";
import { categoryReducer, type CategoryAction } from "./categoryReducer";

interface CategoryContextType {
    id: string;
    dispatch: React.Dispatch<CategoryAction>;
}

const createCategoryContext: (id: string) => CategoryContextType = (id) => {
    return {
        id,
        dispatch: () => {},
    };
};

const CategoryContext = createContext<CategoryContextType>(createCategoryContext(""));

interface CategoryContextProviderProps {
    id: string;
    children: React.ReactNode;
};

const CategoryContextProvider = ({ id, children }: CategoryContextProviderProps) => {
    const [state, dispatch] = useReducer(categoryReducer, createCategoryContext(id));
    return <CategoryContext.Provider value={{...state, dispatch}}>
        {children}
    </CategoryContext.Provider>;
}

export { createCategoryContext, CategoryContext, CategoryContextProvider };
export type { CategoryContextType };
