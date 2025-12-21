import type { DregPriceModel } from "@/api-models";
import { dregPriceCommand } from "@/commands";
import Scroller from "@/component/Scroller";
import { formatAsRupiah, formatDateId, numberLayout, textOrNumberKeyboardDisplay, toaster } from "@/utility";
import { Button, Flex, Heading, NumberInput, Stack, Table } from "@chakra-ui/react";
import { useEffect, useRef, useState } from "react";
import Keyboard, { type SimpleKeyboard } from "react-simple-keyboard";

export default function UpdateDregPriceTab() {

  const keyboardRef = useRef<SimpleKeyboard>(null)
  const [newDregPrice, setNewDregPrice] = useState<number>(0)
  const [prices, setPrices] = useState<DregPriceModel[]>([])

  const loadDregPrices = () => {
    dregPriceCommand.getAllDregPrice()
      .then((listPrices) => setPrices(listPrices))
  }

  const handleOnUpdateDregPrice = () => {
    if (newDregPrice <= 0) {
      toaster.create({
        title: 'Harga Tidak Boleh Kurang Dari nol (0)',
        type: 'error'
      })
      return;
    }

    dregPriceCommand.updateDregPrice(newDregPrice)
      .then(_ => loadDregPrices())

    keyboardRef.current?.setInput('0')
    setNewDregPrice(0)
  }

  useEffect(() => {
    loadDregPrices()
  }, [])

  const dregPrices = () =>
    <Scroller>
      <Table.Root stickyHeader>
        <Table.Header>
          <Table.Row>
            <Table.ColumnHeader>
              Tanggal
            </Table.ColumnHeader>
            <Table.ColumnHeader>
              Harga
            </Table.ColumnHeader>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {
            prices.map(price =>
              <Table.Row>
                <Table.Cell>
                  {formatDateId(price.createdDate)}
                </Table.Cell>

                <Table.Cell>
                  {formatAsRupiah(price.price)}
                </Table.Cell>
              </Table.Row>
            )
          }
        </Table.Body>
      </Table.Root>
    </Scroller>

  return (
    <Stack>
      <Heading>
        Harga Baru
      </Heading>

      <Flex>
        <Stack>
          <NumberInput.Root
            defaultValue="0"
            value={newDregPrice.toString()}
            formatOptions={{
              style: "currency",
              currency: "IDR",
            }}
          >
            <NumberInput.Control />
            <NumberInput.Input fontSize={'2xl'} />
          </NumberInput.Root>
          <Keyboard
            keyboardRef={(kb) => keyboardRef.current = kb as SimpleKeyboard}
            layout={numberLayout}
            display={textOrNumberKeyboardDisplay}
            onChange={(value) => setNewDregPrice(Number(value))}
            onKeyPress={() => undefined}
          />
          <Button
            height={'80px'}
            fontSize={'2xl'}
            onClick={() => handleOnUpdateDregPrice()}
          >
            Ganti Harga
          </Button>
        </Stack>

        <Stack>
          {dregPrices()}
        </Stack>
      </Flex>
    </Stack>
  )
}
