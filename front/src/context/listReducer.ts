import { USER } from "../mockup";
import { handleDndOver, handleDndStart, handleDndStop, resetDnd } from "./dnd";
import type { ListContextType } from "./ListContext";

enum ListActionType {
  SET_CATEGORIES = "SET_CATEGORIES",
  SET_LIST_PRODUCTS = "SET_LIST_PRODUCTS",
  SET_PRODUCTS = "SET_PRODUCTS",

  ADD_PRODUCT_TO_CATEGORY_LIST = "ADD_PRODUCT_TO_CATEGORY_LIST",
  REMOVE_PRODUCT_FROM_CATEGORY_LIST = "REMOVE_PRODUCT_FROM_CATEGORY_LIST",
  UPDATE_PRODUCT_LIST_AMOUNT = "UPDATE_PRODUCT_LIST_AMOUNT",
  UPDATE_PRODUCT_LIST_UNIT = "UPDATE_PRODUCT_LIST_UNIT",
};

enum DNDActionType {
  DND_START = "DND_START",
  DND_OVER = "DND_OVER",
  DND_STOP = "DND_STOP",
}

enum DndType {
  CATEGORY = "CATEGORY",
  PRODUCT = "PRODUCT",
}

type ListAction = {
  type: ListActionType.SET_CATEGORIES;
  payload: any[]; // TODO type
} | {
  type: ListActionType.SET_LIST_PRODUCTS;
  payload: any[]; // TODO type
} | {
  type: ListActionType.SET_PRODUCTS;
  payload: any[]; // TODO type
} | {
  type: ListActionType.ADD_PRODUCT_TO_CATEGORY_LIST;
  payload: {
    productId: string;
    categoryId: string;
    index: 0 | -1;
  };
} | {
  type: ListActionType.REMOVE_PRODUCT_FROM_CATEGORY_LIST;
  payload: {
    productListId: string;
  };
} | {
  type: ListActionType.UPDATE_PRODUCT_LIST_AMOUNT;
  payload: {
    id: string;
    amount: string;
  };
} | {
  type: ListActionType.UPDATE_PRODUCT_LIST_UNIT;
  payload: {
    id: string;
    unit: string;
  };
} | {
  type: DNDActionType;
  payload: { id: string };
};

const listReducer = (
  state: ListContextType,
  action: ListAction,
): ListContextType => {
  console.group("ListReducer");
  console.log("Action:", action);
  console.debug("State:", state);
  const newState = { ...state, events: { ...state.events } };
  switch (action.type) {
    case ListActionType.SET_CATEGORIES:
      newState.categories = action.payload;
      break;
    case ListActionType.SET_LIST_PRODUCTS:
      newState.listProducts = action.payload;
      break;
    case ListActionType.SET_PRODUCTS:
      newState.products = action.payload;
      break;
    case ListActionType.ADD_PRODUCT_TO_CATEGORY_LIST:
      addProductToCategoryList(newState, action.payload);
      break;
    case ListActionType.REMOVE_PRODUCT_FROM_CATEGORY_LIST:
      removeProductFromCategoryList(newState, action.payload);
      break;
    case ListActionType.UPDATE_PRODUCT_LIST_AMOUNT:
    case ListActionType.UPDATE_PRODUCT_LIST_UNIT:
      updateProductList(newState, action.payload);
      break;
    // Drag and Drop actions (DnD)
    case DNDActionType.DND_START:
      handleDndStart(newState, action.payload.id);
      break;
    case DNDActionType.DND_STOP:
      handleDndStop(newState);
      resetDnd(newState);
      break;
    case DNDActionType.DND_OVER:
      handleDndOver(newState, action.payload.id);
      break;
    default:
      console.error("Unable to handle action in ListReducer:", action);
      console.groupEnd();
      return state;
  }
  console.debug("New state:", newState);
  console.groupEnd();
  return newState;
};

const updateProductList = (state: ListContextType, {
  id,
  amount,
  unit,
}: {
  id: string;
  amount?: string;
  unit?: string;
}) => {
  const productList = state.listProducts.find((lp) => lp.id === id);
  if (!productList) {
    console.error("Unable to update product list:", id);
    return;
  }
  let modified = false;
  if (amount !== undefined) {
    productList.amount = amount === "" ? undefined : amount;
    modified = true;
  }
  if (unit !== undefined) {
    productList.unit = unit === "" ? undefined : unit;
    modified = true;
  }
  if (modified) {
    productList.updatedAt = new Date().toISOString();
  }
  else {
    console.warn("Nothing to update in product list:", id);
  }
};

const removeProductFromCategoryList = (state: ListContextType, {
  productListId,
}: {
  productListId: string;
}) => {
  const before = state.listProducts.length;
  state.listProducts = state.listProducts.filter((lp) => lp.id !== productListId);
  const after = state.listProducts.length;
  if (before <= after) {
    console.error("Unable to remove product from category list:", productListId);
  }
};

const addProductToCategoryList = (state: ListContextType, {
  productId,
  categoryId,
  index,
}: {
  productId: string;
  categoryId: string;
  index: 0 | -1;
}) => {
  // TODO categoryId may technically not exist
  let newIdx;
  if (index === 0) { // TODO this can be optimized
    state.listProducts.forEach((lp) => {
      if (lp.categoryId !== categoryId) {
        return;
      }
      lp.index += 1;
    });
    newIdx = 0;
  }
  else {
    newIdx = state.listProducts.filter((lp) => lp.categoryId === categoryId).length;
  }
  const listProduct = {
    id: `list-product-${Math.random().toString(36).substring(2, 9)}`,
    listId: state.id,
    categoryId,
    productId,
    index: newIdx,
    amount: undefined,
    unit: undefined,
    createdAt: new Date().toISOString(),
    createdBy: USER.id, // TODO who am I?
    updatedAt: new Date().toISOString(),
    updatedBy: USER.id, // TODO who am I?
  };
  state.listProducts.push(listProduct);
}

export { listReducer, ListActionType, DNDActionType, DndType };
export type { ListAction };
