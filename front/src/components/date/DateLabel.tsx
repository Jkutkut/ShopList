import { useEffect, useState } from "react";

const DEFAULT_REFRESH_INTERVAL = 60000;
const NO_REFRESH_INTERVAL = -1;

interface Props {
    date: Date | string | number;
    refreshInterval?: number;
};

const date2str = (date: Date) => {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    if (diff < 60000) { // < 1 min
        return "just now";
    }
    if (diff < 1800000) { // < 30 min
        const minutes = Math.floor(diff / 60000);
        if (minutes === 1) {
            return "1 minute ago";
        }
        return `${minutes} minutes ago`;
    }
    const time = date.toLocaleTimeString();
    if (diff < 86400000 && date.getDay() === now.getDay()) { // < 1 day
        return time;
    }
    const day = date.toLocaleDateString();
    return `${time} ${day}`;
};

const DateLabel = ({
    date,
    refreshInterval = DEFAULT_REFRESH_INTERVAL
}: Props) => {
    const dateObj = (typeof date === "object") ? date : new Date(date);
    const dateStr = dateObj.toISOString();
    const [dateLabel, setDayLabel] = useState(date2str(dateObj));

    useEffect(() => {
        setDayLabel(date2str(dateObj));
    }, [date]);

    useEffect(() => {
        if (refreshInterval === NO_REFRESH_INTERVAL) {
            return;
        }
        const interval = setInterval(() => {
            setDayLabel(date2str(dateObj));
        }, refreshInterval);
        return () => clearInterval(interval);
    }, []);
    return (
        <time className="date2format" data-date={dateStr}>
            {dateLabel}
        </time>
    );
};

export default DateLabel;
