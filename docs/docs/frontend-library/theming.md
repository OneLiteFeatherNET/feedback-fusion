# Theming 

## Default behavior
Feedback-Fusion defines a bunch of default theming colors via css variables:


```css
feedback-fusion-prompt {
  --feedback-fusion-text: 255, 255, 245; /* #FFFFF5 */
  --feedback-fusion-subtitle: 117, 117, 117; /* #757575 */
  --feedback-fusion-sheet: 33, 33, 33; /* #212121 */
  --feedback-fusion-primary: 52, 152, 219; /* #3498db */
  --feedback-fusion-inactive: 117, 117, 117; /* #757575 */
  --feedback-fusion-success: 76, 175, 80; /* #4caf50 */
  --feedback-fusion-error: 211, 61, 61; /* #d33d3d */
}
```

Using the default configuration your prompt should look something like this:
<feedback-fusion-prompt baseUrl="http://mock.mock" prompt="prompt" />

## Using a custom theme 
Therefore we can just overwrite these css variables in our required scope by e.g just defining
the `feedback-fusion-prompt` as a target or select it via a `class` or an `id`.

In the following example we want to change the theme of all our prompts with the class `nice-theme`
```css
.nice-theme {
  --feedback-fusion-text: 245, 245, 245; /* #F5F5F5 */
  --feedback-fusion-subtitle: 170, 170, 170; /* #AAAAAA */
  --feedback-fusion-sheet: 50, 50, 50; /* #323232 */
  --feedback-fusion-primary: 255, 152, 0; /* #FF9800 */
  --feedback-fusion-inactive: 117, 117, 117; /* #757575 */
  --feedback-fusion-success: 76, 175, 80; /* #4CAF50 */
  --feedback-fusion-error: 211, 61, 61; /* #D33D3D */
}
```

```html
<feedback-fusion-prompt baseUrl="http://mock.mock" prompt="prompt" class="nice-theme" />
```

<feedback-fusion-prompt baseUrl="http://mock.mock" prompt="prompt" class="nice-theme" />

<script setup>
import "../../../lib/dist/src/components/Prompt.js"
</script>

<style scoped>
.nice-theme {
  --feedback-fusion-text: 245, 245, 245; /* #F5F5F5 */
  --feedback-fusion-subtitle: 170, 170, 170; /* #AAAAAA */
  --feedback-fusion-sheet: 50, 50, 50; /* #323232 */
  --feedback-fusion-primary: 255, 152, 0; /* #FF9800 */
  --feedback-fusion-inactive: 117, 117, 117; /* #757575 */
  --feedback-fusion-success: 76, 175, 80; /* #4CAF50 */
  --feedback-fusion-error: 211, 61, 61; /* #D33D3D */
}
</style>
