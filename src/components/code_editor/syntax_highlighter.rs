use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet, Theme};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use yew::prelude::*;
use once_cell::sync::Lazy;

// Initialize syntax definitions and themes once
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| {
    SyntaxSet::load_defaults_newlines()
});

static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| {
    ThemeSet::load_defaults()
});

pub struct SyntaxHighlighter {
    syntax: &'static SyntaxReference,
    theme: &'static Theme,
}

impl SyntaxHighlighter {
    pub fn new(language: &str, theme_name: &str) -> Self {
        let syntax = Self::get_syntax(language);
        let theme = Self::get_theme(theme_name);
        
        Self { syntax, theme }
    }
    
    fn get_syntax(language: &str) -> &'static SyntaxReference {
        SYNTAX_SET.find_syntax_by_extension(language)
            .or_else(|| SYNTAX_SET.find_syntax_by_name(language))
            .or_else(|| match language.to_lowercase().as_str() {
                "rs" | "rust" => SYNTAX_SET.find_syntax_by_name("Rust"),
                "js" | "javascript" => SYNTAX_SET.find_syntax_by_name("JavaScript"),
                "ts" | "typescript" => SYNTAX_SET.find_syntax_by_name("TypeScript"),
                "py" | "python" => SYNTAX_SET.find_syntax_by_name("Python"),
                "go" | "golang" => SYNTAX_SET.find_syntax_by_name("Go"),
                "java" => SYNTAX_SET.find_syntax_by_name("Java"),
                "c" => SYNTAX_SET.find_syntax_by_name("C"),
                "cpp" | "c++" => SYNTAX_SET.find_syntax_by_name("C++"),
                "cs" | "csharp" => SYNTAX_SET.find_syntax_by_name("C#"),
                "rb" | "ruby" => SYNTAX_SET.find_syntax_by_name("Ruby"),
                "php" => SYNTAX_SET.find_syntax_by_name("PHP"),
                "swift" => SYNTAX_SET.find_syntax_by_name("Swift"),
                "kotlin" | "kt" => SYNTAX_SET.find_syntax_by_name("Kotlin"),
                "scala" => SYNTAX_SET.find_syntax_by_name("Scala"),
                "sh" | "bash" | "shell" => SYNTAX_SET.find_syntax_by_name("Bourne Again Shell (bash)"),
                "yml" | "yaml" => SYNTAX_SET.find_syntax_by_name("YAML"),
                "json" => SYNTAX_SET.find_syntax_by_name("JSON"),
                "xml" => SYNTAX_SET.find_syntax_by_name("XML"),
                "html" => SYNTAX_SET.find_syntax_by_name("HTML"),
                "css" => SYNTAX_SET.find_syntax_by_name("CSS"),
                "scss" | "sass" => SYNTAX_SET.find_syntax_by_name("Sass"),
                "sql" => SYNTAX_SET.find_syntax_by_name("SQL"),
                "md" | "markdown" => SYNTAX_SET.find_syntax_by_name("Markdown"),
                "toml" => SYNTAX_SET.find_syntax_by_name("TOML"),
                "dockerfile" => SYNTAX_SET.find_syntax_by_name("Dockerfile"),
                _ => Some(SYNTAX_SET.find_syntax_plain_text()),
            })
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
    }
    
    fn get_theme(theme_name: &str) -> &'static Theme {
        let theme_key = match theme_name.to_lowercase().as_str() {
            "dark" | "monokai" => "base16-monokai.dark",
            "light" | "inspired" => "InspiredGitHub",
            "ocean" => "base16-ocean.dark",
            "solarized-dark" => "Solarized (dark)",
            "solarized-light" => "Solarized (light)",
            _ => if theme_name.contains("dark") {
                "base16-monokai.dark"
            } else {
                "InspiredGitHub"
            }
        };
        
        THEME_SET.themes.get(theme_key)
            .unwrap_or_else(|| THEME_SET.themes.get("InspiredGitHub").unwrap())
    }
    
    pub fn highlight_line(&self, line: &str) -> Html {
        let mut h = HighlightLines::new(self.syntax, self.theme);
        
        if let Ok(ranges) = h.highlight_line(line, &SYNTAX_SET) {
            self.render_highlighted_ranges(&ranges, line)
        } else {
            // Fallback to plain text
            html! { <span>{ line }</span> }
        }
    }
    
    fn render_highlighted_ranges(&self, ranges: &[(Style, &str)], _original: &str) -> Html {
        let spans: Vec<Html> = ranges.iter().map(|(style, text)| {
            let color = format!(
                "color: rgb({}, {}, {});",
                style.foreground.r,
                style.foreground.g,
                style.foreground.b
            );
            
            let mut styles = vec![color];
            
            if style.font_style.contains(syntect::highlighting::FontStyle::BOLD) {
                styles.push("font-weight: bold;".to_string());
            }
            if style.font_style.contains(syntect::highlighting::FontStyle::ITALIC) {
                styles.push("font-style: italic;".to_string());
            }
            if style.font_style.contains(syntect::highlighting::FontStyle::UNDERLINE) {
                styles.push("text-decoration: underline;".to_string());
            }
            
            let style_str = styles.join(" ");
            
            html! {
                <span style={style_str}>{ text }</span>
            }
        }).collect();
        
        html! {
            <>{ for spans }</>
        }
    }
    
    pub fn highlight_line_for_diff(&self, line: &str, preserve_bg: bool) -> Html {
        // When in diff mode, we want to preserve background colors but apply syntax colors
        let mut h = HighlightLines::new(self.syntax, self.theme);
        
        if let Ok(ranges) = h.highlight_line(line, &SYNTAX_SET) {
            let spans: Vec<Html> = ranges.iter().map(|(style, text)| {
                // Only apply foreground color and font styles, not background
                let color = if preserve_bg {
                    // Use a slightly muted version of colors in diff mode for better contrast
                    format!(
                        "color: rgba({}, {}, {}, 0.95);",
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b
                    )
                } else {
                    format!(
                        "color: rgb({}, {}, {});",
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b
                    )
                };
                
                let mut styles = vec![color];
                
                if style.font_style.contains(syntect::highlighting::FontStyle::BOLD) {
                    styles.push("font-weight: 600;".to_string());
                }
                if style.font_style.contains(syntect::highlighting::FontStyle::ITALIC) {
                    styles.push("font-style: italic;".to_string());
                }
                
                let style_str = styles.join(" ");
                
                html! {
                    <span style={style_str}>{ text }</span>
                }
            }).collect();
            
            html! {
                <>{ for spans }</>
            }
        } else {
            html! { <span>{ line }</span> }
        }
    }
}

// Helper function for easy use in components
pub fn highlight_code_line(line: &str, language: &str, theme: &str, for_diff: bool) -> Html {
    let highlighter = SyntaxHighlighter::new(language, theme);
    
    if for_diff {
        highlighter.highlight_line_for_diff(line, true)
    } else {
        highlighter.highlight_line(line)
    }
}