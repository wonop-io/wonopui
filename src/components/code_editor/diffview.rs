use similar::{DiffTag, TextDiff};
use yew::prelude::*;
use web_sys::KeyboardEvent;
use wasm_bindgen::JsCast;

use super::diff_types::{ChangeType, DiffHunk, DiffLine, DiffLineInfo, DiffSide, DiffViewMode, WordChange};
use super::syntax_highlighter::highlight_code_line;

#[derive(Properties, PartialEq, Clone)]
pub struct DiffViewProps {
    pub old_text: String,
    pub new_text: String,
    
    #[prop_or_default]
    pub mode: DiffViewMode,
    
    #[prop_or(true)]
    pub show_line_numbers: bool,
    
    #[prop_or(3)]
    pub context_lines: usize,
    
    #[prop_or_else(|| "rust".to_string())]
    pub language: String,
    
    #[prop_or_else(|| "auto".to_string())]
    pub theme: String,
    
    #[prop_or(14)]
    pub font_size: u8,
    
    #[prop_or_else(|| "JetBrains Mono, monospace".to_string())]
    pub font_family: String,
    
    #[prop_or(1.5)]
    pub line_height: f32,
    
    #[prop_or(false)]
    pub unified_diff: bool,
    
    #[prop_or(false)]
    pub word_diff: bool,
    
    #[prop_or(false)]
    pub ignore_whitespace: bool,
    
    #[prop_or(false)]
    pub collapsible_unchanged: bool,
    
    #[prop_or_default]
    pub on_line_click: Option<Callback<DiffLineInfo>>,
    
    #[prop_or_default]
    pub class: Classes,
}

pub struct DiffView {
    diff_hunks: Vec<DiffHunk>,
    collapsed_hunks: Vec<bool>,
    search_term: String,
    search_results: Vec<(usize, usize)>, // (hunk_index, line_index)
    current_search_result: usize,
}

pub enum DiffViewMessage {
    ToggleCollapse(usize),
    Search(String),
    NextSearchResult,
    PrevSearchResult,
    KeyPress(KeyboardEvent),
}

impl Component for DiffView {
    type Message = DiffViewMessage;
    type Properties = DiffViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let diff_hunks = compute_diff_with_options(
            &ctx.props().old_text,
            &ctx.props().new_text,
            ctx.props().context_lines,
            ctx.props().word_diff,
        );
        
        let collapsed_hunks = vec![false; diff_hunks.len()];
        
