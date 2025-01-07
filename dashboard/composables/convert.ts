import { FieldType } from "./feedback-fusion-v1";

export const numberToKind = (value: number) => {
  switch (value) {
    case FieldType.TEXT:
      return "text";
    case FieldType.NUMBER:
      return "number";
    case FieldType.SELECTION:
      return "selection";
    case FieldType.RANGE:
      return "range";
    case FieldType.CHECKBOX:
      return "checkbox";
    case FieldType.RATING:
      return "rating";
    default:
      return undefined;
  }
};
