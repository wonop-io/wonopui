use wonopui::{*, prelude::*};
use yew::prelude::*;

#[function_component(PageContentDocumentation)]
pub fn page_content_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "PageContent Component" }</H1>
            <Paragraph>
                { "The PageContent component provides a standardized container for the main content area of your application, with support for paddings, backgrounds, and borders." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[300px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <PageContent>
                        <H3>{ "Page Title" }</H3>
                        <Paragraph>{ "This is an example of basic page content with default styling. It provides proper spacing and a clean background." }</Paragraph>
                        <Button class="mt-4">{ "Action Button" }</Button>
                    </PageContent>
                </div>
            </div>

            <H2>{ "With Custom Styling" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[300px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <MainContent class="bg-blue-50 dark:bg-blue-950 border-blue-100 dark:border-blue-900">
                        <PageContent>
                        <H3>{ "Custom Styled Content" }</H3>
                        <Paragraph>{ "This example demonstrates customizing the PageContent with a different background color and border style." }</Paragraph>
                        <Alert alert_type={AlertType::Info} class="mt-4">
                            <AlertTitle>{ "Information" }</AlertTitle>
                            <AlertDescription>
                                { "You can apply custom classes to change the appearance of the PageContent component." }
                            </AlertDescription>
                        </Alert>
                        </PageContent>
                    </MainContent>
                </div>
            </div>

            <H2>{ "With Header and Footer" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <div class="h-[400px] relative overflow-hidden border border-gray-200 dark:border-zinc-700 rounded-md">
                    <PageContent>
                        <div class="border-b border-gray-200 dark:border-zinc-700 pb-4 mb-4 flex justify-between items-center">
                            <H3 class="m-0">{ "Page With Header and Footer" }</H3>
                            <div class="flex space-x-2">
                                <Button variant={ButtonVariant::Secondary} size={ButtonSize::Small}>{ "Cancel" }</Button>
                                <Button size={ButtonSize::Small}>{ "Save" }</Button>
                            </div>
                        </div>

                        <div class="min-h-[250px]">
                            <Paragraph>{ "This is the main content area of the page." }</Paragraph>
                            <Paragraph>{ "You can structure your page with distinct header and footer sections." }</Paragraph>
                        </div>

                        <div class="border-t border-gray-200 dark:border-zinc-700 pt-4 mt-4 flex justify-end">
                            <div class="flex space-x-2">
                                <Button variant={ButtonVariant::Secondary}>{ "Cancel" }</Button>
                                <Button>{ "Save Changes" }</Button>
                            </div>
                        </div>
                    </PageContent>
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "The content to display within the page content area" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "class" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Additional CSS classes" }</td>
                    </tr>
                </tbody>
            </table>

            <H2>{ "Best Practices" }</H2>
            <ul class="list-disc pl-5 space-y-2 mb-4">
                <li>{ "Use PageContent as the main container for your application's page content" }</li>
                <li>{ "Maintain consistent spacing and padding across different pages" }</li>
                <li>{ "Consider using PageHeader in conjunction with PageContent for a complete page layout" }</li>
                <li>{ "Apply custom classes to match your application's design system when necessary" }</li>
            </ul>
        </Container>
    }
}