        Self { 
            diff_hunks,
            collapsed_hunks,
            search_term: String::new(),
            search_results: Vec::new(),
            current_search_result: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DiffViewMessage::ToggleCollapse(index) => {
                if index < self.collapsed_hunks.len() {
                    self.collapsed_hunks[index] = !self.collapsed_hunks[index];
                    true
                } else {
                    false
                }
            }
            DiffViewMessage::Search(term) => {
                self.search_term = term;
                self.search_results.clear();
                
                // Perform search through all hunks and lines
                for (hunk_idx, hunk) in self.diff_hunks.iter().enumerate() {
                    for (line_idx, line) in hunk.lines.iter().enumerate() {
                        if line.content.to_lowercase().contains(&self.search_term.to_lowercase()) {
                            self.search_results.push((hunk_idx, line_idx));
                        }
                    }
                }
                
                self.current_search_result = 0;
                true
            }
            DiffViewMessage::NextSearchResult => {
                if !self.search_results.is_empty() {
                    self.current_search_result = (self.current_search_result + 1) % self.search_results.len();
                    true
                } else {
                    false
                }
            }
            DiffViewMessage::PrevSearchResult => {
                if !self.search_results.is_empty() {
                    if self.current_search_result == 0 {
                        self.current_search_result = self.search_results.len() - 1;
                    } else {
                        self.current_search_result -= 1;
                    }
                    true
                } else {
                    false
                }
            }
            DiffViewMessage::KeyPress(event) => {
                match event.key().as_str() {
                    "f" if event.ctrl_key() => {
                        // Open search dialog (would need to implement)
                        true
                    }
                    "n" if event.ctrl_key() => {
                        yew::Component::update(self, _ctx, DiffViewMessage::NextSearchResult)
                    }
                    "p" if event.ctrl_key() => {
                        yew::Component::update(self, _ctx, DiffViewMessage::PrevSearchResult)
                    }
                    _ => false
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.old_text != ctx.props().old_text 
            || old_props.new_text != ctx.props().new_text 
            || old_props.context_lines != ctx.props().context_lines 
            || old_props.word_diff != ctx.props().word_diff
        {
            self.diff_hunks = compute_diff_with_options(
                &ctx.props().old_text,
                &ctx.props().new_text,
                ctx.props().context_lines,
                ctx.props().word_diff,
            );
            self.collapsed_hunks = vec![false; self.diff_hunks.len()];
            self.search_results.clear();
            self.current_search_result = 0;
            true
        } else {
            old_props != ctx.props()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        let container_style = format!(
            "font-family: {}; font-size: {}px; line-height: {};",
            props.font_family,
            props.font_size,
            props.line_height,
        );

        let onkeydown = ctx.link().callback(DiffViewMessage::KeyPress);
        
        html! {
            <div 
                class={classes!(
                    props.class.clone(),
                    "diff-view-container",
                    "border", "border-gray-300", "dark:border-gray-700",
                    "bg-white", "dark:bg-gray-900", "rounded-sm", "overflow-hidden",
                    "transition-all", "duration-300", "ease-in-out"
                )}
                style={container_style}
                role="region"
                aria-label="Diff viewer"
                aria-description={format!("Showing differences between original and modified text in {} mode", 
                    if matches!(props.mode, DiffViewMode::SideBySide) { "side-by-side" } else { "inline" })}
                tabindex="0"
                onkeydown={onkeydown}
            >
                <div class="relative min-h-[200px]">
                    <div class={classes!(
                        "transition-opacity", "duration-300",
                        if matches!(props.mode, DiffViewMode::SideBySide) { "opacity-100" } else { "opacity-0 absolute inset-0 pointer-events-none" }
                    )}>
                        { if matches!(props.mode, DiffViewMode::SideBySide) {
                            self.render_side_by_side(ctx)
                        } else {
                            html! {}
                        }}
                    </div>
                    <div class={classes!(
                        "transition-opacity", "duration-300",
                        if matches!(props.mode, DiffViewMode::Inline) { "opacity-100" } else { "opacity-0 absolute inset-0 pointer-events-none" }
                    )}>
                        { if matches!(props.mode, DiffViewMode::Inline) {
                            self.render_inline(ctx)
                        } else {
                            html! {}
                        }}
                    </div>
                </div>
            </div>
        }
    }
}

impl DiffView {
    /// Export the diff as a unified diff string
    pub fn export_unified_diff(&self, filename: &str) -> String {
        let mut output = String::new();
        
        // Add file headers
        output.push_str(&format!("--- a/{}\n", filename));
        output.push_str(&format!("+++ b/{}\n", filename));
        
        // Add hunks
        for hunk in &self.diff_hunks {
            output.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                hunk.old_start, hunk.old_count,
                hunk.new_start, hunk.new_count
            ));
            
            for line in &hunk.lines {
                let prefix = match line.change_type {
                    ChangeType::Added => "+",
                    ChangeType::Removed => "-",
                    _ => " ",
                };
                output.push_str(&format!("{}{}", prefix, line.content));
                if !line.content.ends_with('\n') {
                    output.push('\n');
                }
            }
        }
        
        output
    }
    
    /// Export the diff as a patch file
    pub fn export_patch(&self, old_filename: &str, new_filename: &str) -> String {
        let mut output = String::new();
        
        // Add git-style header
        output.push_str(&format!("diff --git a/{} b/{}\n", old_filename, new_filename));
        output.push_str("index 0000000..0000000 100644\n");
        output.push_str(&format!("--- a/{}\n", old_filename));
        output.push_str(&format!("+++ b/{}\n", new_filename));
        
        // Add the unified diff content
        for hunk in &self.diff_hunks {
            output.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                hunk.old_start, hunk.old_count,
                hunk.new_start, hunk.new_count
            ));
            
            for line in &hunk.lines {
                let prefix = match line.change_type {
                    ChangeType::Added => "+",
                    ChangeType::Removed => "-",
                    _ => " ",
                };
                output.push_str(&format!("{}{}", prefix, line.content));
                if !line.content.ends_with('\n') {
                    output.push('\n');
                }
            }
        }
        
