use wonopui::prelude::*;
use yew::prelude::*;

#[function_component(CopyButtonDocumentation)]
pub fn copy_button_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "CopyButton Component" }</H1>
            <Paragraph>
                { "The CopyButton component provides an easy way to copy text content to the clipboard with visual feedback for the user." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="flex items-center space-x-2">
                    <code class="bg-gray-100 dark:bg-zinc-700 p-2 rounded">{ "npm install wonopui" }</code>
                    <CopyButton copy_text="npm install wonopui" />
                </div>
            </div>

            <H2>{ "With Custom Children" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="flex items-center space-x-2">
                    <code class="bg-gray-100 dark:bg-zinc-700 p-2 rounded">{ "yarn add wonopui" }</code>
                    <CopyButton copy_text="yarn add wonopui">
                        <Button variant={ButtonVariant::Secondary} size={ButtonSize::Small}>
                            { "Copy command" }
                        </Button>
                    </CopyButton>
                </div>
            </div>

            <H2>{ "With Success Text" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="flex items-center space-x-2">
                    <code class="bg-gray-100 dark:bg-zinc-700 p-2 rounded">{ "pnpm add wonopui" }</code>
                    <CopyButton
                        copy_text="pnpm add wonopui"
                        copied_text={Some("Copied to clipboard!".to_string())}
                    />
                </div>
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "text" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Required" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "The text to copy to the clipboard" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "success_text" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<String>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional text to display when copy is successful" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "children" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Html>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None (uses default icon)" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional custom element to display instead of the default copy icon" }</td>
                    </tr>
                </tbody>
            </table>
        </Container>
    }
}
