import {useState} from "react";

const useForm = <T>(initialValues: T) => {
  const [formState, setFormState] = useState(initialValues);

  const onChange = ({target}: any) => {
    const { name, value } = target;
    setFormState({
      ...formState,
      [name]: value
    });
  };
  return {
    ...formState,
    formState,
    onChange
  } as T & { onChange: (e: any) => void; formState: Record<string, any> };
};

export default useForm;