        output
    }
    
    fn render_side_by_side(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        // Prepare left and right lines
        let (left_lines, right_lines) = self.prepare_side_by_side_lines();
        
        html! {
            <div class="flex divide-x divide-gray-200 dark:divide-gray-700 bg-white dark:bg-gray-900">
                // Left pane (old text)
                <div class="flex-1 min-w-0 bg-white dark:bg-gray-900" role="group" aria-label="Original text">
                    <div class="bg-gradient-to-r from-red-100 to-red-50 dark:from-red-500/10 dark:to-red-600/5 px-4 py-2 border-b-2 border-red-300 dark:border-red-500 flex items-center justify-between shadow-xs dark:shadow-red-900/20">
                        <h3 class="text-sm font-bold text-red-700 dark:text-red-300 flex items-center gap-2">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                            </svg>
                            {"Original"}
                        </h3>
                        <span class="text-xs font-medium text-red-600 dark:text-red-400">
                            {format!("{} lines", left_lines.iter().filter(|l| l.is_some()).count())}
                        </span>
                    </div>
                    <div class="overflow-x-auto">
                        { self.render_side_pane(&left_lines, DiffSide::Left, props.show_line_numbers, ctx) }
                    </div>
                </div>
                
                // Right pane (new text)
                <div class="flex-1 min-w-0 bg-white dark:bg-gray-900" role="group" aria-label="Modified text">
                    <div class="bg-gradient-to-r from-green-100 to-green-50 dark:from-green-500/10 dark:to-green-600/5 px-4 py-2 border-b-2 border-green-300 dark:border-green-500 flex items-center justify-between shadow-xs dark:shadow-green-900/20">
                        <h3 class="text-sm font-bold text-green-700 dark:text-green-300 flex items-center gap-2">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                            {"Modified"}
                        </h3>
                        <span class="text-xs font-medium text-green-600 dark:text-green-400">
                            {format!("{} lines", right_lines.iter().filter(|l| l.is_some()).count())}
                        </span>
                    </div>
                    <div class="overflow-x-auto">
                        { self.render_side_pane(&right_lines, DiffSide::Right, props.show_line_numbers, ctx) }
                    </div>
                </div>
            </div>
        }
    }

    fn render_inline(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        html! {
            <div class="relative">
                <div class="bg-gradient-to-r from-gray-100 to-gray-50 dark:from-gray-800 dark:to-gray-850 px-4 py-2 border-b border-gray-300 dark:border-gray-700 flex items-center justify-between">
                    <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">{"Unified Diff View"}</h3>
                    <div class="flex items-center gap-4 text-xs">
                        <span class="flex items-center gap-1">
                            <span class="w-3 h-3 bg-green-500 rounded-sm shadow-xs border border-green-600"></span>
                            <span class="text-gray-600 dark:text-gray-400">{"Added"}</span>
                        </span>
                        <span class="flex items-center gap-1">
                            <span class="w-3 h-3 bg-red-500 rounded-sm shadow-xs border border-red-600"></span>
                            <span class="text-gray-600 dark:text-gray-400">{"Removed"}</span>
                        </span>
                        <span class="flex items-center gap-1">
                            <span class="w-3 h-3 bg-yellow-500 rounded-sm shadow-xs border border-yellow-600"></span>
                            <span class="text-gray-600 dark:text-gray-400">{"Modified"}</span>
                        </span>
                    </div>
                </div>
                <div class="overflow-x-auto">
                    { 
                        if self.diff_hunks.is_empty() {
                            html! {
                                <div class="p-8 text-center text-gray-500 dark:text-gray-400">
                                    <div class="text-lg mb-2">{"No differences found"}</div>
                                    <div class="text-sm">{"The texts are identical"}</div>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="divide-y divide-gray-200 dark:divide-gray-700">
                                    { 
                                        for self.diff_hunks.iter().enumerate().map(|(idx, hunk)| {
                                            self.render_inline_hunk_enhanced(hunk, ctx, idx)
                                        })
                                    }
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        }
    }

    fn prepare_side_by_side_lines(&self) -> (Vec<Option<DiffLine>>, Vec<Option<DiffLine>>) {
        let mut left_lines = Vec::new();
        let mut right_lines = Vec::new();

        for hunk in &self.diff_hunks {
            let mut i = 0;
            while i < hunk.lines.len() {
                let line = &hunk.lines[i];
                match line.change_type {
                    ChangeType::Unchanged => {
                        // Unchanged lines go on both sides, perfectly aligned
                        left_lines.push(Some(line.clone()));
                        right_lines.push(Some(line.clone()));
                        i += 1;
                    }
                    ChangeType::Removed => {
                        // Collect all consecutive removed lines
                        let mut removed_lines = vec![line.clone()];
                        let mut j = i + 1;
                        while j < hunk.lines.len() && hunk.lines[j].change_type == ChangeType::Removed {
                            removed_lines.push(hunk.lines[j].clone());
                            j += 1;
                        }
                        
                        // Check for added lines immediately after
                        let mut added_lines = Vec::new();
                        while j < hunk.lines.len() && hunk.lines[j].change_type == ChangeType::Added {
                            added_lines.push(hunk.lines[j].clone());
                            j += 1;
                        }
                        
                        // Align removed and added lines side by side
                        let max_count = removed_lines.len().max(added_lines.len());
                        for idx in 0..max_count {
                            if idx < removed_lines.len() {
                                left_lines.push(Some(removed_lines[idx].clone()));
                            } else {
                                left_lines.push(None);
                            }
                            
                            if idx < added_lines.len() {
                                right_lines.push(Some(added_lines[idx].clone()));
                            } else {
                                right_lines.push(None);
                            }
                        }
                        
                        i = j;
                    }
                    ChangeType::Added => {
                        // If we hit an Added without a preceding Removed, it's a pure addition
                        left_lines.push(None);
                        right_lines.push(Some(line.clone()));
                        i += 1;
                    }
                    ChangeType::Modified => {
                        // Modified lines appear on both sides for comparison
                        left_lines.push(Some(line.clone()));
                        right_lines.push(Some(line.clone()));
                        i += 1;
                    }
                }
            }
        }

        (left_lines, right_lines)
    }

    fn render_side_pane(&self, lines: &[Option<DiffLine>], side: DiffSide, show_line_numbers: bool, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {
                    for lines.iter().enumerate().map(|(idx, line_opt)| {
                        self.render_side_line(line_opt, idx, side, show_line_numbers, ctx)
                    })
                }
            </div>
        }
    }
    
    fn render_side_line(&self, line_opt: &Option<DiffLine>, idx: usize, side: DiffSide, show_line_numbers: bool, ctx: &Context<Self>) -> Html {
        if let Some(line) = line_opt {
            let (bg_class, hover_class, border_class) = match line.change_type {
                ChangeType::Added => match side {
                    DiffSide::Right => (
                        "bg-green-50 dark:bg-green-500/20",
                        "hover:bg-green-100 dark:hover:bg-green-500/30",
                        "border-l-4 border-green-500 dark:border-green-400"
                    ),
                    _ => ("dark:bg-gray-800/50", "hover:bg-gray-50 dark:hover:bg-gray-800/70", "")
                },
                ChangeType::Removed => match side {
                    DiffSide::Left => (
                        "bg-red-50 dark:bg-red-500/20",
                        "hover:bg-red-100 dark:hover:bg-red-500/30",
                        "border-l-4 border-red-500 dark:border-red-400"
                    ),
                    _ => ("dark:bg-gray-800/50", "hover:bg-gray-50 dark:hover:bg-gray-800/70", "")
                },
                ChangeType::Modified => (
                    "bg-yellow-50 dark:bg-yellow-500/20",
                    "hover:bg-yellow-100 dark:hover:bg-yellow-500/30",
                    "border-l-4 border-yellow-500 dark:border-yellow-400"
                ),
                ChangeType::Unchanged => (
                    "dark:bg-transparent",
                    "hover:bg-gray-100 dark:hover:bg-gray-700",
                    ""
                ),
            };
            
            let line_no = match side {
                DiffSide::Left => line.old_line_no,
                DiffSide::Right => line.new_line_no,
                _ => None,
            };
            
            let prefix = match line.change_type {
                ChangeType::Added => "+",
                ChangeType::Removed => "−",
                ChangeType::Modified => "~",
                _ => " ",
            };
            
            let prefix_class = match line.change_type {
                ChangeType::Added => "text-green-600 dark:text-green-400 font-bold",
                ChangeType::Removed => "text-red-600 dark:text-red-400 font-bold",
                ChangeType::Modified => "text-yellow-600 dark:text-yellow-400 font-bold",
                _ => "text-gray-400 dark:text-gray-500",
            };
            
            html! {
                <div
                    key={idx}
                    class={classes!(
                        "flex", "min-h-[1.5rem]", "leading-6", "font-mono", "text-sm",
                        "transition-colors", "duration-150",
                        bg_class, hover_class, border_class
                    )}
                >
                    if show_line_numbers {
                        <div class="flex-none w-12 px-2 text-right text-gray-600 dark:text-gray-400 select-none bg-gray-50/50 dark:bg-gray-900/50 border-r border-gray-200 dark:border-gray-700">
                            { line_no.map(|n| n.to_string()).unwrap_or_else(|| String::new()) }
                        </div>
                    }
                    <div class="flex-none px-2 text-center select-none">
                        <span class={classes!("inline-block", "w-4", prefix_class)}>
                            { prefix }
                        </span>
                    </div>
                    <div class="flex-1 px-2 whitespace-pre overflow-x-auto">
                        <span class="text-gray-900 dark:text-gray-100">
                            { if !line.content.trim().is_empty() {
                                self.highlight_line_with_theme(&line.content, ctx.props())
                            } else {
                                html! { {&line.content} }
                            }}
                        </span>
                    </div>
                </div>
            }
        } else {
            // Empty placeholder for missing lines
            html! {
                <div
                    key={idx}
                    class="flex min-h-[1.5rem] leading-6 font-mono text-sm bg-gray-50/30 dark:bg-gray-800/30"
                >
                    if show_line_numbers {
                        <div class="flex-none w-12 px-2 text-right select-none bg-gray-50/50 dark:bg-gray-900/50 border-r border-gray-200 dark:border-gray-700">
                            {" "}
                        </div>
                    }
                    <div class="flex-1 px-4 text-gray-300 dark:text-gray-700">
                        {" "}
                    </div>
                </div>
            }
        }
    }


    fn render_inline_hunk_enhanced(&self, hunk: &DiffHunk, ctx: &Context<Self>, hunk_idx: usize) -> Html {
        let props = ctx.props();
        let is_collapsed = self.collapsed_hunks.get(hunk_idx).copied().unwrap_or(false);
        let has_only_unchanged = hunk.lines.iter().all(|l| l.change_type == ChangeType::Unchanged);
        let link = ctx.link();
        
        // Determine if this hunk should be collapsible
        let is_collapsible = props.collapsible_unchanged && has_only_unchanged && hunk.lines.len() > 5;
        
        let toggle_callback = link.callback(move |_| DiffViewMessage::ToggleCollapse(hunk_idx));
        
        html! {
            <div class="group hover:bg-gray-50 dark:hover:bg-gray-800/30 transition-colors">
                if props.unified_diff || is_collapsible {
                    <div class="sticky top-0 bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-500/10 dark:to-indigo-500/10 px-3 py-1 text-xs text-blue-700 dark:text-blue-300 font-mono border-b border-blue-200 dark:border-blue-700 flex items-center justify-between shadow-xs dark:shadow-blue-900/20">
                        <div class="flex items-center gap-2">
                            if is_collapsible {
                                <button 
                                    onclick={toggle_callback}
                                    class="hover:bg-blue-200 dark:hover:bg-blue-800 p-1 rounded-sm transition-colors"
                                    aria-label={if is_collapsed { "Expand section" } else { "Collapse section" }}
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        if is_collapsed {
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                        } else {
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                        }
                                    </svg>
                                </button>
                            }
                            <span>
                                { format!("@@ -{},{} +{},{} @@", 
                                    hunk.old_start, hunk.old_count, 
                                    hunk.new_start, hunk.new_count) }
                            </span>
                            if is_collapsed {
                                <span class="text-blue-600 dark:text-blue-400 italic">
                                    { format!("({} unchanged lines)", hunk.lines.len()) }
                                </span>
                            }
                        </div>
                        <span class="text-blue-600 dark:text-blue-400 opacity-60">
                            { format!("Hunk {}/{}", hunk_idx + 1, self.diff_hunks.len()) }
                        </span>
                    </div>
                }
                if !is_collapsed {
                    <div class="relative">
                        {
                            for hunk.lines.iter().map(|line| {
                                self.render_enhanced_diff_line(line, props)
                            })
                        }
                    </div>
                }
            </div>
        }
    }
    
    fn render_enhanced_diff_line(&self, line: &DiffLine, props: &DiffViewProps) -> Html {
        let (bg_class, border_class, symbol, symbol_class) = match line.change_type {
            ChangeType::Added => (
                "bg-green-100 dark:bg-green-900/50 hover:bg-green-200 dark:hover:bg-green-900/70 border-l-4 border-green-500",
                "border-l-4 border-emerald-500",
                "+",
                "text-green-700 dark:text-green-300 font-bold text-lg"
            ),
            ChangeType::Removed => (
                "bg-red-100 dark:bg-red-900/50 hover:bg-red-200 dark:hover:bg-red-900/70 border-l-4 border-red-500",
                "border-l-4 border-rose-500",
                "−",
                "text-red-700 dark:text-red-300 font-bold text-lg"
            ),
            ChangeType::Modified => (
                "bg-yellow-100 dark:bg-yellow-900/50 hover:bg-yellow-200 dark:hover:bg-yellow-900/70 border-l-4 border-yellow-500",
                "border-l-4 border-amber-500",
                "~",
                "text-yellow-700 dark:text-yellow-300 font-bold text-lg"
            ),
            ChangeType::Unchanged => (
                "hover:bg-gray-50 dark:hover:bg-gray-850",
                "",
                " ",
                "text-gray-400 dark:text-gray-600"
            ),
        };
        
        html! {
            <div class={classes!("flex", "group", bg_class, border_class, "transition-all", "duration-150")}>
                if props.show_line_numbers {
                    <div class="flex-none px-2 py-0 text-xs text-gray-500 dark:text-gray-400 select-none bg-gray-50 dark:bg-gray-850 border-r border-gray-200 dark:border-gray-700 font-mono">
                        <span class="inline-block w-12 text-right opacity-70">
                            { line.old_line_no.map(|n| n.to_string()).unwrap_or_else(|| " ".to_string()) }
                        </span>
                        <span class="mx-1 opacity-50">{"→"}</span>
                        <span class="inline-block w-12 text-right opacity-70">
                            { line.new_line_no.map(|n| n.to_string()).unwrap_or_else(|| " ".to_string()) }
                        </span>
                    </div>
                }
                <div class="flex-none px-2 text-center select-none">
                    <span class={classes!("inline-block", "w-4", symbol_class)}>
                        { symbol }
                    </span>
                </div>
                <div class="flex-1 px-2 whitespace-pre overflow-x-auto">
                    { self.highlight_line_with_theme(&line.content, props) }
                </div>
            </div>
        }
    }
    
    fn render_inline_hunk(&self, hunk: &DiffHunk, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        html! {
            <div class="mb-4 border border-gray-200 dark:border-gray-700 rounded-sm bg-white dark:bg-gray-900">
                if props.unified_diff {
                    <div class="bg-gray-100 dark:bg-gray-800/70 px-2 py-1 text-xs text-gray-600 dark:text-gray-300 font-mono border-b border-gray-200 dark:border-gray-700">
                        { format!("@@ -{},{} +{},{} @@", 
                            hunk.old_start, hunk.old_count, 
                            hunk.new_start, hunk.new_count) }
                    </div>
                }
                <div>
                    {
                        for hunk.lines.iter().map(|line| {
                            let (bg_class, text_class) = match line.change_type {
                                ChangeType::Added => (
                                    "bg-green-50 dark:bg-green-500/20 border-l-4 border-green-500 dark:border-green-400",
                                    "text-green-800 dark:text-green-200 font-medium"
                                ),
                                ChangeType::Removed => (
                                    "bg-red-50 dark:bg-red-500/20 border-l-4 border-red-500 dark:border-red-400",
                                    "text-red-800 dark:text-red-200 font-medium"
                                ),
                                ChangeType::Modified => (
                                    "bg-yellow-50 dark:bg-yellow-500/20 border-l-4 border-yellow-500 dark:border-yellow-400",
                                    "text-yellow-800 dark:text-yellow-200 font-medium"
                                ),
                                ChangeType::Unchanged => (
                                    "bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800",
                                    "text-gray-800 dark:text-gray-200"
                                ),
                            };
                            
                            let prefix = match line.change_type {
                                ChangeType::Added => "+",
                                ChangeType::Removed => "-",
                                ChangeType::Modified => "~",
                                ChangeType::Unchanged => " ",
                            };
                            
                            html! {
                                <div class={classes!("flex", bg_class, text_class)}>
                                    if props.show_line_numbers {
                                        <div class={classes!(
                                            "flex-none", "px-2", "py-0", "text-right", "select-none", "border-r", "min-w-[6rem]",
                                            match line.change_type {
                                                ChangeType::Added => "bg-green-50/50 dark:bg-green-900/20 text-green-700 dark:text-green-400 border-green-200 dark:border-green-700",
                                                ChangeType::Removed => "bg-red-50/50 dark:bg-red-900/20 text-red-700 dark:text-red-400 border-red-200 dark:border-red-700",
                                                ChangeType::Modified => "bg-yellow-50/50 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400 border-yellow-200 dark:border-yellow-700",
                                                ChangeType::Unchanged => "bg-gray-50 dark:bg-gray-800 text-gray-600 dark:text-gray-400 border-gray-200 dark:border-gray-700",
                                            }
                                        )}>
                                            <span class="inline-block w-10 text-right">
                                                { line.old_line_no.map(|n| n.to_string()).unwrap_or_default() }
                                            </span>
                                            <span class="mx-1">{"→"}</span>
                                            <span class="inline-block w-10 text-right">
                                                { line.new_line_no.map(|n| n.to_string()).unwrap_or_default() }
                                            </span>
                                        </div>
                                    }
                                    <div class="flex-1 px-2 whitespace-pre overflow-x-auto">
                                        <span class="select-none opacity-50 font-bold">{prefix}{" "}</span>
                                        <span class="text-gray-900 dark:text-gray-100">
                                            { if !line.content.trim().is_empty() {
                                                self.highlight_line(&line.content, &props.language)
                                            } else {
                                                html! { {&line.content} }
                                            }}
                                        </span>
                                    </div>
                                </div>
                            }
                        })
                    }
                </div>
            </div>
        }
    }

    fn highlight_line(&self, content: &str, language: &str) -> Html {
        // Use the syntect-based highlighter for proper syntax highlighting
        // Auto-detect theme based on dark mode preference
        let theme = if web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
            .and_then(|m| m)
            .map(|m| m.matches())
            .unwrap_or(false)
        {
            "dark"
        } else {
            "light"
        };
        highlight_code_line(content, language, theme, true)
    }
    
    fn highlight_line_with_theme(&self, content: &str, props: &DiffViewProps) -> Html {
        // Use theme-aware highlighting with auto detection
        let theme = match props.theme.as_str() {
            "auto" => {
                // Check if dark mode is active via CSS media query or class
                if web_sys::window()
                    .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
                    .and_then(|m| m)
                    .map(|m| m.matches())
                    .unwrap_or(false)
                {
                    "dark"
                } else {
                    "light"
                }
            }
            "dark" => "dark",
            "light" => "light",
            theme => theme,
        };
        highlight_code_line(content, &props.language, theme, true)
    }

    fn highlight_rust_line(&self, line: &str) -> Html {
        let keywords = [
            "fn", "let", "mut", "pub", "struct", "impl", "enum", "trait", "use", "mod",
            "match", "if", "else", "for", "while", "loop", "return", "self", "Self",
            "true", "false", "const", "static", "async", "await", "move", "ref",
        ];

        // Check for comment
        if let Some(comment_start) = line.find("//") {
            let before_comment = &line[..comment_start];
            let comment_part = &line[comment_start..];
            
            return html! {
                <>
                    { self.highlight_rust_tokens(before_comment, &keywords) }
                    <span class="text-slate-500 dark:text-slate-400 italic">{ comment_part }</span>
                </>
            };
        }

        self.highlight_rust_tokens(line, &keywords)
    }

    fn highlight_rust_tokens(&self, text: &str, keywords: &[&str]) -> Html {
        let mut result = Vec::new();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_char = '"';

        for ch in text.chars() {
            if in_string {
                current_word.push(ch);
                if ch == string_char {
                    result.push(html! { 
                        <span class="text-green-600 dark:text-green-400 font-bold">
                            { current_word.clone() }
                        </span> 
                    });
                    current_word.clear();
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' => {
                    if !current_word.is_empty() {
                        self.push_word(&mut result, &current_word, keywords);
                        current_word.clear();
                    }
                    current_word.push(ch);
                    in_string = true;
                    string_char = ch;
                }
                ' ' | '\t' => {
                    if !current_word.is_empty() {
                        self.push_word(&mut result, &current_word, keywords);
                        current_word.clear();
                    }
                    result.push(html! { { ch.to_string() } });
                }
                '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' | '=' | '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '!' => {
                    if !current_word.is_empty() {
                        self.push_word(&mut result, &current_word, keywords);
                        current_word.clear();
                    }
                    result.push(html! { 
                        <span class="text-slate-600 dark:text-slate-400">
                            { ch.to_string() }
                        </span> 
                    });
                }
                _ => {
                    current_word.push(ch);
                }
            }
        }

        if !current_word.is_empty() {
            if in_string {
                result.push(html! { 
                    <span class="text-emerald-600 dark:text-emerald-400">
                        { current_word }
                    </span> 
                });
            } else {
                self.push_word(&mut result, &current_word, keywords);
            }
        }

        html! { <>{ for result }</> }
    }

    fn highlight_js_line(&self, line: &str) -> Html {
        // Simplified for now
        html! { { line } }
    }

    fn push_word(&self, result: &mut Vec<Html>, word: &str, keywords: &[&str]) {
        if keywords.contains(&word) {
            result.push(html! { 
                <span class="text-indigo-600 dark:text-indigo-400 font-bold">
                    { word.to_string() }
                </span> 
            });
        } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
            result.push(html! { 
                <span class="text-fuchsia-600 dark:text-fuchsia-400">
                    { word.to_string() }
                </span> 
            });
        } else {
            result.push(html! { 
                <span class="text-gray-800 dark:text-gray-200">
                    { word.to_string() }
                </span> 
            });
        }
    }
}

