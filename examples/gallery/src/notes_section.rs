use wonopui::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NotesSectionProps {
    pub title: String,
    pub notes: Vec<String>,
}

#[function_component(NotesSection)]
pub fn notes_section(props: &NotesSectionProps) -> Html {
    html! {
        <div class="mt-8 mb-6">
            <h3 class="text-xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { &props.title }
            </h3>
            <ul class="list-disc list-inside space-y-2 text-zinc-600 dark:text-zinc-400">
                {for props.notes.iter().map(|note| {
                    html! {
                        <li>{ note }</li>
                    }
                })}
            </ul>
        </div>
    }
}
