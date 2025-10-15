import {useEffect, useMemo, useState} from "react";

interface Props<T> {
  key: string
  fetchFunc: () => Promise<T>
  expiration?: number,
  nullValueWhileLoading?: boolean
};

const DEFAULT_EXPIRATION = 3600000;
const NO_EXPIRATION = -1;

const useCachedValue = <T>({key, fetchFunc, expiration, nullValueWhileLoading}: Props<T>) => {
  nullValueWhileLoading = (nullValueWhileLoading === undefined) ? false : nullValueWhileLoading;
  const expirationMillis = useMemo(() => expiration || DEFAULT_EXPIRATION, [expiration]);
  const [value, setValue] = useState<T | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  const removeCachedValue = () => localStorage.removeItem(key);

  const loadValue = async () => {
    const value = localStorage.getItem(key);
    if (value) {
      const {time, data} = JSON.parse(value);
      const now = new Date().getTime();
      const lastUpdate = new Date(time).getTime();
      if (now - lastUpdate < expirationMillis || expirationMillis === NO_EXPIRATION) {
        setValue(data);
        setIsLoading(false);
        return;
      }
      else {
        localStorage.removeItem(key);
      }
    }
    setIsLoading(true);
    if (nullValueWhileLoading) {
      setValue(null);
    }
    console.debug(`Fetching new value for ${key}`);
    const data = await fetchFunc().catch(() => null);
    if (data !== null) {
      const nowStr = new Date().toISOString();
      const save = {time: nowStr, data};
      localStorage.setItem(key, JSON.stringify(save));
      setValue(data);
    }
    setIsLoading(false);
  };

  const flushReload = () => {
    removeCachedValue();
    loadValue();
  };

  useEffect(() => {
    loadValue();
    if (expirationMillis === NO_EXPIRATION) {
      return;
    }
    const interval = setInterval(flushReload, expirationMillis);
    return () => clearInterval(interval);
  }, []);

  return {
    value,
    isLoading,
    flush: flushReload
  };
};

export default useCachedValue;
export {NO_EXPIRATION};
