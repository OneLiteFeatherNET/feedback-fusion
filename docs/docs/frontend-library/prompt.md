# Prompt

## Usage

```html 
<feedback-fusion-prompt baseUrl="https://example.com" promptId="prompt" />
```

<feedback-fusion-prompt baseUrl="https://mock.mock" promptId="prompt" />

# Attributes 

| Attribute   | Type    | Default  | Description                                                                                                               | Required |
|-------------|---------|----------|---------------------------------------------------------------------------------------------------------------------------|----------|
| `autoClose` | Boolean | `false`  | Determines whether the prompt should automatically close after form submission.                                           | No       |
| `baseUrl`   | String  |          | The base URL where the backend is proxied via gRPC-web.                                                                   | Yes      |
| `closeAfter`| Number  | `1000`   | Specifies the duration (in milliseconds) after which the prompt should close automatically if `autoClose` is set to true. | No       |
| `locale`    | String  | `en`     | Defines the locale for the prompt.                                                                                        | No       |
| `promptId`  | String  |          | Specifies which prompt should be used.                                                                                    | Yes      |

<script setup>
import "../../../lib/dist/src/components/Prompt.js";
</script>
