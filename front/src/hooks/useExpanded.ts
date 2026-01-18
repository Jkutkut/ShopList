import { useState } from "react";

const useExpanded = (initialValue: boolean = false) => {
  const [isExpanded, setIsExpanded] = useState(initialValue);
  const toggleIsExpanded = () => setIsExpanded(!isExpanded);
  return {
    isExpanded,
    setIsExpanded,
    toggleIsExpanded,
  };
};

export default useExpanded;
