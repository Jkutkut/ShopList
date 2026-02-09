import { useEffect, useState } from "react";
import type { User, Uuid } from "../../types";
import userService from "../../api/versions/v1/userService";
import DateLabel from "./DateLabel";
import { asDate } from "../../utils";

interface CreatedProps {
    author: Uuid | User;
    date: string;
}

const getUserName = async (user: Uuid) => {
    const result = await userService.userInfo(user);
    if (result.isErr()) {
        console.error("Unable to get current user:", result.unwrapErr());
        return null;
    }
    return result.unwrap().data.name;
}

const Created = ({ author, date }: CreatedProps) => {
    const [isLoading, setIsLoading] = useState(true);
    const [authorName, setAuthorName] = useState<string | null>(null);

    useEffect(() => {
        if (typeof author === "string") {
            setIsLoading(true);
            getUserName(author).then((name) => {
                setAuthorName(name);
                setIsLoading(false);
            });
        }
        else {
            setAuthorName(author.name);
            setIsLoading(false);
        }
    }, [author]);
    if (isLoading) {
        return <span></span>;
    }

    return <span className="">
        Created {authorName ? `by ${authorName}` : 'at'} <DateLabel date={date} />
    </span>
};

interface UpdatedProps {
    author: Uuid | User;
    date: string;
    createdAt: string;
    printThreshold?: number;
}

const Updated = ({
    author,
    date,
    createdAt,
    printThreshold = 60000
}: UpdatedProps) => {
    const [isLoading, setIsLoading] = useState(true);
    const [authorName, setAuthorName] = useState<string | null>(null);

    useEffect(() => {
        if (typeof author === "string") {
            setIsLoading(true);
            getUserName(author).then((name) => {
                setAuthorName(name);
                setIsLoading(false);
            });
        }
        else {
            setAuthorName(author.name);
            setIsLoading(false);
        }
    }, [author]);

    if (asDate(date).getTime() - asDate(createdAt).getTime() <= printThreshold) {
        return <span></span>;
    }
    if (isLoading) {
        return <span></span>;
    }

    return <span className="">
        Updated {authorName ? `by ${authorName}` : 'at'} <DateLabel date={date} />
    </span>
};

export {
    Created,
    Updated
};
