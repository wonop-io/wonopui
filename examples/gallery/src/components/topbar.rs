use wonopui::*;
use yew::prelude::*;

#[function_component(TopbarDocumentation)]
pub fn topbar_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "Topbar Component" }</H1>
            <Paragraph>
                { "The Topbar component provides a fixed navigation bar at the top of your application for branding, navigation controls, and user actions." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[200px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <Topbar>
                        <Container class="flex items-center justify-between py-2" variant={ContainerVariant::Large} padding_y={false}>
                            <div class="font-semibold text-lg">{ "App Name" }</div>
                            <div class="flex items-center space-x-4">
                                <Button variant={ButtonVariant::Ghost}>{ "Dashboard" }</Button>
                                <Button variant={ButtonVariant::Ghost}>{ "Projects" }</Button>
                                <Button variant={ButtonVariant::Ghost}>{ "Team" }</Button>
                            </div>
                            <div>
                                <Button>{ "Sign In" }</Button>
                            </div>
                        </Container>
                    </Topbar>
                    <div class="pt-[60px] p-4">
                        <H3>{ "Content Area" }</H3>
                        <Paragraph>{ "This is where your main content would appear below the topbar." }</Paragraph>
                    </div>
                </div>
            </div>

            <H2>{ "With Search" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[200px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <Topbar>
                        <Container class="flex items-center justify-between py-2" variant={ContainerVariant::Large} padding_y={false}>
                            <div class="flex items-center space-x-4">
                                <div class="font-semibold text-lg">{ "App Name" }</div>
                                <div class="w-64">
                                    <Input placeholder="Search..." />
                                </div>
                            </div>
                            <div class="flex items-center space-x-2">
                                <Button variant={ButtonVariant::Ghost} class="rounded-full w-8 h-8 p-0 flex items-center justify-center">
                                    <span class="i-lucide-bell w-5 h-5"></span>
                                </Button>
                                <Button variant={ButtonVariant::Ghost} class="rounded-full w-8 h-8 p-0 flex items-center justify-center">
                                    <span class="i-lucide-settings w-5 h-5"></span>
                                </Button>
                                <div class="w-8 h-8 rounded-full bg-blue-600 text-white flex items-center justify-center">{ "JD" }</div>
                            </div>
                        </Container>
                    </Topbar>
                    <div class="pt-[60px] p-4">
                        <H3>{ "Content Area" }</H3>
                        <Paragraph>{ "This example includes a search input and user profile icons." }</Paragraph>
                    </div>
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "children" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Children" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Required" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "The content to display in the topbar" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "class" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Additional CSS classes" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "fixed" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "bool" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "true" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Whether the topbar should be fixed to the top of the viewport" }</td>
                    </tr>
                </tbody>
            </table>

            <H2>{ "Best Practices" }</H2>
            <ul class="list-disc pl-5 space-y-2 mb-4">
                <li>{ "Use the Topbar with the Layout component for proper spacing and positioning" }</li>
                <li>{ "Keep the topbar content minimal and focused on key actions" }</li>
                <li>{ "Use the Container component within the Topbar to control content width" }</li>
                <li>{ "Ensure sufficient contrast between the topbar and the content below" }</li>
            </ul>
        </Container>
    }
}
