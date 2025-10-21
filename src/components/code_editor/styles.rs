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

            .animate-blink {
                animation: blink 1s infinite;
            }

            /* Enhanced syntax highlighting colors */
            .token.comment, .token.prolog, .token.doctype, .token.cdata {
                @apply text-slate-500 dark:text-slate-400 italic font-medium opacity-80;
            }

            .token.punctuation {
                @apply text-slate-600 dark:text-slate-400 font-semibold;
            }

            .token.property, .token.tag, .token.constant, .token.symbol, .token.deleted {
                @apply text-rose-600 dark:text-rose-400 font-semibold;
            }

            .token.boolean, .token.number {
                @apply text-fuchsia-600 dark:text-fuchsia-400 font-bold;
            }

            .token.selector, .token.attr-name, .token.string, .token.char, .token.builtin, .token.inserted {
                @apply text-emerald-600 dark:text-emerald-400 font-medium;
            }

            .token.operator, .token.entity, .token.url, .language-css .token.string, .style .token.string {
                @apply text-amber-600 dark:text-amber-400 font-semibold;
            }

            .token.atrule, .token.attr-value, .token.keyword {
                @apply text-indigo-600 dark:text-indigo-400 font-bold;
            }

            .token.function, .token.class-name {
                @apply text-cyan-600 dark:text-cyan-400 font-bold;
            }

            .token.regex, .token.important, .token.variable {
                @apply text-orange-600 dark:text-orange-400 font-semibold;
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

            /* Enhanced selection styles */
            .selection-highlight {
                @apply bg-indigo-200 dark:bg-indigo-800/40 shadow-xs;
            }

            /* Enhanced diff styles with animations */
            .diff-added {
                @apply bg-emerald-200 dark:bg-emerald-800/40 shadow-md;
            }

            .diff-removed {
                @apply bg-rose-200 dark:bg-rose-800/40 shadow-md;
            }

            .diff-modified {
                @apply bg-amber-200 dark:bg-amber-800/40 shadow-md;
            }

            /* Enhanced annotation styles with animations */
            .annotation-error {
                @apply border-b-4 border-rose-500 border-double shadow-xs;
                animation: errorPulse 2s infinite;
            }

            .annotation-warning {
                @apply border-b-4 border-amber-500 border-double shadow-xs;
                animation: warningPulse 2s infinite;
            }

            .annotation-info {
                @apply border-b-4 border-sky-500 border-double shadow-xs;
                animation: infoPulse 2s infinite;
            }

            /* Pulse animations for annotations */
            @keyframes errorPulse {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.7; }
            }

            @keyframes warningPulse {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.7; }
            }

            @keyframes infoPulse {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.7; }
            }
            "#}
        </style>
    }
}
