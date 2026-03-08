use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use wonopui::*;
#[function_component(ModalDocumentation)]
pub fn modal_documentation() -> Html {
let open_small = use_state(|| false);
let open_medium = use_state(|| false);
let open_large = use_state(|| false);
let open_fullscreen = use_state(|| false);
let open_with_footer = use_state(|| false);
let open_no_close = use_state(|| false);

let toggle_small = {
let open = open_small.clone();
Callback::from(move |_: MouseEvent| open.set(!*open))
};
let close_small = {
let open = open_small.clone();
Callback::from(move |_| open.set(false))
};

let toggle_medium = {
let open = open_medium.clone();
Callback::from(move |_: MouseEvent| open.set(!*open))
};
let close_medium = {
let open = open_medium.clone();
Callback::from(move |_| open.set(false))
};

let toggle_large = {
let open = open_large.clone();
Callback::from(move |_: MouseEvent| open.set(!*open))
};
let close_large = {
let open = open_large.clone();
Callback::from(move |_| open.set(false))
};

let toggle_fullscreen = {
let open = open_fullscreen.clone();
Callback::from(move |_: MouseEvent| open.set(!*open))
};
let close_fullscreen = {
let open = open_fullscreen.clone();
Callback::from(move |_| open.set(false))
};

let toggle_footer = {
let open = open_with_footer.clone();
Callback::from(move |_: MouseEvent| open.set(!*open))
};
let close_footer = {
let open = open_with_footer.clone();
Callback::from(move |_| open.set(false))
};
let close_footer_btn = {
let open = open_with_footer.clone();
Callback::from(move |_: MouseEvent| open.set(false))
};
let confirm_footer_btn = {
let open = open_with_footer.clone();
Callback::from(move |_: MouseEvent| open.set(false))
};

let toggle_no_close = {
let open = open_no_close.clone();
Callback::from(move |_: MouseEvent| open.set(!*open))
};
let close_no_close = {
let open = open_no_close.clone();
Callback::from(move |_| open.set(false))
};
let close_no_close_btn = {
let open = open_no_close.clone();
Callback::from(move |_: MouseEvent| open.set(false))
};

html! {
<Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
<h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Modal Component" }</h1>
<p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Modal component provides a dialog overlay for focused interactions. It supports multiple sizes, backdrop click handling, keyboard navigation, and focus trapping." }</p>

<h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Size Variants" }</h2>
<ExampleCode
preview={html! {
<div class="flex gap-4 flex-wrap">
<Button onclick={toggle_small.clone()}>{"Small Modal"}</Button>
<Button onclick={toggle_medium.clone()}>{"Medium Modal"}</Button>
<Button onclick={toggle_large.clone()}>{"Large Modal"}</Button>
<Button onclick={toggle_fullscreen.clone()}>{"Full Screen"}</Button>

<Modal
open={*open_small}
on_close={close_small}
title="Small Modal"
description="This is a small modal"
size={ModalSize::Small}
>
{"Small modal content goes here."}
</Modal>

<Modal
open={*open_medium}
on_close={close_medium}
title="Medium Modal"
description="This is the default size"
size={ModalSize::Medium}
>
{"Medium modal content with more space for content."}
</Modal>

<Modal
open={*open_large}
on_close={close_large}
title="Large Modal"
description="This is a large modal"
size={ModalSize::Large}
>
{"Large modal content with plenty of room for complex layouts."}
</Modal>

<Modal
open={*open_fullscreen}
on_close={close_fullscreen}
title="Full Screen Modal"
description="This takes up the full screen"
size={ModalSize::FullScreen}
>
{"Full screen modal for maximum content area."}
</Modal>
</div>
}}
code={r#"
<Modal
open={open}
on_close={on_close}
title="Modal Title"
description="Optional description"
size={ModalSize::Medium}
>
{"Modal content"}
</Modal>"#.to_string()}
/>

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "With Footer" }</h2>
<ExampleCode
preview={html! {
<div>
<Button onclick={toggle_footer.clone()}>{"Open Modal with Footer"}</Button>

<Modal
open={*open_with_footer}
on_close={close_footer.clone()}
title="Confirm Action"
description="Are you sure you want to proceed?"
footer={html! {
<>
<Button variant={ButtonVariant::Secondary} onclick={close_footer_btn.clone()}>{"Cancel"}</Button>
<Button variant={ButtonVariant::Primary} onclick={confirm_footer_btn.clone()}>{"Confirm"}</Button>
</>
}}
>
{"This action cannot be undone. Please confirm your choice."}
</Modal>
</div>
}}
code={r#"
<Modal
open={open}
on_close={on_close}
title="Confirm Action"
footer={html! {
<>
<Button variant={ButtonVariant::Secondary} onclick={cancel}>{"Cancel"}</Button>
<Button variant={ButtonVariant::Primary} onclick={confirm}>{"Confirm"}</Button>
</>
}}
>
{"Modal content"}
</Modal>"#.to_string()}
/>

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">{ "Without Close Button" }</h2>
<ExampleCode
preview={html! {
<div>
<Button onclick={toggle_no_close.clone()}>{"Open Modal (No X Button)"}</Button>

<Modal
open={*open_no_close}
on_close={close_no_close.clone()}
title="Custom Close"
description="This modal has no X button - close via footer"
show_close_button={false}
footer={html! {
<Button variant={ButtonVariant::Primary} onclick={close_no_close_btn.clone()}>{"Close"}</Button>
}}
>
{"Use the button below to close this modal."}
</Modal>
</div>
}}
code={r#"
<Modal
open={open}
on_close={on_close}
title="Custom Close"
show_close_button={false}
footer={html! { <Button onclick={close}>{"Close"}</Button> }}
>
{"Content"}
</Modal>"#.to_string()}
/>

<Features features={vec![
"Multiple size variants (Small, Medium, Large, FullScreen)",
"Escape key to close",
"Backdrop click to close (configurable)",
"Title and description support",
"Footer slot for action buttons",
"Focus trap and keyboard navigation",
"Optional close button (X) in header",
"Smooth enter animations",
]} />

<h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
{ "API" }
</h2>

<ApiSection
title="Modal"
description="Props for the Modal component."
props={vec![
("open", "bool", "Whether the modal is open"),
("on_close", "Callback<()>", "Callback when modal requests to close"),
("title", "Option<AttrValue>", "Modal title"),
("description", "Option<AttrValue>", "Modal description/subtitle"),
("footer", "Option<Html>", "Footer content (typically buttons)"),
("size", "ModalSize", "Size variant: Small, Medium (default), Large, FullScreen"),
("close_on_backdrop", "bool", "Close when clicking backdrop (default: true)"),
("close_on_escape", "bool", "Close when pressing Escape (default: true)"),
("show_close_button", "bool", "Show X button in header (default: true)"),
("class", "Classes", "Additional CSS classes"),
]}
/>

<NotesSection
title={"Usage Notes".to_string()}
notes={vec![
"Use modals sparingly for focused interactions that require user attention.".to_string(),
"Provide clear actions in the footer for confirmation dialogs.".to_string(),
"Consider using Drawer for side panels instead of modals.".to_string(),
"The modal traps focus and closes on Escape by default.".to_string(),
"Tab navigation cycles within the modal when open.".to_string(),
]}
/>
</Container>
}
}
