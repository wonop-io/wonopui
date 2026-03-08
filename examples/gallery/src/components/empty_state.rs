use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
#[function_component(EmptyStateDocumentation)]
pub fn empty_state_documentation() -> Html {
html! {
<Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
<h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "EmptyState Component" }</h1>
<p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The EmptyState component displays a placeholder when there is no data to show. Use it to guide users on what to do next." }</p>

<h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Basic Usage" }</h2>
<ExampleCode
preview={html! {
<div class="border border-zinc-200 dark:border-zinc-700 rounded-lg">
<EmptyState
title="No projects"
description="Get started by creating a new project."
icon={html! {
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-full h-full">
<path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
</svg>
}}
action={html! {
<Button variant={ButtonVariant::Primary}>
{"Create Project"}
</Button>
}}
/>
</div>
}}
code={r#"
<EmptyState
title="No projects"
description="Get started by creating a new project."
icon={html! { <FolderIcon /> }}
action={html! {
<Button variant={ButtonVariant::Primary}>
{"Create Project"}
</Button>
}}
/>"#.to_string()}
/>

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Size Variants" }</h2>
<ExampleCode
preview={html! {
<div class="space-y-6">
<div class="border border-zinc-200 dark:border-zinc-700 rounded-lg">
<EmptyState
size={EmptyStateSize::Small}
title="No items"
description="Small empty state"
/>
</div>

<div class="border border-zinc-200 dark:border-zinc-700 rounded-lg">
<EmptyState
size={EmptyStateSize::Medium}
title="No items"
description="Medium empty state (default)"
/>
</div>

<div class="border border-zinc-200 dark:border-zinc-700 rounded-lg">
<EmptyState
size={EmptyStateSize::Large}
title="No items"
description="Large empty state for prominent display"
/>
</div>
</div>
}}
code={r#"
<EmptyState size={EmptyStateSize::Small} title="No items" />
<EmptyState size={EmptyStateSize::Medium} title="No items" />
<EmptyState size={EmptyStateSize::Large} title="No items" />"#.to_string()}
/>

<Features features={vec![
"Three size variants (Small, Medium, Large)",
"Icon slot for visual context",
"Title and description text",
"Action slot for call-to-action buttons",
"Centered layout with proper spacing",
]} />

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
{ "API" }
</h2>

<ApiSection
title="EmptyState"
description="Props for the EmptyState component."
props={vec![
("title", "Option<AttrValue>", "Title text"),
("description", "Option<AttrValue>", "Description text"),
("icon", "Option<Html>", "Icon element"),
("action", "Option<Html>", "Action element (typically a button)"),
("size", "EmptyStateSize", "Size variant: Small, Medium (default), Large"),
("class", "Classes", "Additional CSS classes"),
]}
/>

<NotesSection
title={"Usage Notes".to_string()}
notes={vec![
"Use EmptyState to provide context when lists or tables have no data.".to_string(),
"Include a clear call-to-action to help users populate the empty state.".to_string(),
"Choose an appropriate icon that represents the type of content.".to_string(),
"Keep description text brief and actionable.".to_string(),
]}
/>
</Container>
}
}