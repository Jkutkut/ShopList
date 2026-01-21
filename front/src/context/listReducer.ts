import { USER } from "../mockup";
import { handleDndOver, handleDndStart, handleDndStop, resetDnd } from "./dnd";
import type { ListContextType } from "./ListContext";

enum ListActionType {
  SET_CATEGORIES = "SET_CATEGORIES",
  SET_LIST_PRODUCTS = "SET_LIST_PRODUCTS",
  SET_PRODUCTS = "SET_PRODUCTS",

  ADD_PRODUCT_TO_CATEGORY_LIST = "ADD_PRODUCT_TO_CATEGORY_LIST",
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
      // TODO categoryId may technically not exist
      const { productId, categoryId, index } = action.payload;
      let newIdx;
      if (index === 0) { // TODO this can be optimized
        newState.listProducts.forEach((lp) => {
          if (lp.categoryId !== categoryId) {
            return;
          }
          lp.index += 1;
        });
        newIdx = 0;
      }
      else {
        newIdx = newState.listProducts.filter((lp) => lp.categoryId === categoryId).length;
      }
      const listProduct = {
        id: `list-product-${Math.random().toString(36).substring(2, 9)}`,
        listId: newState.id,
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
      newState.listProducts.push(listProduct);
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

export { listReducer, ListActionType, DNDActionType, DndType };
export type { ListAction };
