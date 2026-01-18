import type { CategoryContextType } from "./CategoryContext";

enum CategoryActionType {
  FOO = "FOO",
}

type CategoryAction = {
  type: CategoryActionType.FOO;
  payload: undefined;
};

const categoryReducer = (
  state: CategoryContextType,
  action: CategoryAction,
): CategoryContextType => {
  console.group("CategoryReducer");
  console.log("Action:", action);
  console.debug("State:", state);
  const newState = { ...state };
  switch (action.type) {
    default:
      console.error("Unable to handle action in CategoryReducer:", action);
      console.groupEnd();
      return state;
  }
  console.debug("New state:", newState);
  console.groupEnd();
  return newState;
};

export { categoryReducer, CategoryActionType };
export type { CategoryAction };
