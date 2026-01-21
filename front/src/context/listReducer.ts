import { handleDndOver, handleDndStart, handleDndStop, idSplit, resetDnd } from "./dnd";
import type { ListContextType } from "./ListContext";

enum ListActionType {
  SET_CATEGORIES = "SET_CATEGORIES",
  SET_LIST_PRODUCTS = "SET_LIST_PRODUCTS",
  SET_PRODUCTS = "SET_PRODUCTS",
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
