import DefaultTheme from "vitepress/theme";
import { http, HttpResponse } from "msw";
import { setupWorker } from "msw/browser";
import { Timestamp } from "../../../lib/src/google/protobuf/timestamp";
import { Prompt, CheckboxStyle, FieldPage, FieldType } from "../../../lib/src/feedback-fusion-v1";
import {
  BinaryWriteOptions,
  BinaryWriter,
  IBinaryWriter,
} from "@protobuf-ts/runtime";

export default {
  ...DefaultTheme,
};

// https://lucas-levin.com/code/blog/mocking-grpc-web-requests-for-integration-testing
function createGrpcResponse<Message>(
  data: Message,
  encode: (
    message: Message,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ) => IBinaryWriter,
) {
  const encoded = encode(data, new BinaryWriter(), {
    writeUnknownFields: false,
    writerFactory: () => new BinaryWriter(),
  }).finish();
  // create the data length bytes - there is probably a more concise way, but this works
  const dataLengthBytes = new Uint8Array(
    // @ts-ignore
    new Uint32Array([encoded.byteLength]).buffer,
  );
  dataLengthBytes.reverse();
  // @ts-ignore
  const dataFrame = new Uint8Array(encoded.byteLength + 5);
  dataFrame.set([0x00], 0); // set the magic byte 0x00 to identify the data frame
  dataFrame.set(dataLengthBytes, 1); // set the length bytes
  dataFrame.set(encoded, 5); // set the actual data

  // you can add mock errors by tweaking the trailers string with different status codes/messages
  const trailersString = `grpc-status: 0\r\ngrpc-message: `;
  const encoder = new TextEncoder();
  const trailers = encoder.encode(trailersString);
  const trailersLengthBytes = new Uint8Array(
    new Uint32Array([trailers.byteLength]).buffer,
  );
  trailersLengthBytes.reverse();
  const trailersFrame = new Uint8Array(trailers.byteLength + 5);
  trailersFrame.set([0x80], 0); // magic byte for trailers is 0x80
  trailersFrame.set(trailersLengthBytes, 1);
  trailersFrame.set(trailers, 5);

  // create the final body by combining the data frame and trailers frame
  const body = new Uint8Array(dataFrame.byteLength + trailersFrame.byteLength);
  body.set(dataFrame, 0);
  body.set(trailersFrame, dataFrame.byteLength);

  return HttpResponse.arrayBuffer(body.buffer, {
    status: 200,
    headers: {
      "content-type": "application/grpc-web+proto",
    },
  });
}

const handlers = [
  http.options("http://mock.mock/feedback-fusion-v1.PublicFeedbackFusionV1/*", () => {
    return HttpResponse.text(null, {
      status: 200,
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "POST",
        "Access-Control-Allow-Headers": "*",
      },
    })
  }),
  http.post(
    "http://mock.mock/feedback_fusion_v1.PublicFeedbackFusionV1/GetPrompt",
    () => {
      return createGrpcResponse(
        {
          id: "prompt",
          target: "target",
          description: "Description",
          title: "Mocked Prompt",
          active: true,
          updatedAt: Timestamp.now(),
          createdAt: Timestamp.now(),
        },
        Prompt.internalBinaryWrite,
      );
    },
  ),

  http.post(
    "http://mock.mock/feedback_fusion_v1.PublicFeedbackFusionV1/GetActiveFields",
    () => {
      return createGrpcResponse(
        {
          "pageToken": 1,
          "pageSize": 10,
          "nextPageToken": 2,
          "total": 3,
          "fields": [
            {
              "id": "text",
              "prompt": "prompt",
              "title": "Text",
              "fieldType": FieldType.TEXT,
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "text",
                  "text": {
                    "placeholder": "placeholder",
                    "lines": 1,
                  },
                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
            {
              "id": "textarea",
              "prompt": "prompt",
              "title": "Textarea",
              "fieldType": FieldType.TEXT,
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "text",
                  "text": {
                    "placeholder": "placeholder",
                    "lines": 3,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
            {
              "id": "rating",
              "prompt": "prompt",
              "title": "Rating",
              "fieldType": FieldType.RATING,
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "rating",
                  "rating": {
                    "max": 5,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
            {
              "id": "checkbox",
              "prompt": "prompt",
              "fieldType": FieldType.CHECKBOX,
              "title": "Checkbox",
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "checkbox",
                  checkbox: {
                    "defaultState": true,
                    "style": CheckboxStyle.NORMAL,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
            {
              "id": "switch",
              "prompt": "prompt",
              "fieldType": FieldType.CHECKBOX,
              "title": "Switch",
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "checkbox",
                  checkbox: {
                    "defaultState": true,
                    "style": CheckboxStyle.SWITCH,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },

            {
              "id": "selection",
              "prompt": "prompt",
              "title": "Selection",
              "fieldType": FieldType.SELECTION,
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "selection",
                  "selection": {
                    "values": ["foo", "bar"],
                    "combobox": false,
                    "multiple": true,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
            {
              "id": "combobox",
              "title": "Combobox",
              "prompt": "prompt",
              "fieldType": FieldType.SELECTION,
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "selection",
                  "selection": {
                    "values": ["foo", "bar"],
                    "combobox": true,
                    "multiple": true,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },

            {
              "id": "range",
              "title": "Range",
              "prompt": "prompt",
              "fieldType": FieldType.RANGE,
              "options": {
                "options": {
                  "oneofKind": "range",
                  "range": {
                    "min": 1,
                    "max": 10,
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
            {
              "id": "number",
              "prompt": "prompt",
              "title": "Number",
              "fieldType": FieldType.NUMBER,
              "description": "description",
              "options": {
                "options": {
                  "oneofKind": "number",
                  "number": {
                    "min": 1,
                    "max": 10,
                    "placeholder": "placeholder",
                  },

                }
              },
              "updatedAt": Timestamp.now(),
              "createdAt": Timestamp.now(),
            },
          ],
        },
        FieldPage.internalBinaryWrite,
      );
    },
  ),
];
const worker = setupWorker(...handlers);
await worker.start({ onUnhandledRequest: "bypass" });
