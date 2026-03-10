import { createTheme, MantineColorsTuple } from "@mantine/core";

const myColor: MantineColorsTuple = [
  '#edf3fd',
  '#d8e3f4',
  '#acc4ec',
  '#7ea3e5',
  '#5988df',
  '#4276dc',
  '#376edb',
  '#2a5dc3',
  '#2252ae',
  '#061630'
];

export const theme = createTheme({
  colors: {
    myColor,
  },
  primaryColor: 'myColor',
});