pub fn compute_diff(old_text: &str, new_text: &str, context_lines: usize) -> Vec<DiffHunk> {
    compute_diff_with_options(old_text, new_text, context_lines, false)
}

pub fn compute_diff_with_options(old_text: &str, new_text: &str, context_lines: usize, word_diff: bool) -> Vec<DiffHunk> {
    let diff = TextDiff::from_lines(old_text, new_text);
    let mut hunks = Vec::new();
    
    for group in diff.grouped_ops(context_lines) {
        let mut lines = Vec::new();
        let mut old_line_no = group.first().map(|op| op.old_range().start + 1).unwrap_or(1);
        let mut new_line_no = group.first().map(|op| op.new_range().start + 1).unwrap_or(1);
        
        let old_start = old_line_no;
        let new_start = new_line_no;
        
        for op in group {
            match op.tag() {
                DiffTag::Delete => {
                    let old_range = op.old_range();
                    for idx in old_range.clone() {
                        let old_line = diff.old_slices()[idx];
                        lines.push(DiffLine {
                            old_line_no: Some(old_line_no),
                            new_line_no: None,
                            change_type: ChangeType::Removed,
                            content: old_line.to_string(),
                            word_changes: None,
                        });
                        old_line_no += 1;
                    }
                }
                DiffTag::Insert => {
                    let new_range = op.new_range();
                    for idx in new_range.clone() {
                        let new_line = diff.new_slices()[idx];
                        lines.push(DiffLine {
                            old_line_no: None,
                            new_line_no: Some(new_line_no),
                            change_type: ChangeType::Added,
                            content: new_line.to_string(),
                            word_changes: None,
                        });
                        new_line_no += 1;
                    }
                }
                DiffTag::Equal => {
                    let old_range = op.old_range();
                    let new_range = op.new_range();
                    for (old_idx, new_idx) in old_range.zip(new_range) {
                        let old_line = diff.old_slices()[old_idx];
                        lines.push(DiffLine {
                            old_line_no: Some(old_line_no),
                            new_line_no: Some(new_line_no),
                            change_type: ChangeType::Unchanged,
                            content: old_line.to_string(),
                            word_changes: None,
                        });
                        old_line_no += 1;
                        new_line_no += 1;
                    }
                }
                DiffTag::Replace => {
                    // Handle replace as delete + insert
                    let old_range = op.old_range();
                    for idx in old_range.clone() {
                        let old_line = diff.old_slices()[idx];
                        lines.push(DiffLine {
                            old_line_no: Some(old_line_no),
                            new_line_no: None,
                            change_type: ChangeType::Removed,
                            content: old_line.to_string(),
                            word_changes: None,
                        });
                        old_line_no += 1;
                    }
                    
                    let new_range = op.new_range();
                    for idx in new_range.clone() {
                        let new_line = diff.new_slices()[idx];
                        lines.push(DiffLine {
                            old_line_no: None,
                            new_line_no: Some(new_line_no),
                            change_type: ChangeType::Added,
                            content: new_line.to_string(),
                            word_changes: None,
                        });
                        new_line_no += 1;
                    }
                }
            }
        }
        
        let old_count = old_line_no - old_start;
        let new_count = new_line_no - new_start;
        
        hunks.push(DiffHunk {
            old_start,
            old_count,
            new_start,
            new_count,
            lines,
        });
    }
    
    hunks
}

