use wonopui::prelude::*;
use wonopui::*;
use yew::prelude::*;

#[function_component(MulticolSidebarDocumentation)]
pub fn multicol_sidebar_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "MultiColumnSidebar Component" }</H1>
            <Paragraph>
                { "The MultiColumnSidebar component provides a multi-column sidebar layout for complex navigation structures, making it ideal for applications with multiple levels of navigation hierarchy." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[400px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <MultiColumnSidebar>
                        <SidebarColumn>
                            <div class="h-full flex flex-col">
                                <div class="p-4 font-semibold text-lg border-b border-gray-200 dark:border-zinc-700">{ "Categories" }</div>
                                <div class="flex-1 overflow-y-auto p-2">
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
                                                <span>{ "User Management" }</span>
                                            </div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div class="flex items-center space-x-2">
                                                <span class="i-lucide-settings w-5 h-5"></span>
                                                <span>{ "Settings" }</span>
                                            </div>
                                        </SidebarItem>
                                    </SidebarMenu>
                                </div>
                            </div>
                        </SidebarColumn>
                        <SidebarColumn>
                            <div class="h-full flex flex-col">
                                <div class="p-4 font-semibold text-lg border-b border-gray-200 dark:border-zinc-700">{ "User Management" }</div>
                                <div class="flex-1 overflow-y-auto p-2">
                                    <SidebarMenu>
                                        <SidebarItem>
                                            <div>{ "Users" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "Groups" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "Permissions" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "Roles" }</div>
                                        </SidebarItem>
                                    </SidebarMenu>
                                </div>
                            </div>
                        </SidebarColumn>
                    </MultiColumnSidebar>
                    <div class="pl-[500px] p-4">
                        <H3>{ "Main Content Area" }</H3>
                        <Paragraph>{ "This is where your main content would appear next to the multi-column sidebar." }</Paragraph>
                    </div>
                </div>
            </div>

            <H2>{ "With Tertiary Column" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[400px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <MultiColumnSidebar>
                        <SidebarColumn>
                            <div class="h-full flex flex-col">
                                <div class="p-4 font-semibold text-lg border-b border-gray-200 dark:border-zinc-700">{ "Categories" }</div>
                                <div class="flex-1 overflow-y-auto p-2">
                                    <SidebarMenu>
                                        <SidebarItem>
                                            <div>{ "Dashboard" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "User Management" }</div>
                                        </SidebarItem>
                                    </SidebarMenu>
                                </div>
                            </div>
                        </SidebarColumn>
                        <SidebarColumn>
                            <div class="h-full flex flex-col">
                                <div class="p-4 font-semibold text-lg border-b border-gray-200 dark:border-zinc-700">{ "User Management" }</div>
                                <div class="flex-1 overflow-y-auto p-2">
                                    <SidebarMenu>
                                        <SidebarItem>
                                            <div>{ "Users" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "Groups" }</div>
                                        </SidebarItem>
                                    </SidebarMenu>
                                </div>
                            </div>
                        </SidebarColumn>
                        <SidebarColumn>
                            <div class="h-full flex flex-col">
                                <div class="p-4 font-semibold text-lg border-b border-gray-200 dark:border-zinc-700">{ "Users" }</div>
                                <div class="flex-1 overflow-y-auto p-2">
                                    <SidebarMenu>
                                        <SidebarItem>
                                            <div>{ "Create User" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "Edit Users" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "Delete Users" }</div>
                                        </SidebarItem>
                                        <SidebarItem>
                                            <div>{ "User Activity" }</div>
                                        </SidebarItem>
                                    </SidebarMenu>
                                </div>
                            </div>
                        </SidebarColumn>
                    </MultiColumnSidebar>
                    <div class="pl-[750px] p-4">
                        <H3>{ "Main Content Area" }</H3>
                        <Paragraph>{ "This example includes a third level of navigation with tertiary_column." }</Paragraph>
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Vec<SidebarColumn>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Required" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Content for the sidebar columns" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "curtain_content" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Html" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Default empty" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Content for the curtain/backdrop when sidebar is open in mobile view" }</td>
                    </tr>
                </tbody>
            </table>

            <H2>{ "SidebarColumn Properties" }</H2>
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Content for the column" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "width" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<i32>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Fixed width in pixels" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "hide_when_folded" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "bool" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "false" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Whether to hide the column when sidebar is folded" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "header" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Html>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional header content" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "footer" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Html>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional footer content" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "class" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Classes" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Default" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Additional CSS classes" }</td>
                    </tr>
                </tbody>
            </table>

            <H2>{ "Usage Guidelines" }</H2>
            <ul class="list-disc pl-5 space-y-2 mb-4">
                <li>{ "Use MultiColumnSidebar for applications with complex navigation hierarchies" }</li>
                <li>{ "Keep column content organized with clear headers and logical grouping" }</li>
                <li>{ "Consider how many columns are necessary for your navigation needs" }</li>
                <li>{ "Ensure each column has a distinct purpose and provides clear navigation contexts" }</li>
            </ul>
        </Container>
    }
}