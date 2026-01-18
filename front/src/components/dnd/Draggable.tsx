import { useDraggable } from "@dnd-kit/core";

interface Props {
    id: string;
    className?: string;
    style?: DraggableStyleProps;
    children: React.ReactNode;
}

interface DraggableStyleProps {
    exportStyles?: boolean;
    xTranslate?: boolean;
    yTranslate?: boolean;
}

interface ComputeStyleProps {
    transform: {x: number; y: number} | null;
    xTranslate?: boolean;
    yTranslate?: boolean;
}

const computeStyle = ({
    transform,
    xTranslate,
    yTranslate,
}: ComputeStyleProps) => {
    if (!transform) return undefined;
    const transformX = xTranslate ? transform.x : 0;
    const transformY = yTranslate ? transform.y : 0;
    return {
        transform: `translate3d(${transformX}px, ${transformY}px, 0)`,
        zIndex: 9999,
    };
}

const Draggable = ({
    id,
    className,
    style: {
        exportStyles, xTranslate, yTranslate
    } = {
        exportStyles: false,
        xTranslate: true,
        yTranslate: true,
    },
    children,
}: Props) => {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id,
    });
    const style = computeStyle({transform, xTranslate, yTranslate});
    className = "dnd-draggable " + (className ? className : "full-w");
    return {
        node: <div ref={setNodeRef}
            style={exportStyles ? undefined : style}
            className={className}
            {...listeners}
            {...attributes}
        >
            {children}
        </div>,
        style: exportStyles ? style : undefined,
    };
};

export default Draggable;
