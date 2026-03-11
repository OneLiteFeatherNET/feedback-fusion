import { ProtoFieldType } from "./feedback-fusion-v1/field.js";

export const numberToKind = (value: number) => {
  switch (value) {
    case ProtoFieldType.TEXT:
      return "text";
    case ProtoFieldType.NUMBER:
      return "number";
    case ProtoFieldType.SELECTION:
      return "selection";
    case ProtoFieldType.RANGE:
      return "range";
    case ProtoFieldType.CHECKBOX:
      return "checkbox";
    case ProtoFieldType.RATING:
      return "rating";
    default:
      return undefined;
  }
};
