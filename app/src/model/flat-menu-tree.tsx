import type { JSX } from "react";

export interface FlatMenuTree {
  path: string,
  title: string,
  description?: string,
  groupName?: string,
  component?: JSX.Element
}
