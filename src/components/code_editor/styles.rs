use yew::prelude::*;

/// Component for the Code Editor's CSS styles
#[function_component(CodeEditorStyles)]
pub fn code_editor_styles() -> Html {
    html! {
        <style>
            {r#"
            /* Cursor blinking animation */
            @keyframes blink {
                0%, 100% { opacity: 1; }
                50% { opacity: 0; }
            }
            
            /* Syntax highlighting colors */
            .token.comment, .token.prolog, .token.doctype, .token.cdata {
                @apply text-gray-500 dark:text-gray-400 italic;
            }
            
            .token.punctuation {
                @apply text-gray-500 dark:text-gray-400;
            }
            
            .token.property, .token.tag, .token.constant, .token.symbol, .token.deleted {
                @apply text-pink-600 dark:text-pink-400;
            }
            
            .token.boolean, .token.number {
                @apply text-purple-600 dark:text-purple-400;
            }
            
            .token.selector, .token.attr-name, .token.string, .token.char, .token.builtin, .token.inserted {
                @apply text-green-600 dark:text-green-400;
            }
            
            .token.operator, .token.entity, .token.url, .language-css .token.string, .style .token.string {
                @apply text-yellow-600 dark:text-yellow-400;
            }
            
            .token.atrule, .token.attr-value, .token.keyword {
                @apply text-blue-600 dark:text-blue-400;
            }
            
            .token.function, .token.class-name {
                @apply text-red-600 dark:text-red-400;
            }
            
            .token.regex, .token.important, .token.variable {
                @apply text-orange-600 dark:text-orange-400;
            }
            
            .token.important, .token.bold {
                @apply font-bold;
            }
            
            .token.italic {
                @apply italic;
            }
            
            .token.entity {
                @apply cursor-help;
            }
            
            /* Selection styles */
            .selection-highlight {
                @apply bg-blue-200 dark:bg-blue-800 bg-opacity-50 dark:bg-opacity-50;
            }
            
            /* Diff styles */
            .diff-added {
                @apply bg-green-200 dark:bg-green-900 bg-opacity-30 dark:bg-opacity-30;
            }
            
            .diff-removed {
                @apply bg-red-200 dark:bg-red-900 bg-opacity-30 dark:bg-opacity-30;
            }
            
            .diff-modified {
                @apply bg-yellow-200 dark:bg-yellow-900 bg-opacity-30 dark:bg-opacity-30;
            }
            
            /* Annotation styles */
            .annotation-error {
                @apply border-b-2 border-red-500;
            }
            
            .annotation-warning {
                @apply border-b-2 border-yellow-500;
            }
            
            .annotation-info {
                @apply border-b-2 border-blue-500;
            }

            /* Markdown-specific syntax highlighting */
            .token.markdown-header {
                @apply text-purple-600 dark:text-purple-400 font-bold;
            }

            .token.markdown-header-text {
                @apply text-purple-700 dark:text-purple-300 font-bold text-lg;
            }

            .token.markdown-bold {
                @apply font-bold text-gray-900 dark:text-white;
            }

            .token.markdown-bold-marker {
                @apply text-gray-500 dark:text-gray-400 font-bold;
            }

            .token.markdown-italic {
                @apply italic text-gray-900 dark:text-white;
            }

            .token.markdown-italic-marker {
                @apply text-gray-500 dark:text-gray-400 italic;
            }

            .token.markdown-code {
                @apply bg-gray-100 dark:bg-gray-800 text-pink-600 dark:text-pink-400 px-1 rounded font-mono;
            }

            .token.markdown-code-marker {
                @apply text-gray-400 dark:text-gray-600;
            }

            .token.markdown-code-fence {
                @apply text-gray-400 dark:text-gray-600 font-mono;
            }

            .token.markdown-link-text {
                @apply text-blue-600 dark:text-blue-400 underline;
            }

            .token.markdown-link-url {
                @apply text-blue-400 dark:text-blue-600;
            }

            .token.markdown-link-marker {
                @apply text-gray-500 dark:text-gray-400;
            }

            .token.markdown-image-alt {
                @apply text-green-600 dark:text-green-400;
            }

            .token.markdown-image-url {
                @apply text-green-400 dark:text-green-600;
            }

            .token.markdown-image-marker {
                @apply text-gray-500 dark:text-gray-400;
            }

            .token.markdown-list-marker {
                @apply text-orange-600 dark:text-orange-400 font-bold;
            }

            .token.markdown-blockquote {
                @apply text-gray-600 dark:text-gray-400 italic;
            }

            .token.markdown-blockquote-marker {
                @apply text-gray-500 dark:text-gray-500 font-bold;
            }

            .token.markdown-hr {
                @apply text-gray-400 dark:text-gray-600;
            }

            .token.markdown-strikethrough {
                @apply line-through text-gray-600 dark:text-gray-400;
            }

            .token.markdown-strikethrough-marker {
                @apply text-gray-500 dark:text-gray-400;
            }

            /* Drag and drop styles */
            .editor-drag-over {
                @apply ring-4 ring-blue-500 ring-opacity-50 bg-blue-50 dark:bg-blue-900 bg-opacity-20;
            }

            .editor-drag-indicator {
                @apply absolute inset-0 flex items-center justify-center bg-blue-500 bg-opacity-10 pointer-events-none;
            }
            "#}
        </style>
    }
}