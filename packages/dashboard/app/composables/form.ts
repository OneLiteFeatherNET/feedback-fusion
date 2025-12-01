export const required = (t: any) => (value: any) =>
  !!value ||
  typeof value === "number" ||
  (Array.isArray(value) && value.length > 0) ||
  t("form.required");

export const disabled = (t: any, ...items: any[]) =>
  items.some((item: any) => typeof required(t)(item) === "string");
