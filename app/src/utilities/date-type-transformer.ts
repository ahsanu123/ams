import { defineTransformer } from "orval";

export default defineTransformer((docs) => {
  const schemas = docs.components?.schemas;

  if (!schemas) return docs;

  for (const schemaName in schemas) {
    const schema: any = schemas[schemaName];

    if (!schema.properties) continue;

    for (const propName in schema.properties) {
      const prop = schema.properties[propName];

      if (prop.type === "string" && prop.format === "date-time") {
        prop.type = "Date";
        delete prop.format;
      }
    }
  }

  return docs;
});
