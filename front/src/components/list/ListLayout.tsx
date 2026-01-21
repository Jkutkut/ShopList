import { DndContext } from "@dnd-kit/core";
import { ACTION } from "../../mockup";
import Category from "./Category";
import { useContext } from "react";
import { ListContext, ListContextProvider } from "../../context/ListContext";
import { DNDActionType } from "../../context/listReducer";

interface Props {
    id: string;
}

const ListLayout = ({
    id,
}: Props) => {
    return <ListContextProvider id={id}>
        <List />
    </ListContextProvider>;
};

const List = () => {
    const { categories, listProducts, dispatch } = useContext(ListContext);

    return <section className="list-layout col gap">
        <DndContext
            onDragStart={(e) => dispatch({
                type: DNDActionType.DND_START,
                payload: { id: `${e.active.id}` },
            })}
            onDragEnd={(e) => dispatch({
                type: DNDActionType.DND_STOP,
                payload: { id: `${e.active.id}` },
            })}
            onDragOver={(e) => dispatch({
                type: DNDActionType.DND_OVER,
                payload: { id: `${e.over?.id}` },
            })}
            onDragAbort={() => console.log("Drag aborted list")}
            onDragCancel={() => console.log("Drag cancelled list")}
        >
            <a className="btn btn-primary" onClick={ACTION("new category")}>Create new category</a>
            <div className="categories col gap">
                {categories.sort((a, b) => a.index - b.index).map((c) => (
                    <Category
                        key={c.id}
                        category={c}
                    />
                ))}
                <Category
                    category={undefined}
                />
            </div>
            <a className="btn btn-primary" onClick={ACTION("new category")}>Create new category</a>
        </DndContext>
    </section>;
};

export default ListLayout;
