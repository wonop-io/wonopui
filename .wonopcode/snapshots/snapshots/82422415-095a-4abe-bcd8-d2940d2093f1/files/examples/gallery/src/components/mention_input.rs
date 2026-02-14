use wonopui::*;
use yew::prelude::*;

#[function_component(MentionInputDocumentation)]
pub fn mention_input_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <H1>{ "MentionInput Component" }</H1>
            <Paragraph>
                { "The MentionInput component allows users to input text with support for mentions, like @username, with autocomplete suggestions." }
            </Paragraph>

            <H2>{ "Basic Usage" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <MentionInput
                    placeholder="Type @ to mention someone"
                    value="Try typing @user in this input"
                />
            </div>

            <H2>{ "With Autocomplete" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <MentionInput
                    placeholder="Type @ to see suggestions"
                    default_value="You can mention @john or others"
                    candidates={Callback::from(|input: String| {
                        let users = vec![
                            "john", "jane", "alice", "bob",
                            "charlie", "dave", "emma", "frank", "grace", "hank"
                        ];

                        users.into_iter()
                            .filter(|&user| user.to_lowercase().contains(&input.to_lowercase()))
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    })}
                />
            </div>

            <H2>{ "With Update Callback" }</H2>
            <div class="bg-white dark:bg-zinc-800 border dark:border-zinc-700 rounded-md p-4 mb-4">
                <MentionInput
                    placeholder="Type @ to mention someone"
                    onupdate={Callback::from(|text: String| {
                        // In a real app, you would use this callback to update your state
                        web_sys::console::log_1(&format!("Text updated: {}", text).into());
                    })}
                />
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
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "id" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<String>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Optional ID for the input element" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "default_value" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Initial text to display" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "start_token" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<String>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "@" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Token that starts a mention" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "end_token" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<String>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ " " }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Token that ends a mention" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "candidates" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Callback<String, Vec<String>>>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Callback that returns mention suggestions based on current input" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "onupdate" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Option<Callback<String>>" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "None" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Callback triggered when the text content is updated" }</td>
                    </tr>
                    <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-zinc-100">{ "placeholder" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "String" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "" }</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-zinc-400">{ "Placeholder text for the input" }</td>
                    </tr>
                </tbody>
            </table>
        </Container>
    }
}
