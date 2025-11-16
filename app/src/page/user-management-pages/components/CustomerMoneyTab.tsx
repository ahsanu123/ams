import { Box, Button, Flex, Heading, Input, Text, Stack, Card, Avatar, DataList } from '@chakra-ui/react';
import Keyboard from 'react-simple-keyboard';
import 'react-simple-keyboard/build/css/index.css';
import Scroller from '@/component/Scroller';
import { dataListItemValue, formatAsRupiah, numberLayout, textOrNumberKeyboardDisplay } from '@/utility';

export default function CustomerMoneyTab() {


  return (
    <Stack>
      <Heading>

      </Heading>

      <Flex>
        <Scroller
          title='Uang Pelanggan'
          leftNavigation
        >
          {
            Array(10).fill(0).map(item =>
              <Card.Root>
                <Card.Header>
                  <Flex justifyContent={'space-between'}>
                    <Flex alignItems={'center'} gap={5}>
                      <Avatar.Root>
                        <Avatar.Fallback name='user name' />
                      </Avatar.Root>
                      <Text>
                        User Name
                      </Text>
                    </Flex>
                    <Button>
                      Choose
                    </Button>
                  </Flex>
                </Card.Header>

                <Card.Body>
                  <DataList.Root>
                    {dataListItemValue("Uang", `${formatAsRupiah(1900000)}`)}
                    {dataListItemValue("Hutang", `${formatAsRupiah(1900000)}`)}
                  </DataList.Root>
                </Card.Body>

              </Card.Root>
            )
          }
        </Scroller>

        <Scroller
          title='List Money History'
          leftNavigation
        >
          {
            Array(10).fill(0).map(item =>
              <Card.Root>
                <Card.Header>
                  <Flex justifyContent={'space-between'}>
                    <Flex alignItems={'center'} gap={5}>
                      <Avatar.Root>
                        <Avatar.Fallback name='user name' />
                      </Avatar.Root>
                      <Text>
                        User Name
                      </Text>
                    </Flex>
                  </Flex>
                </Card.Header>

                <Card.Body>
                  <DataList.Root>
                    {dataListItemValue("Uang", `${formatAsRupiah(1900000)}`)}
                    {dataListItemValue("Hutang", `${formatAsRupiah(1900000)}`)}
                  </DataList.Root>
                </Card.Body>

              </Card.Root>
            )
          }
        </Scroller>

        <Box>
          <Heading>
            Tambahkan Uang Ke User name
          </Heading>
          <Input
            type='number'
            size={'2xl'}
            placeholder="contoh: 90000"
            variant="subtle"
          />
          <Keyboard
            layout={numberLayout}
            display={textOrNumberKeyboardDisplay}
          />
          <Button>
            Add Money
          </Button>
        </Box>

      </Flex>

    </Stack>
  )
}
