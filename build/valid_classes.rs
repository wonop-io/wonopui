// build.rs
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn is_valid_tailwind_class(class: &str) -> bool {
    if class.ends_with(":") || class.ends_with("-") {
        return false;
    }
    let class = if class.starts_with("-") {
        &class[1..]
    } else {
        class
    };
    // Define a comprehensive list of Tailwind CSS prefixes and standalone classes
    let valid_prefixes = [
        // Layout
        "container",
        "box-",
        "block",
        "inline",
        "flex",
        "grid",
        "table",
        "hidden",
        // Flexbox & Grid
        "flex-",
        "grid-",
        "order-",
        "col-",
        "row-",
        "gap-",
        "justify-",
        "items-",
        "content-",
        "place-",
        // Spacing
        "p-",
        "px-",
        "py-",
        "pt-",
        "pr-",
        "pb-",
        "pl-",
        "m-",
        "mx-",
        "my-",
        "mt-",
        "mr-",
        "mb-",
        "ml-",
        "space-",
        // Sizing
        "w-",
        "min-w-",
        "max-w-",
        "h-",
        "min-h-",
        "max-h-",
        // Typography
        "font-",
        "text-",
        "leading-",
        "tracking-",
        "whitespace-",
        "break-",
        "truncate",
        "indent-",
        "list-",
        "align-",
        "uppercase",
        "lowercase",
        "capitalize",
        "normal-case",
        // Backgrounds
        "bg-",
        "from-",
        "via-",
        "to-",
        "gradient-",
        // Borders
        "border",
        "border-",
        "rounded",
        "rounded-",
        "divide-",
        "ring-",
        "ring-offset-",
        // Effects
        "shadow-",
        "opacity-",
        "mix-blend-",
        "blur-",
        "brightness-",
        "contrast-",
        "grayscale-",
        "hue-rotate-",
        "invert-",
        "saturate-",
        "sepia-",
        // Filters
        "filter",
        "backdrop-",
        // Tables
        "table-",
        // Transitions & Animation
        "transition-",
        "duration-",
        "ease-",
        "delay-",
        "animate-",
        // Transforms
        "scale-",
        "rotate-",
        "translate-",
        "skew-",
        "origin-",
        "transform",
        // Interactivity
        "cursor-",
        "select-",
        "resize-",
        "scroll-",
        "snap-",
        "touch-",
        "user-",
        "pointer-events-",
        "appearance-",
        "outline-",
        "caret-",
        // SVG
        "fill-",
        "stroke-",
        // Accessibility
        "sr-",
        "not-sr-",
        // Variants
        "hover:",
        "focus:",
        "active:",
        "group-hover:",
        "focus-within:",
        "focus-visible:",
        "disabled:",
        "dark:",
        "sm:",
        "md:",
        "lg:",
        "xl:",
        "2xl:",
        "first:",
        "last:",
        "odd:",
        "even:",
        "visited:",
        "checked:",
        "indeterminate:",
        "default:",
        "required:",
        "valid:",
        "invalid:",
        "in-range:",
        "out-of-range:",
        "placeholder-shown:",
        "autofill:",
        "read-only:",
        // Display
        "inline-",
        "flow-",
        // Position
        "static",
        "fixed",
        "absolute",
        "relative",
        "sticky",
        "top-",
        "right-",
        "bottom-",
        "left-",
        "inset-",
        // Visibility
        "visible",
        "invisible",
        // Z-index
        "z-",
        // Overflow
        "overflow-",
        // Float
        "float-",
        "clear-",
        // Object fit
        "object-",
        // Aspect ratio
        "aspect-",
        // Columns
        "columns-",
        // Break
        "break-",
        // Additional cases
        "prose",
        "prose-",
        "underline",
        "overline",
        "line-through",
        "no-underline",
        "antialiased",
        "subpixel-antialiased",
        "italic",
        "not-italic",
        "ordinal",
        "slashed-zero",
        "lining-nums",
        "oldstyle-nums",
        "proportional-nums",
        "tabular-nums",
        "diagonal-fractions",
        "stacked-fractions",
        "overscroll-",
        "scroll-",
        "hyphens-",
        "write-",
        "accent-",
        "decoration-",
        "placeholder-",
        "backdrop-",
        "will-change-",
        "content-",
        // Additional prefixes to cover all cases
        "group",
        "peer",
        "motion-",
        "print:",
        "rtl:",
        "ltr:",
        "open:",
        "closed:",
        "file:",
        "dir:",
        "before:",
        "after:",
        "marker:",
        "selection:",
        "first-of-type:",
        "last-of-type:",
        "only-of-type:",
        "only-child:",
        "empty:",
        "target:",
        "enabled:",
        "focus-visible:",
        "optional:",
        "placeholder:",
        "read-write:",
        "landscape:",
        "portrait:",
        "motion-safe:",
        "motion-reduce:",
        "contrast-more:",
        "contrast-less:",
        "3xl:",
        "4xl:",
        "5xl:",
        "6xl:",
        "7xl:",
        "8xl:",
        "9xl:",
        "2xs:",
        "xs:",
        "supports-",
        "not-",
        "group-",
        "peer-",
        "all:",
        "children:",
        "siblings:",
        "sibling:",
    ];

    // Check if the class starts with any valid prefix or is a valid standalone class
    valid_prefixes
        .iter()
        .any(|&prefix| class.starts_with(prefix) || class == prefix.trim_end_matches('-'))
}