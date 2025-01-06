export const required = (t: any) => (value: any) =>
  !!value || t("form.required");

export const disabled = (t: any, ...items: any[]) =>
  items.some((item: any) => typeof required(t)(item) === "string");
