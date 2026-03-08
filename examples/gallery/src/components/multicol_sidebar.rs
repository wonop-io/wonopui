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
            <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 overflow-hidden mb-6">
                <div class="h-[400px] relative bg-zinc-50 dark:bg-zinc-900">
                    <MultiColumnSidebar width={400}>
                        <SidebarColumn 
                            width={Some(180)}
                            header={html! {
                                <div class="px-4 py-3 font-semibold text-sm text-zinc-900 dark:text-zinc-100 border-b border-zinc-100 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50">
                                    { "Categories" }
                                </div>
                            }}
                        >
                            <div class="p-2">
                                <SidebarMenu>
                                    <SidebarItem>
                                        <div class="flex items-center gap-3">
                                            <svg class="size-4 text-zinc-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>
                                            <span>{ "Dashboard" }</span>
                                        </div>
                                    </SidebarItem>
                                    <SidebarItem>
                                        <div class="flex items-center gap-3">
                                            <svg class="size-4 text-zinc-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"></path></svg>
                                            <span>{ "Users" }</span>
                                        </div>
                                    </SidebarItem>
                                    <SidebarItem>
                                        <div class="flex items-center gap-3">
                                            <svg class="size-4 text-zinc-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                                            <span>{ "Settings" }</span>
                                        </div>
                                    </SidebarItem>
                                </SidebarMenu>
                            </div>
                        </SidebarColumn>
                        <SidebarColumn
                            header={html! {
                                <div class="px-4 py-3 font-semibold text-sm text-zinc-900 dark:text-zinc-100 border-b border-zinc-100 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50">
                                    { "User Management" }
                                </div>
                            }}
                        >
                            <div class="p-2">
                                <SidebarMenu>
                                    <SidebarItem>{ "All Users" }</SidebarItem>
                                    <SidebarItem>{ "Groups" }</SidebarItem>
                                    <SidebarItem>{ "Permissions" }</SidebarItem>
                                    <SidebarItem>{ "Roles" }</SidebarItem>
                                </SidebarMenu>
                            </div>
                        </SidebarColumn>
                    </MultiColumnSidebar>
                    <div class="ml-[400px] p-6">
                        <div class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-2">{ "Main Content Area" }</div>
                        <p class="text-zinc-600 dark:text-zinc-400">{ "This is where your main content would appear next to the multi-column sidebar." }</p>
                    </div>
                </div>
            </div>

            <H2>{ "Three Column Layout" }</H2>
            <div class="rounded-xl border border-zinc-200 dark:border-zinc-800 overflow-hidden mb-6">
                <div class="h-[400px] relative bg-zinc-50 dark:bg-zinc-900">
                    <MultiColumnSidebar width={520}>
                        <SidebarColumn 
                            width={Some(140)}
                            header={html! {
                                <div class="px-4 py-3 font-semibold text-sm text-zinc-900 dark:text-zinc-100 border-b border-zinc-100 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50">
                                    { "Modules" }
                                </div>
                            }}
                        >
                            <div class="p-2">
                                <SidebarMenu>
                                    <SidebarItem>{ "Dashboard" }</SidebarItem>
                                    <SidebarItem>{ "Users" }</SidebarItem>
                                    <SidebarItem>{ "Analytics" }</SidebarItem>
                                </SidebarMenu>
                            </div>
                        </SidebarColumn>
                        <SidebarColumn
                            width={Some(160)}
                            header={html! {
                                <div class="px-4 py-3 font-semibold text-sm text-zinc-900 dark:text-zinc-100 border-b border-zinc-100 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50">
                                    { "Users" }
                                </div>
                            }}
                        >
                            <div class="p-2">
                                <SidebarMenu>
                                    <SidebarItem>{ "All Users" }</SidebarItem>
                                    <SidebarItem>{ "Active" }</SidebarItem>
                                    <SidebarItem>{ "Pending" }</SidebarItem>
                                </SidebarMenu>
                            </div>
                        </SidebarColumn>
                        <SidebarColumn
                            header={html! {
                                <div class="px-4 py-3 font-semibold text-sm text-zinc-900 dark:text-zinc-100 border-b border-zinc-100 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50">
                                    { "Actions" }
                                </div>
                            }}
                        >
                            <div class="p-2">
                                <SidebarMenu>
                                    <SidebarItem>{ "Create User" }</SidebarItem>
                                    <SidebarItem>{ "Import" }</SidebarItem>
                                    <SidebarItem>{ "Export" }</SidebarItem>
                                    <SidebarItem>{ "Settings" }</SidebarItem>
                                </SidebarMenu>
                            </div>
                        </SidebarColumn>
                    </MultiColumnSidebar>
                    <div class="ml-[520px] p-6">
                        <div class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-2">{ "Main Content Area" }</div>
                        <p class="text-zinc-600 dark:text-zinc-400">{ "Three-column navigation for complex hierarchies." }</p>
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