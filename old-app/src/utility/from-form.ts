export async function fromFormData<T>(request: Request): Promise<T> {
  const form = await request.formData();
  const result: any = {};
  for (const [key, value] of form.entries()) {
    try {
      result[key] = JSON.parse(value as string);
    } catch {
      result[key] = value;
    }
  }
  return result as T;
}
