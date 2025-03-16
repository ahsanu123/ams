import { getRandomArbitrary } from "./random-arbitrary"

const clockAnimation = [
  "🕙", "🕜", "🕥",
  "🕦", "🕚", "🕛",
  "🕑", "🕧", "🕝",
  "🕞", "🕒", "🕓",
  "🕔", "🕟", "🕠",
  "🕡", "🕕", "🕖",
  "🕗", "🕢", "🕣",
  "🕤", "🕘",
]
export function displayAsClock(date: Date) {

  const dateString = date.toLocaleDateString("id-id", {
    year: "numeric",
    month: "long",
    day: "numeric",
  })
  const dateClock = date.toLocaleTimeString("en-Us", {
    hour12: false
  })

  const clockCharacter = parseInt(getRandomArbitrary(0, clockAnimation.length - 1).toFixed())

  const result = `${clockAnimation[clockCharacter]} ${dateString} - ${dateClock}`

  return result
}
