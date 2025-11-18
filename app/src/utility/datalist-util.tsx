import { DataList, Text } from "@chakra-ui/react";
import type { JSX } from "react";
import type React from "react";

export const dataListItemValue = (item: string | JSX.Element, value: string | JSX.Element, additionalText?: JSX.Element) =>
  <DataList.Item>

    <Text>
      {additionalText}
    </Text>

    <DataList.ItemLabel>
      <Text textStyle={'lg'} fontWeight={'bold'}>
        {item}
      </Text>
    </DataList.ItemLabel>

    <DataList.ItemValue>
      <Text textStyle={'lg'} fontWeight={'bold'}>
        {value}
      </Text>
    </DataList.ItemValue>

  </DataList.Item>
