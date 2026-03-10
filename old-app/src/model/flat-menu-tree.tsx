import type { JSX } from "react";

export interface FlatMenuTree {
  path: string,
  title: string,
  groupName: string,
  isRoot: boolean,
  description?: string,
  component?: JSX.Element
}
