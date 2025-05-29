use wonopui::*;
use yew::prelude::*;

#[function_component(SidebarDocumentation)]
pub fn sidebar_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "Sidebar Component" }</H1>
            <Paragraph>
                { "The Sidebar component provides a collapsible side navigation panel that can be used for application menus, navigation links, and other sidebar content." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[400px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <Sidebar
                        header={html!{
                            <div class="p-4 font-semibold text-lg">{ "App Name" }</div>
                        }}
                    >
                        <SidebarHeading>{ "Main Navigation" }</SidebarHeading>
                        <SidebarMenu>
                            <SidebarItem>
                                <div class="flex items-center space-x-2">
                                    <span class="i-lucide-home w-5 h-5"></span>
                                    <span>{ "Dashboard" }</span>
                                </div>
                            </SidebarItem>
                            <SidebarItem>
                                <div class="flex items-center space-x-2">
                                    <span class="i-lucide-users w-5 h-5"></span>
                                    <span>{ "Users" }</span>
                                </div>
                            </SidebarItem>
                            <SidebarItem>
                                <div class="flex items-center space-x-2">
                                    <span class="i-lucide-settings w-5 h-5"></span>
                                    <span>{ "Settings" }</span>
                                </div>
                            </SidebarItem>
                        </SidebarMenu>
                    </Sidebar>
                    <div class="pl-[250px] p-4">
                        <H3>{ "Main Content Area" }</H3>
                        <Paragraph>{ "This is where your main content would appear next to the sidebar." }</Paragraph>
                    </div>
                </div>
            </div>

            <H2>{ "With Footer" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[400px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <Sidebar
                        header={html!{
                            <div class="p-4 font-semibold text-lg">{ "App Name" }</div>
                        }}
                        footer={html!{
                            <div class="p-4 border-t border-gray-200 dark:border-zinc-700">
                                <div class="flex items-center space-x-2">
                                    <img src="https://via.placeholder.com/32" class="rounded-full" alt="User avatar" />
                                    <div>
                                        <div class="font-medium">{ "John Doe" }</div>
                                        <div class="text-sm text-gray-500 dark:text-zinc-400">{ "john@example.com" }</div>
                                    </div>
                                </div>
                            </div>
                        }}
                    >
                        <SidebarHeading>{ "Main Navigation" }</SidebarHeading>
                        <SidebarMenu>
                            <SidebarItem>
                                <div class="flex items-center space-x-2">
                                    <span class="i-lucide-home w-5 h-5"></span>
                                    <span>{ "Dashboard" }</span>
                                </div>
                            </SidebarItem>
                            <SidebarItem>
                                <div class="flex items-center space-x-2">
                                    <span class="i-lucide-users w-5 h-5"></span>
                                    <span>{ "Users" }</span>
                                </div>
                            </SidebarItem>
                        </SidebarMenu>
                    </Sidebar>
                    <div class="pl-[250px] p-4">
                        <H3>{ "Main Content Area" }</H3>
                        <Paragraph>{ "This example includes a footer section with user profile." }</Paragraph>
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "The content to display in the sidebar" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "header" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Html>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional content to display in the sidebar header" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "footer" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Html>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional content to display in the sidebar footer" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "folded" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "bool" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "false" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Whether the sidebar is collapsed" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "class" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Additional CSS classes" }</td>
                    </tr>
                </tbody>
            </table>

            <H2>{ "Related Components" }</H2>
            <ul class="list-disc pl-5 space-y-2 mb-4">
                <li><strong>{ "SidebarHeading" }</strong>{ " - Used to create section headings in the sidebar" }</li>
                <li><strong>{ "SidebarMenu" }</strong>{ " - Container for sidebar menu items" }</li>
                <li><strong>{ "SidebarItem" }</strong>{ " - Individual navigation item in the sidebar" }</li>
                <li><strong>{ "SidebarLink" }</strong>{ " - Navigation link that supports Yew Router integration" }</li>
            </ul>
        </Container>
    }
}
