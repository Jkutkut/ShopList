import { useState } from "react";

const useExpanded = (initialValue: boolean = false) => {
  const [expanded, setExpanded] = useState(initialValue);
  const toggleExpanded = () => setExpanded(!expanded);
  return {
    expanded,
    setExpanded,
    toggleExpanded,
  };
};

export default useExpanded;
