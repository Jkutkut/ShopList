import { useContext } from "react";
import { ListContext } from "../context/ListContext";

const useListContext = () => {
  const ctx = useContext(ListContext);
  const {
    products, categories, listProducts,
  } = ctx;

  const getProductById = (id: string) => {
    return products.find((product) => product.id === id);
  };
  const getCategoryById = (id: string) => {
    return categories.find((category) => category.id === id);
  };
  const getListProductsByCategoryId = (categoryId?: string) => {
    return listProducts.filter((p) => p.categoryId === categoryId);
  };
  const searchProductsByQuery = (query: string) => {
    const lowerQuery = query.toLowerCase();
    return products.filter((p) =>
        p.name.toLowerCase().includes(lowerQuery) ||
        p.description.toLowerCase().includes(lowerQuery)
    );
  }

  return {
    ...ctx,
    getProductById,
    getCategoryById,
    getListProductsByCategoryId,
    searchProductsByQuery,
  };
};

export default useListContext;
