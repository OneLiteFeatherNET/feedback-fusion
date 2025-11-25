# Frontend Library

::: info 
The documentation reuses the same prompt definition in the most parts. As we do not want to strip 
down all our underlying functions we use the `msw` to mock the gRPC requests made by the library.
Therefore we have to wait till the msw is up and running and longer loading times for the components 
will occur.
:::

## About

We ship a basic frontend component library based on [lit](https://lit.dev) providing
web components compliant to `openwc`

[NPM](https://www.npmjs.com/package/@onelitefeathernet/feedback-fusion)

## Quick Setup

### Node

First of all you need to install the library using a package manager of your choice:

```sh
pnpm i --save @onelitefeathernet/feedback-fusion
```

Afterwards just import the the module in order to use the `feedback-fusion-prompt` component. Here
is a basic example using Vue:

```vue 
<template>
    <feedback-fusion-prompt baseUrl="https://example.com" promptId="prompt" /> // [!code highlight]
</template>

<script setup>
import "@onelitefeathernet/feedback-fusion" // [!code highlight]
</script>
```

### Browser 

If you want to use the Feedback-Fusion frontend library in a browser context or with plain html
you can import the module via a js cdn, e.g: 

```html 
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Title</title>
    <script src="https://cdn.jsdelivr.net/npm/@onelitefeathernet/feedback-fusion@latest"></script> // [!code highlight]
</head>
<body>
    <feedback-fusion-prompt baseUrl="https://example.com" promptId="prompt" /> // [!code highlight]
</body>
</html>
```
