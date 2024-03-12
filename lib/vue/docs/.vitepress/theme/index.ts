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
            "options": {
              "description": "description",
              "placeholder": "placeholder",
            },
            "updated_at": new Date(),
            "created_at": new Date(),
          },
          {
            "id": "rating1",
            "prompt": "prompt",
            "title": "Rating1",
            "type": "rating",
            "options": {
              "description": "description",
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
            "options": {
              "description": "description",
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