/// Compute word-level differences between two strings
pub fn compute_word_diff(old_text: &str, new_text: &str) -> Vec<WordChange> {
    let old_words = tokenize_line(old_text);
    let new_words = tokenize_line(new_text);
    
    let old_joined = old_words.join(" ");
    let new_joined = new_words.join(" ");
    
    let diff = TextDiff::from_words(&old_joined, &new_joined);
    let mut word_changes = Vec::new();
    let mut char_offset = 0;
    
    for op in diff.ops() {
        match op.tag() {
            DiffTag::Delete => {
                let old_range = op.old_range();
                for idx in old_range {
                    if let Some(word) = old_words.get(idx) {
                        word_changes.push(WordChange {
                            start: char_offset,
                            end: char_offset + word.len(),
                            change_type: ChangeType::Removed,
                        });
                        char_offset += word.len();
                    }
                }
            }
            DiffTag::Insert => {
                let new_range = op.new_range();
                for idx in new_range {
                    if let Some(word) = new_words.get(idx) {
                        word_changes.push(WordChange {
                            start: char_offset,
                            end: char_offset + word.len(),
                            change_type: ChangeType::Added,
                        });
                        char_offset += word.len();
                    }
                }
            }
            DiffTag::Equal => {
                let old_range = op.old_range();
                for idx in old_range {
                    if let Some(word) = old_words.get(idx) {
                        char_offset += word.len();
                    }
                }
            }
            DiffTag::Replace => {
                // Handle replace as delete + insert
                let old_range = op.old_range();
                for idx in old_range {
                    if let Some(word) = old_words.get(idx) {
                        word_changes.push(WordChange {
                            start: char_offset,
                            end: char_offset + word.len(),
                            change_type: ChangeType::Removed,
                        });
                    }
                }
                let new_range = op.new_range();
                for idx in new_range {
                    if let Some(word) = new_words.get(idx) {
                        word_changes.push(WordChange {
                            start: char_offset,
                            end: char_offset + word.len(),
                            change_type: ChangeType::Added,
                        });
                        char_offset += word.len();
                    }
                }
            }
        }
    }
    
    word_changes
}

/// Tokenize a line into words for word-level diff
pub fn tokenize_line(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut in_word = false;
    
    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            if !in_word {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
                in_word = true;
            }
            current_word.push(ch);
        } else {
            if in_word {
                words.push(current_word.clone());
                current_word.clear();
                in_word = false;
            }
            current_word.push(ch);
        }
    }
    
    if !current_word.is_empty() {
        words.push(current_word);
    }
    
    words
}