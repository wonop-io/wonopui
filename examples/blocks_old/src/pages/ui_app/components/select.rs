use super::example_code::ExampleCode;
use wonopui::*;
use yew::prelude::*;

#[function_component(SelectDocumentation)]
pub fn select_documentation() -> Html {
    html! {
        <div>{"TODO"}</div>
    }
    /*
        html! {
            <div class="p-6 bg-white dark:bg-zinc-900 min-h-screen">
                <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">{ "Select Component" }</h1>
                <p class="mb-6 text-zinc-600 dark:text-zinc-400">{ "The Select component is a versatile dropdown component that allows users to select an option from a list. It is composed of several subcomponents to provide a flexible and customizable experience." }</p>

                <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "Example" }</h2>
                <ExampleCode
                    preview={html! {
                            <Select>
                                <SelectTrigger class="w-[180px]" placeholder="Select a fruit" />
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectLabel label="Fruits" />
                                        <SelectItem value="apple" label="Apple" />
                                        <SelectItem value="banana" label="Banana" />
                                        <SelectItem value="blueberry" label="Blueberry" />
                                        <SelectItem value="grapes" label="Grapes" />
                                        <SelectItem value="pineapple" label="Pineapple" />
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                    }}
                    code={r#"
    <div class=\"mb-6 p-4 bg-zinc-50 dark:bg-zinc-800 rounded shadow\">
        <Select>
            <SelectTrigger class=\"w-[180px]\" placeholder=\"Select a fruit\" />
            <SelectContent>
                <SelectGroup>
                    <SelectLabel label=\"Fruits\" />
                    <SelectItem value=\"apple\" label=\"Apple\" />
                    <SelectItem value=\"banana\" label=\"Banana\" />
                    <SelectItem value=\"blueberry\" label=\"Blueberry\" />
                    <SelectItem value=\"grapes\" label=\"Grapes\" />
                    <SelectItem value=\"pineapple\" label=\"Pineapple\" />
                </SelectGroup>
            </SelectContent>
        </Select>
    </div>"#.to_string()}
                />

                <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">{ "API" }</h2>
                <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "Select" }</h3>
                <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The main container for the select component." }</p>
                <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                    <li>{ "children: Children - The child elements to be rendered inside the select component." }</li>
                </ul>

                <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "SelectTrigger" }</h3>
                <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The button that triggers the opening and closing of the select dropdown." }</p>
                <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                    <li>{ "class: String - Additional CSS classes for styling the trigger button." }</li>
                    <li>{ "placeholder: String - The placeholder text to be displayed when no option is selected." }</li>
                </ul>

                <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "SelectContent" }</h3>
                <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "The container for the dropdown content." }</p>
                <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                    <li>{ "children: Children - The child elements to be rendered inside the dropdown content." }</li>
                </ul>

                <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "SelectGroup" }</h3>
                <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "A group of related options within the dropdown." }</p>
                <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                    <li>{ "children: Children - The child elements to be rendered inside the group." }</li>
                </ul>

                <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "SelectLabel" }</h3>
                <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "A label for a group of options." }</p>
                <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                    <li>{ "label: String - The text to be displayed as the label." }</li>
                </ul>

                <h3 class="text-xl font-semibold mb-2 text-zinc-900 dark:text-white">{ "SelectItem" }</h3>
                <p class="mb-2 text-zinc-600 dark:text-zinc-400">{ "An individual option within the dropdown." }</p>
                <ul class="list-disc list-inside mb-4 text-zinc-600 dark:text-zinc-400">
                    <li>{ "value: String - The value of the option." }</li>
                    <li>{ "label: String - The text to be displayed for the option." }</li>
                </ul>
            </div>
        }
        */
}
