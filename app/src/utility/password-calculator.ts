export function calculatePassword(): number {
  const now = new Date()

  let dateMonth = now.getDate() - now.getMonth() - 1
  const monthYear = (now.getMonth() + 1) * now.getFullYear()

  if (dateMonth <= 0) dateMonth = 1

  return dateMonth * monthYear
}
