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
  payload: { id: string };
} | {
  type: ListActionType.DND_OVER;
  payload: { id: string };
} | {
  type: ListActionType.DND_STOP;
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
    case ListActionType.DND_START:
      const [type] = idSplit(action.payload.id);
      if (type == DndType.CATEGORY) {
        newState.events.inCategoryDnd = true;
      }
      else if (type == DndType.PRODUCT) {
        newState.events.inProductDnd = true;
      }
      newState.events.dndId = action.payload.id;
      break;
    case ListActionType.DND_STOP:
      handleDndStop(newState);
      newState.events.inCategoryDnd = false;
      newState.events.inProductDnd = false;
      newState.events.dndId = undefined;
      newState.events.dndOverId = undefined;
      break;
    case ListActionType.DND_OVER:
      newState.events.dndOverId = action.payload.id;
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

const idSplit = (fullId: string) => {
  const [meta, id] = fullId.split("_");
  const [_, type] = meta.split("-");
  return [type, id];
};

const handleDndStop = (state: ListContextType) => {
  const swap = <T>(arr: T[], fromIdx: number, toIdx: number): T[] => {
    const from = arr[fromIdx];
    const to = arr[toIdx];
    arr[fromIdx] = to;
    arr[toIdx] = from;
    return arr;
  };
  if (!state.events.dndId || !state.events.dndOverId) {
    console.warn("No dndId or dndOverId - ignoring");
    return;
  }
  if (state.events.dndId === state.events.dndOverId) {
    console.info("dndId and dndOverId are the same - ignoring");
    // TODO can happen?
    return;
  }
  const [fromType, fromId] = idSplit(state.events.dndId);
  let [toType, toId] = idSplit(state.events.dndOverId);
  if (fromType !== toType) {
    if (fromType === DndType.PRODUCT && toType === DndType.CATEGORY) {
      console.warn("TODO handle change of category");
      // TODO
      return;
    }
    else { // fromType === DndType.CATEGORY && toType === DndType.PRODUCT
      toId = state.listProducts.find((p) => p.productId === toId).categoryId;
      toType = DndType.CATEGORY;
    }
  }
  console.log(`Handling DnD stop from ${fromId} to ${toId} of type ${fromType}`, state);
  if (state.events.inCategoryDnd && fromType == DndType.CATEGORY) {
    const fromIdx = state.categories.findIndex((c) => c.id === fromId);
    const toIdx = state.categories.findIndex((c) => c.id === toId);
    if (fromIdx === -1 || toIdx === -1) {
      console.error("Unable to find category for DnD swap - ignoring");
      return;
    }
    const tmpIndex = state.categories[fromIdx].index;
    state.categories[fromIdx].index = state.categories[toIdx].index;
    state.categories[toIdx].index = tmpIndex;
    swap(state.categories, fromIdx, toIdx); // TODO needed?
    console.log(`Swapped categories ${fromId} and ${toId}`);
    state.setCategories(state.categories);
  }
  else if (state.events.inProductDnd && fromType == DndType.PRODUCT) {
    const fromIdx = state.listProducts.findIndex((p) => p.productId === fromId);
    const toIdx = state.listProducts.findIndex((p) => p.productId === toId);
    if (fromIdx === -1 || toIdx === -1) {
      console.error("Unable to find product for DnD swap - ignoring");
      return;
    }
    const tmpIndex = state.listProducts[fromIdx].index;
    state.listProducts[fromIdx].index = state.listProducts[toIdx].index;
    state.listProducts[toIdx].index = tmpIndex;
    swap(state.listProducts, fromIdx, toIdx); // TODO needed?
    console.log(`Swapped list products ${fromId} and ${toId}`);
    state.setListProducts(state.listProducts);
  }
  else {
    console.error(`Unhandled DnD type ${fromType} and inCategoryDnd ${state.events.inCategoryDnd}, inProductDnd ${state.events.inProductDnd}`, state);
    return;
  }
};

export { listReducer, ListActionType, DndType };
export type { ListAction };
