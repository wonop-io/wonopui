use wonopui::prelude::*;
use yew::prelude::*;

#[function_component(DividerDocumentation)]
pub fn divider_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "Divider Component" }</H1>
            <Paragraph>
                { "The Divider component is used to create a horizontal separation between content with an optional text label in the middle." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <p>{ "Content above the divider" }</p>
                <Divider />
                <p>{ "Content below the divider" }</p>
            </div>

            <H2>{ "With Text" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <p>{ "Content above the divider" }</p>
                <Divider text="OR" />
                <p>{ "Content below the divider" }</p>
            </div>

            <H2>{ "Custom Style" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <p>{ "Content above the divider" }</p>
                <Divider style="border-color: #3b82f6; border-width: 2px;" text="Custom Style" class="text-blue-500" />
                <p>{ "Content below the divider" }</p>
            </div>

            <H2>{ "Properties" }</H2>
            <table class="min-w-full divide-y divide-gray-200 dark:divide-zinc-700 mb-4">
                <thead class="bg-gray-50 dark:bg-zinc-800">
                    <tr>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-zinc-400 uppercase tracking-wider">{ "Name" }</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-zinc-400 uppercase tracking-wider">{ "Type" }</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-zinc-400 uppercase tracking-wider">{ "Default" }</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-zinc-400 uppercase tracking-wider">{ "Description" }</th>
                    </tr>
                </thead>
                <tbody class="bg-white dark:bg-zinc-900 divide-y divide-gray-200 dark:divide-zinc-800">
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "class" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Additional CSS classes" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "style" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Inline CSS styles" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "text" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional text to display in the middle of the divider" }</td>
                    </tr>
                </tbody>
            </table>
        </Container>
    }
}
