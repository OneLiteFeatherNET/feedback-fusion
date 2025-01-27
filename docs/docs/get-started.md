# Get started

## Understanding the resources 

We currently have 3 different resources defined: 
- Targets
- Prompts
- Fields

### Target

A Target is supposed to contain multiple prompts to group them (for better permission management etc).
Other than that it does not really have any functionality as of now.

Maybe targets will be converted into a composite in the future. 

### Prompt

A prompt represents a prompt on your website (e.g for feedback forms) and has a single target as its parent.
You can include these prompts after the creation in your website by using the [frontend-library](/docs/frontend-library) with the
generated `promptId`.

Prompts contain the attribute `active` which defines wether clients can load this prompt when visiting your website. If you included
this prompt via the library and its inactive the library would just see the prompt is inactive and won't do any modifications to your
client website, therefore the form does not get injected.
We plan more functionalty for the `active` state in the future (e.g random prompts).

### Field

A prompt of course needs fields. Currently we support the following different field types:
- Text (also multi-line)
- Select (also combobox)
- Number
- Range
- Rating (stars)
- Checkbox (also switch)

For each of these types you can specify different validation options which will get enforced by the server and library.
