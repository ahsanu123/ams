export function toFormData(obj: Record<string, any>): Record<string, string> {
  return Object.fromEntries(
    Object.entries(obj).map(([k, v]) => [k, v == null ? "" : JSON.stringify(v)])
  );
}
