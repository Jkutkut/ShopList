import type { ListContextType } from "./ListContext";

enum ListActionType {
  SET_CATEGORIES = "SET_CATEGORIES",
  SET_LIST_PRODUCTS = "SET_LIST_PRODUCTS",
  SET_PRODUCTS = "SET_PRODUCTS",

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
  type: ListActionType.DND_START;
  payload: { type: DndType, id: string };
} | {
  type: ListActionType.DND_OVER;
  payload: { type: DndType, id: string };
} | {
  type: ListActionType.DND_STOP;
  payload: { type: DndType, id: string };
};

const listReducer = (
  state: ListContextType,
  action: ListAction,
): ListContextType => {
  console.group("ListReducer");
  console.log("Action:", action);
  console.debug("State:", state);
  const newState = { ...state };
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
    // Drag and Drop actions (DnD)
    case ListActionType.DND_START:
      if (action.payload.type === DndType.CATEGORY) {
        newState.events.inCategoryDnd = true;
      } else if (action.payload.type === DndType.PRODUCT) {
        newState.events.inProductDnd = true;
      }
      newState.events.dndId = action.payload.id;
      break;
    case ListActionType.DND_STOP:
      if (action.payload.type === DndType.CATEGORY) {
        newState.events.inCategoryDnd = false;
      } else if (action.payload.type === DndType.PRODUCT) {
        newState.events.inProductDnd = false;
      }
      newState.events.dndId = undefined;
      break;
    case ListActionType.DND_OVER:
      console.warn("DND_OVER action not implemented yet"); // TODO
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

export { listReducer, ListActionType, DndType };
export type { ListAction };
