import type { ListContextType } from "./ListContext";
import { DndType } from "./listReducer";

const idSplit = (fullId: string) => {
  const [meta, id] = fullId.split("_");
  const [_, type] = meta.split("-");
  return [type, id];
};

const handleDndStart = (state: ListContextType, id: string) => {
  const [type] = idSplit(id);
  if (type == DndType.CATEGORY) {
    state.events.inCategoryDnd = true;
  }
  else if (type == DndType.PRODUCT) {
    state.events.inProductDnd = true;
  }
  state.events.dndId = id;
};

const handleDndOver = (state: ListContextType, id: string) => {
  state.events.dndOverId = id;
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
  console.log(state.events)
  const [fromType, fromId] = idSplit(state.events.dndId);
  let [toType, toId] = idSplit(state.events.dndOverId);
  if (fromId === toId) {
    console.info("dndId and dndOverId are the same - ignoring");
    return;
  }
  if (fromType !== toType) {
    if (fromType === DndType.PRODUCT && toType === DndType.CATEGORY) {
      console.warn("TODO handle change of category");
      // TODO
      // ? can happen when moving from the same category to the start or end of the category
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

const resetDnd = (state: ListContextType) => {
  state.events.inCategoryDnd = false;
  state.events.inProductDnd = false;
  state.events.dndId = undefined;
  state.events.dndOverId = undefined;
};

export { idSplit, handleDndStart, handleDndStop, handleDndOver, resetDnd };
