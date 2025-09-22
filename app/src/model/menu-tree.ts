import type { JSX } from "react";

export interface MenuTree {
  isRoot?: boolean,
  helpText?: string,
  key: string,
  component: () => JSX.Element
  children?: MenuTree[]
}
