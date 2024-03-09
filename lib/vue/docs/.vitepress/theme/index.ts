import DefaultTheme from 'vitepress/theme'
import { FeedbackFusion } from '../../../src/'
import fetchMock from "fetch-mock";
import en from "@onelitefeathernet/feedback-fusion-core/locales/en"

fetchMock
    .get("http://mock/v1/target/target/prompt/prompt", {
        status: 200,
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "id": "prompt",
            "target": "target",
            "title": "Mocked Prompt",
            "active": true,
            "updated_at": new Date(),
            "created_at": new Date()
        })
    })

export default {
  ...DefaultTheme,
  enhanceApp({ app }) {
    FeedbackFusion.install(app, {
      locales: [en],
      baseURL: "http://mock",
      target: "target"
    })
  }
}
