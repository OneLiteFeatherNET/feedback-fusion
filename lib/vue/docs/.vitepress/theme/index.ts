import DefaultTheme from "vitepress/theme";
import { FeedbackFusion } from "../../../src/";
import fetchMock from "fetch-mock";
import en from "@onelitefeathernet/feedback-fusion-core/locales/en";

fetchMock
  .get("http://mock/v1/target/target/prompt/prompt", {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      "id": "prompt",
      "target": "target",
      "title": "Mocked Prompt",
      "active": true,
      "updated_at": new Date(),
      "created_at": new Date(),
    }),
  });

fetchMock
  .get("begin:http://mock/v1/target/target/prompt/prompt/fetch", {
    status: 200,
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(
      {
        "page_no": 1,
        "total": 3,
        "records": [
          {
            "id": "text1",
            "prompt": "prompt",
            "title": "Text1",
            "type": "text",
            "description": "description",
            "options": {
              "placeholder": "placeholder",
              "lines": 1
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "textarea",
            "prompt": "prompt",
            "title": "Textarea",
            "type": "text",
            "description": "description",
            "options": {
              "placeholder": "placeholder",
              "lines": 3,
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "rating1",
            "prompt": "prompt",
            "title": "Rating1",
            "type": "rating",
            "description": "description",
            "options": {
              "max": 5,
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "text2",
            "prompt": "prompt",
            "title": "Text2",
            "type": "text",
            "description": "description",
            "options": {
              "placeholder": "placeholder",
              "lines": 1,
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "checkbox",
            "prompt": "prompt",
            "type": "checkbox",
            "title": "Checkbox",
            "description": "description",
            "options": {
              "defaultState": true, 
              "style": "checkbox",
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "switch",
            "prompt": "prompt",
            "type": "checkbox",
            "title": "Switch",
            "description": "description",
            "options": {
              "defaultState": false, 
              "style": "switch",
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },

          {
            "id": "selection",
            "prompt": "prompt",
            "title": "Selection",
            "type": "selection",
            "description": "description",
            "options": {
              "values": ["foo", "bar"],
              "combobox": false,
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "combobox",
            "title": "Combobox",
            "prompt": "prompt",
            "type": "selection",
            "description": "description",
            "options": {
              "values": ["foo", "bar"],
              "combobox": true,
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },

          {
            "id": "range",
            "title": "Range",
            "prompt": "prompt",
            "type": "range",
            "options": {
              "min": 1,
              "max": 10,
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "number",
            "prompt": "prompt",
            "title": "Number",
            "type": "number",
            "description": "description",
            "options": {
              "min": 1,
              "max": 10,
              "placeholder": "placeholder",
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
        ],
      },
    ),
  });

export default {
  ...DefaultTheme,
  enhanceApp({ app }) {
    app.use(FeedbackFusion, {
      locales: [en],
      baseURL: "http://mock",
      target: "target",
    });
  },
};
