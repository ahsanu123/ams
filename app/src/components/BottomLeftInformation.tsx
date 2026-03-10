import { Title } from "@mantine/core"
import { ReactNode } from "react"
import Clock from "./Clock"

interface BottomCornerLeftInfoProps {
  child?: ReactNode
}

export default function BottomCornerLeftInfo(props: BottomCornerLeftInfoProps) {
  return (<Clock />)
}
