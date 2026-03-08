use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
#[function_component(FormFieldDocumentation)]
pub fn form_field_documentation() -> Html {
html! {
<Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
<h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "FormField Component" }</h1>
<p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The FormField component wraps form inputs with labels, descriptions, and error messages for consistent form layouts." }</p>

<h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Basic Usage" }</h2>
<ExampleCode
preview={html! {
<div class="space-y-4 max-w-md">
<FormField label="Email" description="We will never share your email.">
<Input placeholder="you@example.com" />
</FormField>

<FormField label="Password" required={true}>
<Input kind="password" placeholder="Enter password" />
</FormField>
</div>
}}
code={r#"
<FormField label="Email" description="We will never share your email.">
<Input placeholder="you@example.com" />
</FormField>

<FormField label="Password" required={true}>
<Input kind="password" placeholder="Enter password" />
</FormField>"#.to_string()}
/>

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Error State" }</h2>
<ExampleCode
preview={html! {
<div class="space-y-4 max-w-md">
<FormField 
label="Username" 
error="Username is already taken"
required={true}
>
<Input placeholder="johndoe" class="border-red-500" />
</FormField>

<FormField 
label="Email" 
error="Please enter a valid email address"
>
<Input placeholder="you@example.com" class="border-red-500" />
</FormField>
</div>
}}
code={r#"
<FormField 
label="Username" 
error="Username is already taken"
required={true}
>
<Input placeholder="johndoe" />
</FormField>"#.to_string()}
/>

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "With Description" }</h2>
<ExampleCode
preview={html! {
<div class="space-y-4 max-w-md">
<FormField 
label="Bio" 
description="Write a short bio about yourself. Max 200 characters."
>
<Textarea placeholder="Tell us about yourself..." />
</FormField>
</div>
}}
code={r#"
<FormField 
label="Bio" 
description="Write a short bio about yourself. Max 200 characters."
>
<Textarea placeholder="Tell us about yourself..." />
</FormField>"#.to_string()}
/>

<Features features={vec![
"Label with optional required indicator",
"Description text support",
"Error message display",
"Accessible form structure",
"Works with any input component",
]} />

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
{ "API" }
</h2>

<ApiSection
title="FormField"
description="Props for the FormField component."
props={vec![
("label", "Option<AttrValue>", "Label text for the field"),
("description", "Option<AttrValue>", "Description text shown below label"),
("error", "Option<AttrValue>", "Error message shown below input"),
("required", "bool", "Shows required indicator (*) on label"),
("id", "Option<AttrValue>", "Field ID for accessibility"),
("class", "Classes", "Additional CSS classes"),
]}
/>

<NotesSection
title={"Usage Notes".to_string()}
notes={vec![
"Use FormField to create consistent form layouts across your application.".to_string(),
"Error messages turn the label red to indicate invalid fields.".to_string(),
"Always provide descriptive labels for accessibility.".to_string(),
"Use the id prop to connect labels with inputs.".to_string(),
]}
/>
</Container>
}
}