import { useDroppable } from "@dnd-kit/core";

interface Props {
    id: string;
    className?: string;
    style?: React.CSSProperties;
    children?: React.ReactNode;
}

const Droppable = ({
    id,
    className,
    style,
    children,
}: Props) => {
    const {setNodeRef} = useDroppable({
        id,
    });
    style = style ? style : undefined;
    return <div ref={setNodeRef} className={className} style={style}>
        {children}
    </div>;
};

export default Droppable;
