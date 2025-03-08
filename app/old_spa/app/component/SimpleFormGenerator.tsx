import type { JSX } from "react";

interface FormOption {
  id?: string,
  placeholder?: string,
  type?: React.HTMLInputTypeAttribute

}
export const simpleFormGenerator = <T extends object>(
  model: T,
  option?: Partial<Record<keyof T, FormOption>>
) => {

  const form: JSX.Element[] = []
  let key: keyof T;

  for (key in model) {
    form.push((
      <div
        key={key.toString()}
      >
        <label
          htmlFor={`input-${key.toString()}`}
        >
          {key.toString()}
        </label>
        <input
          id={`input-${key.toString()}`}
          name={key.toString()}
          type={option ? (option[key]?.type ?? "") : ""}
        />
      </div>
    ))
  }

  return (
    <>
      {form}
    </>
  )
}
