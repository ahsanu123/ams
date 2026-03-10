export function calculatePassword(): number {
  const now = new Date()
  // (tanggal - bulan) * tahun * bulan 
  let dateMonth = now.getDate() - now.getMonth() - 1
  const monthYear = (now.getMonth() + 1) * now.getFullYear()

  if (dateMonth === 0) dateMonth = 1
  else if (dateMonth < 0) dateMonth = Math.abs(dateMonth)

  return Math.abs(dateMonth * monthYear)
}
