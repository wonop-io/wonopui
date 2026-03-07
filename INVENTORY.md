# WonopUI Component Inventory - Shadcn Styling Migration

## Overview

This document tracks the progress of migrating WonopUI components to match shadcn/ui v4 styling standards with a premium look and feel.

### Shadcn v4 Key Patterns

1. **Focus States**: `focus-visible:border-zinc-950 focus-visible:ring-zinc-950/50 focus-visible:ring-[3px]`
2. **Disabled States**: `disabled:pointer-events-none disabled:opacity-50`
3. **Animations**: `data-[state=open]:animate-in data-[state=closed]:animate-out`
4. **Data Attributes**: `data-slot="component-name"`, `data-state="open|closed"`
5. **Shadow System**: `shadow-xs`, `shadow-sm`, `shadow-md`, `shadow-lg`
6. **Border Radius**: `rounded-md`, `rounded-lg`, `rounded-xl`
7. **Transitions**: `transition-all duration-200`
8. **Dark Mode**: Proper `dark:` variants with zinc color palette

### Status Legend

| Icon | Status | Description |
|------|--------|-------------|
| ✅ | Done | First pass complete |
| ✅✅ | Second Pass | Verified shadcn v4 compliance + premium styling |
| 🟡 | Partial | Needs more work |
| ❌ | Not Started | No shadcn styling applied |
| ⚪ | N/A | Layout/utility component, no visual styling needed |

---

## Gallery Components (64 total crates)

### Form Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 1 | Accordion | `wonopui-accordion` | ✅ | ✅✅ | v4 focus ring, data-state, proper button trigger |
| 2 | Button | `wonopui-button` | ✅ | ✅✅ | Added Link, Outline variants, Icon sizes |
| 3 | Calendar | `wonopui-calendar` | ✅ | ✅✅ | Premium styling, data-slots, nav buttons |
| 4 | Checkbox | `wonopui-checkbox` | ✅ | ✅✅ | data-state, focus-visible ring pattern |
| 5 | Collapsible | `wonopui-collapsible` | ✅ | ✅✅ | data-state, button trigger, aria-expanded |
| 6 | ColorPicker | `wonopui-color-picker` | ✅ | ✅✅ | **FIXED**: Trigger swatch + popover pattern |
| 7 | Combobox | `wonopui-combobox` | ✅ | ✅✅ | **FIXED**: Better item spacing (py-2.5, px-3, gap-2.5) |
| 8 | DatePicker | `wonopui-date-picker` | ✅ | ✅✅ | **FIXED**: Gallery example, re-exported component |
| 9 | Input | `wonopui-input` | ✅ | ✅✅ | v4 focus ring, shadow-xs, selection colors |
| 10 | Label | `wonopui-label` | ✅ | ✅✅ | **FIXED**: tracking-tight, leading-6 |
| 11 | MentionInput | `wonopui-mention-input` | ✅ | ✅✅ | **FIXED**: Better styling, z-[9999], improved gallery demo |
| 12 | Select | `wonopui-select` | ✅ | ✅✅ | Checkmark indicator, animations |
| 13 | Switch | `wonopui-switch` | ✅ | ✅✅ | **FIXED**: White thumb, zinc-300 track when checked |
| 14 | TagInput | `wonopui-tag-input` | ✅ | ✅✅ | Premium tag badges, v4 input styling |
| 15 | Textarea | `wonopui-textarea` | ✅ | ✅✅ | v4 focus ring, field-sizing-content |
| 16 | Toggle | `wonopui-toggle` | ✅ | ✅✅ | **FIXED**: rounded-lg, proper padding |

### Display Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 17 | Alert | `wonopui-alert` | ✅ | ✅✅ | CSS grid icon layout, data-slots |
| 18 | Avatar | `wonopui-avatar` | ✅ | ✅✅ | size-* classes, group support |
| 19 | Badge | `wonopui-badge` | ✅ | ✅✅ | rounded-full, v4 variants |
| 20 | Card | `wonopui-card` | ✅ | ✅✅ | rounded-xl, gap-6, CardDescription, CardFooter |
| 21 | Carousel | `wonopui-carousel` | ✅ | ✅✅ | Premium nav buttons, dots, ARIA roles |
| 22 | Divider | `wonopui-divider` | ✅ | ✅✅ | **FIXED**: Premium gradient (from-transparent via-zinc-300) |
| 23 | Placeholder | `wonopui-placeholder` | ⚪ | - | WonopUI-specific skeleton |
| 24 | Typography | `wonopui-typography` | ✅ | ✅✅ | **FIXED**: H1-H6 now pass correct HeadingLevel |

### Overlay Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 25 | Command | `wonopui-command` | ✅ | ✅✅ | **FIXED**: Better spacing (gap-3, px-3, py-2.5), larger icon |
| 26 | ContextMenu | `wonopui-context-menu` | ✅ | ✅✅ | data-slots, v4 animations, indicator styles |
| 27 | Dialog | `wonopui-dialog` | ✅ | ✅✅ | Separate overlay, v4 animations, data-state |
| 28 | Drawer | `wonopui-drawer` | ✅ | ✅✅ | v4 slide animations, drawer handle, data-slots |
| 29 | Dropdown | `wonopui-dropdown` | ✅ | ✅✅ | **FIXED**: z-[9999], gallery 3x3 grid demo for positioning |
| 30 | Notification | `wonopui-notification` | ✅ | ✅✅ | **FIXED**: More padding (p-5), shadow-xl |
| 31 | Popover | `wonopui-popover` | ✅ | ✅✅ | **FIXED**: z-[9999], rounded-xl, shadow-xl |

### Navigation Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 32 | Breadcrumb | `wonopui-breadcrumb` | ✅ | ✅✅ | data-slots, v4 link/page classes |
| 33 | Pagination | `wonopui-pagination` | ✅ | ✅✅ | **FIXED**: min-width for nav buttons, size-10 items |
| 34 | Tabs | `wonopui-tabs` | ✅ | ✅✅ | **FIXED**: Removed border, shadow-sm for active tab |

### Data Display Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 35 | DataTable | `wonopui-data-table` | ✅ | ✅✅ | **FIXED**: Pagination buttons now use class constants |
| 36 | Table | `wonopui-table` | ✅ | ✅✅ | data-slots, v4 cell styles, whitespace-nowrap |

### Layout Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 37 | Col | `wonopui-col` | ⚪ | - | Grid column |
| 38 | Container | `wonopui-container` | ⚪ | - | Layout wrapper |
| 39 | Content | `wonopui-content` | ⚪ | - | Content wrapper |
| 40 | Iframe | `wonopui-iframe` | ⚪ | - | Iframe wrapper |
| 41 | MulticolSidebar | `wonopui-multicol-sidebar` | ✅ | ✅✅ | **FIXED**: Premium styling, flex container, shadow-lg, header/footer classes |
| 42 | PageContent | `wonopui-page-content` | ✅ | ✅✅ | **FIXED**: p-6 md:p-8 lg:p-10, shadow-sm, rounded-xl |
| 43 | PageHeader | `wonopui-page-header` | ⚪ | - | Page header area |
| 44 | Resizable | `wonopui-resizable` | ✅ | ✅✅ | **FIXED**: Drag functionality using `use_mut_ref`, pointer capture, larger handles |
| 45 | Sidebar | `wonopui-sidebar` | ⚪ | - | Sidebar layout |
| 46 | Topbar | `wonopui-topbar` | ⚪ | - | Top navigation bar |

### Utility Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 47 | CopyButton | `wonopui-copy-button` | ✅ | ✅✅ | v4 outline button, icon states |
| 48 | DragPoint | `wonopui-drag-point` | ⚪ | - | Drag handle |
| 49 | GroupButton | `wonopui-group-button` | ✅ | ✅✅ | Toggle group styling, v4 focus ring |
| 50 | MediaQuery | `wonopui-media-query` | ⚪ | - | Responsive helper |
| 51 | Selectable | `wonopui-selectable` | ✅ | ✅✅ | **FIXED**: Zinc ring colors, ring-offset |
| 52 | WindowProvider | `wonopui-window-provider` | ⚪ | - | Context provider |
| 53 | BrowserProvider | `wonopui-browser-provider` | ⚪ | - | Browser feature detection context |
| 54 | DarkModeProvider | `wonopui-dark-mode-provider` | ⚪ | - | Dark mode context provider |
| 55 | ThemeProvider | `wonopui-theme-provider` | ⚪ | - | Theme/brand context provider |
| 56 | Layout | `wonopui-layout` | ⚪ | - | Layout state management provider |
| 57 | ComponentEditor | `wonopui-component-editor` | ⚪ | - | Component configuration editor |

### Specialized Components

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 58 | CodeEditor | `wonopui-code-editor` | ⚪ | - | Monaco-like editor |
| 59 | DiffView | `wonopui-diffview` | ⚪ | - | Diff viewer |
| 60 | Kanban | `wonopui-kanban` | ✅ | ✅✅ | **FIXED**: ColumnContext for auto column_id, drag indicator working |
| 61 | MarkdownEditor | `wonopui-markdown-editor` | ⚪ | - | Markdown editing |
| 62 | PaintCanvas | `wonopui-paint-canvas` | ⚪ | - | Drawing canvas |
| 63 | TailwindColorPicker | `wonopui-tailwind-color-picker` | ⚪ | - | Tailwind palette picker |

### Foundation

| # | Component | Crate | Status | Second Pass | Notes |
|---|-----------|-------|--------|-------------|-------|
| 64 | Core | `wonopui-core` | ⚪ | - | Core utilities, merge_classes, shared types |

---

## Progress Summary

| Category | Done | Pending | N/A |
|----------|------|---------|-----|
| Form Components (16) | **16** | 0 | 0 |
| Display Components (8) | **7** | 0 | 1 |
| Overlay Components (7) | **7** | 0 | 0 |
| Navigation Components (3) | **3** | 0 | 0 |
| Data Display Components (2) | **2** | 0 | 0 |
| Layout Components (10) | **3** | 0 | 7 |
| Utility Components (11) | **3** | 0 | 8 |
| Specialized Components (6) | **1** | 0 | 5 |
| Foundation (1) | **0** | 0 | 1 |
| **Total (64)** | **42** | **0** | **22** |

## Completed Fixes (Third Pass)

| # | Component | Fix Applied |
|---|-----------|-------------|
| 1 | ColorPicker | Redesigned with trigger swatch + popover pattern. |
| 2 | Combobox | Increased item padding (py-2.5, px-3, gap-2.5). |
| 3 | DatePicker | Fixed gallery example, re-exported component properly. |
| 4 | Label | Added tracking-tight, better line-height (leading-6). |
| 5 | MentionInput | Updated styling, increased z-index. |
| 6 | Switch | Fixed thumb to always be white, track now zinc-300 when checked. |
| 7 | Toggle | Added rounded-lg, proper padding (py-2, px-3). |
| 8 | Divider | Premium gradient effect (from-transparent via-zinc-300). |
| 9 | Typography | Fixed H1-H6 components to pass correct level. |
| 10 | Command | Increased spacing (gap-3, px-3, py-2.5), larger search icon. |
| 11 | Dropdown | z-[9999] for popover, better item spacing. |
| 12 | Notification | More padding (p-5), larger shadow-xl. |
| 13 | Pagination | min-width for nav buttons, size-10 items. |
| 14 | Tabs | Removed border, added shadow-sm for active tab. |
| 15 | PageContent | Added padding (p-6 md:p-8 lg:p-10), shadow-sm. |
| 16 | Selectable | Zinc ring colors with ring-offset. |

## Completed Fixes (Fourth Pass - Feb 2026)

| # | Component | Fix Applied |
|---|-----------|-------------|
| 17 | Resizable | Fixed drag not working. Changed from Closure-based window event listeners to direct pointer event handlers on handles. Using `use_mut_ref` for shared state. Added pointer capture. Larger handles (w-2, h-2) and visible grip indicators. |
| 18 | Kanban | Fixed indicator not working. Added `ColumnContext` to pass column_id automatically from `KanbanColumn` to child `KanbanCard` components. Cards now correctly know their parent column. |
| 19 | Dropdown | Gallery example updated with 3x3 grid layout demonstrating all 9 positioning options (North/South/East/West combinations). Better visualization of dropdown positions. |
| 20 | MulticolSidebar | Premium styling overhaul. Added `flex` to sidebar container, `shadow-lg`, cleaner borders. Added `COLUMN_HEADER` and `COLUMN_FOOTER` class constants. Updated gallery example with proper headers and icons. |
| 21 | MentionInput | Improved gallery example with interactive demo, clear instructions for usage, and proper autocomplete demonstration. |
| 22 | Iframe | **FIXED**: Added support for copying inline `<style>` tags to iframe (not just `<link>` CSS). Required for Trunk's `data-inline` CSS mode. Added `HtmlStyleElement` to web-sys features. |
| 23 | BlockPreview | **FIXED**: Added `position: relative` container for proper Resizable positioning. |
| 24 | Application Shells | **REWRITTEN v2**: Interactive demos with proper state management. Uses `LayoutProvider`/`LayoutContext` for sidebar folding and mobile menu state. Includes: (1) **Sidebar Folding Toggle** - demonstrates `folded` prop with toggle button, (2) **Mobile Responsive Sidebar** - slide-in overlay with backdrop, hamburger menu button, `lg:` breakpoint responsive behavior. Also: Stacked Layout, Multi-Column Layout. All use WonopUI `Sidebar`, `Topbar`, `Card`, etc. Complete code examples included. |
| 25 | Sidebar Links | **FIXED**: Added missing block categories to sidebar (Headings, Data Display, Lists). |
| 26 | Navigation Blocks | **REWRITTEN**: Now uses WonopUI `Topbar`, `TopbarStart`, `TopbarCenter`, `TopbarEnd`, `Button`, `Avatar`, `Input`, `Badge` components. 4 examples: Simple, Search, User Menu, Badge. |
| 27 | Page Examples | **REWRITTEN**: Now uses WonopUI `Card`, `CardHeader`, `CardTitle`, `CardContent`, `CardDescription`, `SwitchButton`, `Tabs`, `Button`, `Avatar`, `Badge`, `Input` components. Dashboard uses Card for stats, Settings uses SwitchButton for toggles. |
| 28 | Forms - Form Layouts | **NEW**: 4 form layouts (Stacked, Inline, Two-Column, Sectioned) using `Card`, `Input`, `Label`, `Textarea`, `Select`, `Checkbox`, `SwitchButton`, `Divider`. Sectioned form demonstrates notification and privacy settings patterns. |
| 29 | Forms - Input Groups | **NEW**: 5 input group patterns (Leading Icon, Trailing Icon, Text Addon, With Button, With Select) using proper icon placement and combined input controls. |
| 30 | Forms - Sign-in | **UPDATED**: Enhanced sign-in forms with icons, "Remember me" checkbox, social sign-in (Google/GitHub), password requirements hints. Uses `Card` wrapper with proper header/footer. |
| 31 | Navigation - Tabs | **NEW**: 3 tab examples (Simple, With Icons, In Card) using `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`. Demonstrates tab navigation within cards and with icon labels. |
| 32 | Navigation - Breadcrumbs | **NEW**: 3 breadcrumb examples (Simple, With Icon, In Page Header) using `Breadcrumb`, `BreadcrumbItem`. Shows integration with page headers. |
| 33 | Navigation - Pagination | **NEW**: 2 pagination examples (Simple, With Info) using `Pagination` component. Shows results count display pattern. |
| 34 | Lists - Tables | **NEW**: 4 table examples (Simple, With Actions, Striped, With Checkboxes) using `Table`, `TableHead`, `TableBody`, `TableRow`, `TableHeadCell`, `TableCell`, `TableFooter`. Demonstrates sorting, actions, striped rows, and selectable rows. |
| 35 | Data Display - Calendars | **NEW**: 3 calendar examples (Simple, With Selection, In Card) using `Calendar` component. Interactive date selection with state management. |
| 36 | Overlays - Slide-overs | **NEW**: 3 slide-over examples (Right, Form, Bottom Drawer) using `Drawer`, `DrawerProvider`, `DrawerTrigger`. Demonstrates panels from different directions. |
| 37 | Overlays - Notifications | **NEW**: 2 notification examples (Simple, With Action) using `NotificationProvider`, `use_notify`. Toast-style notifications with action buttons. |
| 38 | Feedback - Empty States | **NEW**: 3 empty state examples (Simple, With Action, In Card) for no-data scenarios. Uses icons, descriptions, and call-to-action buttons. |
| 39 | Feedback Blocks Rewrite | **REWRITTEN**: Complete rewrite of feedback/mod.rs. Now includes: (1) Dialogs - 3 examples (Simple, Confirm, Form) using `DialogProvider`/`Dialog`, (2) Slide-overs - 3 examples (Right, Form, Bottom Drawer) using `DrawerProvider`/`Drawer` with typed drawer enums, (3) Notifications - 2 examples (Simple, With Action) using `NotificationProvider`/`use_notify`, (4) Empty States - 3 examples (Simple, With Action, In Card). Total: 11 feedback patterns. |
| 40 | Elements - Button Groups | **NEW**: 3 button group patterns (Simple, With Icons, Vertical) using `GroupButton`, `GroupButtonTrigger`, `GroupButtonDirection`. |
| 41 | Elements - Dividers | **NEW**: 3 divider patterns (Simple, With Text, Content Sections) using `Divider` component. |
| 42 | Forms - Checkboxes | **NEW**: 3 checkbox patterns (Simple, Group, With Description) using `Checkbox`, `Label`. |
| 43 | Forms - Toggles & Switches | **NEW**: 3 toggle patterns (Switch Simple, Switch Group, Button Toggle) using `SwitchButton` and `Toggle`. |
| 44 | Code/Preview Matching | **FIXED**: Updated code constants in elements/mod.rs to exactly match rendered component implementations. |
| 45 | Build Fixes | **FIXED**: Added missing `wasm-bindgen` dependency to wonopui-markdown-editor BUILD.bazel. Added `tw-animate.css` to gallery tailwind_css srcs. |
| 46 | Iframe Auto-Height | **NEW**: Added `auto_height`, `on_height_change`, and `min_height` props to `Iframe` component. Uses MutationObserver to automatically adjust iframe height based on content. BlockPreview now uses auto-height by default instead of fixed 400px height. |
| 47 | Iframe Auto-Height v2 | **IMPROVED**: Added `ResizeObserver` for more reliable height detection. Multiple timeout retries (50ms, 150ms, 300ms, 500ms) to catch async content. When `on_height_change` callback provided, iframe only reports height (parent controls size). |
| 48 | BlockPreview Resize | **FIXED**: Separated width/height state to fix viewport buttons. Horizontal-only resizing (south=false, south_east=false). Height controlled by iframe auto-height. |

## Verification (Fifth Pass - Feb 26, 2026)

Full code review confirms all 55+ components follow shadcn/ui v4 styling:

### Verified Components
- All form inputs (Button, Input, Textarea, Checkbox, Switch, Select, Label, Toggle)
- All data display (Badge, Avatar, Card, Table, Alert, Divider, Pagination, Typography)
- All navigation (Tabs, Breadcrumb, Accordion, Collapsible, Sidebar, Topbar)
- All overlays (Dialog, Drawer, Popover, Dropdown, ContextMenu, Notification)
- All specialized inputs (Calendar, DatePicker, ColorPicker, TagInput, MentionInput, Command, Combobox, GroupButton)

### Styling Patterns Confirmed
1. **Focus Ring**: `focus-visible:ring-zinc-950/50 dark:focus-visible:ring-zinc-300/50 focus-visible:ring-[3px]`
2. **Data Attributes**: All components use `data-slot`, `data-state`, `data-variant`
3. **Color Scheme**: Zinc palette throughout (zinc-900/50 for dark, zinc-200/800 for borders)
4. **Animations**: `animate-in/animate-out` with `fade-in/fade-out`, `zoom-in/zoom-out`, `slide-in/slide-out`
5. **Shadows**: Consistent use of `shadow-xs`, `shadow-sm`, `shadow-md`, `shadow-lg`, `shadow-xl`
6. **Dark Mode**: All components have proper `dark:` variants

## All Visual Components Complete ✅

All 42 visual components have been migrated to shadcn v4 styling. The remaining 22 components are:
- Layout/utility components (no visual styling needed)
- Specialized components (Monaco editors, diff viewers, canvas, etc.)
- Context providers and hooks (DarkModeProvider, ThemeProvider, BrowserProvider, etc.)
- Foundation crate (wonopui-core)

---

## Status Summary (Feb 26, 2026)

| Area | Status | Notes |
|------|--------|-------|
| Component Rust Code | ✅ Complete | All 42 visual components updated |
| `data-slot` Attributes | ✅ Complete | All components have proper slots |
| Focus Ring Pattern | ✅ Complete | Using v4 zinc ring colors |
| Dark Mode Support | ✅ Complete | All `dark:` variants present |
| Animation Classes | ✅ Complete | `tw-animate-css` installed |
| Tailwind Config | ✅ Complete | Animation plugin configured |

---

## Required: Animation Support for WonopUI

WonopUI components use shadcn/ui v4 animation classes that require the `tw-animate-css` package. Without it, animations on Dialog, Popover, Dropdown, Select, Drawer, ContextMenu, and Notification components will not work.

### Animation Classes Used

| Class | Effect |
|-------|--------|
| `animate-in`, `animate-out` | Enter/exit animations |
| `fade-in-0`, `fade-out-0` | Fade from/to 0% opacity |
| `zoom-in-95`, `zoom-out-95` | Scale from/to 95% |
| `slide-in-from-top-2` | Slide in from top |
| `slide-in-from-bottom-2` | Slide in from bottom |

### Installation Options

#### Option 1: NPM (Recommended for npm/bun projects)

```bash
npm install -D tw-animate-css
# or
bun add -D tw-animate-css
```

Then in your CSS file:
```css
@import "tw-animate-css";
@import "tailwindcss";
```

#### Option 2: Manual Download (For Trunk/non-npm projects)

1. Download `tw-animate.css` from the CDN:
   ```bash
   curl -sL "https://unpkg.com/tw-animate-css@1.4.0/dist/tw-animate.css" -o tw-animate.css
   ```

2. Import it in your CSS file:
   ```css
   @import "./tw-animate.css";
   @import "tailwindcss";
   ```

#### Option 3: Tailwind v3 (Legacy)

For Tailwind CSS v3 projects, use `tailwindcss-animate` instead:

```bash
npm install -D tailwindcss-animate
```

```js
// tailwind.config.js
module.exports = {
  plugins: [require('tailwindcss-animate')]
}
```

---

## Gallery Blocks Inventory

### BlockPreview - FIXED ✅ (Feb 27, 2026)

The `BlockPreview` component in `examples/gallery/src/blocks/block_preview.rs` has been updated with auto-height support and horizontal-only resizing:

1. **Iframe isolation** - Each preview is wrapped in an `<Iframe>` component for proper style isolation
2. **Auto-height** - Iframe automatically adjusts height using both `ResizeObserver` and `MutationObserver` for reliable content height detection
3. **Horizontal resizing only** - Users can resize width using east (→) handle; height is controlled by iframe content
4. **Resizable viewport** - Uses `<Resizable>` for responsive viewport testing (Desktop 1200px, Tablet 768px, Mobile 375px)
5. **Dark mode works** - The `body_class={if *dark_mode { "dark" } else { "" }}` prop properly applies dark mode theming
6. **Layout blocks work** - Full-page layouts (sidebars, stacked layouts) can now be properly previewed without affecting the gallery

Implementation pattern:
```rust
// Separate state for width and height
let viewport_width = use_state(|| 1200.0_f64);
let iframe_height = use_state(|| min_height as f64);

// Coordinates derived from separate state (width controlled by resize, height by iframe)
let coordinates = (0., 0., *viewport_width, *iframe_height);

<Resizable 
    east={true} 
    south={false}
    south_east={false}
    coordinates={coordinates} 
    on_coordinates_change={on_coordinates_change}
>
    <Iframe 
        body_class={if *dark_mode { "dark" } else { "" }} 
        class="w-full rounded-lg"
        auto_height={true}
        min_height={min_height}
        on_height_change={on_height_change}
    >
        {iframe_content}
    </Iframe>
</Resizable>
```

**Key Design Decisions:**
- **Separate state** - `viewport_width` and `iframe_height` are separate state values to avoid circular updates
- **Horizontal only** - `south={false}` and `south_east={false}` - height is determined by content, not user drag
- **Viewport buttons work** - Desktop/Tablet/Mobile buttons directly update `viewport_width` state

**Props updated:**
- `height` → `min_height` (minimum height for the preview, defaults to 100px)

**Iframe auto-height improvements (Feb 27, 2026):**
- Added `ResizeObserver` in addition to `MutationObserver` for more reliable height detection
- When `on_height_change` callback is provided, iframe only reports height (parent controls actual size)
- Multiple timeout retries (50ms, 150ms, 300ms, 500ms) to catch async content like images and fonts
- Added `ResizeObserver`, `ResizeObserverEntry` to web-sys features

---

### Block Categories Status

| Category | blocks_old Count | New Gallery Status | Notes |
|----------|------------------|-------------------|-------|
| **Application Shells** | | | |
| └ Sidebar Folding | NEW | ✅ Done | Interactive toggle between normal/narrow sidebar using LayoutContext |
| └ Mobile Responsive | NEW | ✅ Done | Slide-in sidebar overlay with backdrop and hamburger menu |
| └ Stacked Layouts | 9 | ✅ Done | StackedLayoutExample with Topbar navigation |
| └ Multi-Column Layouts | 6 | ✅ Done | MultiColumnLayoutExample (3-column chat) |
| **Page Examples** | | | |
| └ Home Screens | 5+ | ✅ Done | DashboardExample using Card, Avatar, Input, Button |
| └ Detail Screens | 5+ | ✅ Done | UserProfileExample using Card, Tabs, Badge, Avatar |
| └ Settings Screens | 2+ | ✅ Done | SettingsExample using Card, SwitchButton, Input, Button |
| **Headings** | | | |
| └ Page Headings | 13 | ✅ Done | 3 blocks: Simple, Metadata, Profile |
| └ Card Headings | 6 | ✅ Done | 3 blocks: Simple, Description, Actions |
| └ Section Headings | 10 | ✅ Done | 4 blocks: Simple, Centered, Action, Tabs |
| **Data Display** | | | |
| └ Description Lists | 7 | ✅ Done | 3 blocks: Stacked, Two-Column, Card |
| └ Stats | 5 | ✅ Done | 4 blocks: Simple, Trend, Cards, Progress |
| └ Calendars | 8 | ✅ Done | 3 blocks: Simple, With Selection, In Card |
| **Lists** | | | |
| └ Stacked Lists | 17 | ✅ Done | 3 blocks: Simple, Avatars, Actions |
| └ Tables | 20 | ✅ Done | 4 blocks: Simple, With Actions, Striped, With Checkboxes |
| └ Grid Lists | 7 | ✅ Done | 2 blocks: Card Grid, Image Grid |
| └ Feeds | 3 | ✅ Done | 2 blocks: Activity Feed, Comment Thread |
| **Forms** | | | |
| └ Form Layouts | 5 | ✅ Done | 4 layouts: Stacked, Inline, Two-Column, Sectioned |
| └ Input Groups | 21 | ✅ Done | 5 patterns: Leading Icon, Trailing Icon, Addon, Button, Select |
| └ Select Menus | 7 | ❌ Not Started | |
| └ Sign-in/Registration | 5 | ✅ Done | 3 forms: Sign-in, Registration, Social Sign-in |
| └ Textareas | 5 | ❌ Not Started | |
| └ Radio Groups | 12 | ❌ Not Started | |
| └ Checkboxes | 4 | ✅ Done | 3 blocks: Simple, Group, With Description |
| └ Toggles | 5 | ✅ Done | 3 blocks: Switch Simple, Switch Group, Button Toggle |
| └ Action Panels | 8 | ❌ Not Started | |
| └ Comboboxes | 5 | ❌ Not Started | |
| **Feedback** | | | |
| └ Dialogs | 6 | ✅ Done | 3 patterns: Simple, Confirm, Form (using DialogProvider) |
| └ Slide-overs | 12 | ✅ Done | 3 patterns: Right, Form, Bottom Drawer (using DrawerProvider) |
| └ Notifications | 6 | ✅ Done | 2 patterns: Simple, With Action (using use_notify) |
| └ Empty States | 6 | ✅ Done | 3 patterns: Simple, With Action, In Card |
| └ Alerts | 6 | ✅ Done | 4 variants (Success, Error, Warning, Info) |
| **Navigation** | | | |
| └ Navbars | 11 | ✅ Done | 4 examples using WonopUI Topbar (Simple, Search, User Menu, Badge) |
| └ Tabs | 9 | ✅ Done | 3 examples: Simple, With Icons, In Card |
| └ Breadcrumbs | 4 | ✅ Done | 3 examples: Simple, With Icon, In Page Header |
| └ Pagination | 3 | ✅ Done | 2 examples: Simple, With Info |
| └ Vertical Navigation | 6 | ❌ Not Started | |
| └ Sidebar Navigation | 5 | ❌ Not Started | |
| └ Progress Bars | 8 | ❌ Not Started | |
| └ Command Palettes | 9 | ❌ Not Started | |
| **Overlays** | | | |
| └ Dialogs | 6 | ✅ Done | 3 dialog patterns (simple, confirm, form) |
| └ Slide-overs | 12 | ✅ Done | 3 blocks: Right, Form, Bottom Drawer |
| └ Notifications | 6 | ✅ Done | 2 blocks: Simple, With Action |
| **Marketing** | | | |
| └ Hero Sections | 4 | ✅ Done | Centered and Split hero patterns |
| └ Feature Grid | 3 | ✅ Done | 6-feature grid with icons |
| └ Pricing | 3 | ✅ Done | 3-tier pricing table |
| └ Team | 2 | ✅ Done | Team member grid |
| **Elements** | | | |
| └ Avatars | 11 | ✅ Done | 3 sizes (Small, Medium, Large) |
| └ Badges | 18 | ✅ Done | 5 variants (Default, Success, Warning, Error, Info) |
| └ Dropdowns | 5 | ❌ Not Started | |
| └ Buttons | 10 | ✅ Done | 4 variants (Primary, Secondary, Success, Danger) |
| └ Button Groups | 5 | ✅ Done | 3 patterns: Simple, With Icons, Vertical |
| └ Dividers | 8 | ✅ Done | 3 patterns: Simple, With Text, Content Sections |
| **Layout** | | | |
| └ Containers | 5 | ❌ Not Started | |
| └ Panels | 10 | ❌ Not Started | |
| └ List Containers | 7 | ❌ Not Started | |
| └ Media Objects | 8 | ❌ Not Started | |

### App Screens (from blocks_old/src/pages/app_screens/)

These are full application screen examples that demonstrate WonopUI in real-world scenarios:

| Vertical | Screens | Status |
|----------|---------|--------|
| AI GPU Marketplace | 13 screens | ❌ Not Started |
| AI Fitness | 9 screens | ❌ Not Started |
| AI Logo Designer | 4 screens | ❌ Not Started |
| E-Commerce | 4 screens | ❌ Not Started |
| Education | 4 screens | ❌ Not Started |
| Finance | 3 screens | ❌ Not Started |
| Healthcare | 4 screens | ❌ Not Started |
| Social Media | 4 screens | ❌ Not Started |
| Project Services | 9 screens | ❌ Not Started |
| Shopping App | 8 screens | ❌ Not Started |
| Landing Page | 5 screens | ❌ Not Started |
| Generic Screens | 18 screens | ❌ Not Started |

### UI App Components (from blocks_old/src/pages/ui_app/)

Component showcase examples:

| Component | Status | Notes |
|-----------|--------|-------|
| Accordion | ❌ | Has existing gallery example |
| Alert | ❌ | Has existing gallery example |
| Avatar | ❌ | Has existing gallery example |
| Badge | ❌ | Has existing gallery example |
| Breadcrumb | ❌ | Has existing gallery example |
| Button | ❌ | Has existing gallery example |
| Card | ❌ | Has existing gallery example |
| Carousel | ❌ | Has existing gallery example |
| Checkbox | ❌ | Has existing gallery example |
| Combobox | ❌ | Has existing gallery example |
| Command | ❌ | Has existing gallery example |
| Dialog | ❌ | Has existing gallery example |
| Drawer | ❌ | Has existing gallery example |
| Notification | ❌ | Has existing gallery example |
| Popover | ❌ | Has existing gallery example |
| Select | ❌ | Has existing gallery example |
| Tabs | ❌ | Has existing gallery example |
| Toggle | ❌ | Has existing gallery example |

---

### Blocks Progress Summary

| Status | Count | Description |
|--------|-------|-------------|
| ✅ Complete | 32 | Fully ported with Iframe isolation |
| 🟡 Partial | 0 | Basic implementation, needs more variants |
| ❌ Not Started | 7+ | Not yet ported |

**Total blocks in blocks_old**: ~300+
**Total blocks in new gallery**: ~109+ (with proper Iframe isolation)

**Completed Categories:**
- Application Shells: Sidebar Folding Toggle, Mobile Responsive, Stacked, Multi-Column (4 blocks) - Interactive demos with LayoutProvider/LayoutContext state management
- Page Examples: Dashboard, User Profile, Settings (3 blocks) - using WonopUI Card, SwitchButton, Tabs, Avatar
- Elements: Buttons (4), Badges (5), Avatars (3), Alerts (4), Button Groups (3), Dividers (3) = 22 element patterns
- Forms: Form Layouts (4), Input Groups (5), Sign-in/Registration (3), Checkboxes (3), Toggles (3) = 18 form patterns
- Navigation: Navbars (4), Tabs (3), Breadcrumbs (3), Pagination (2) = 12 navigation patterns
- Feedback: Dialogs (3), Slide-overs (3), Notifications (2), Empty States (3), Alerts (4) = 15 overlay/feedback patterns
- Marketing: Hero, Features, Pricing, Team (5 blocks)
- Headings: Page Headings (3), Section Headings (4), Card Headings (3) = 10 blocks
- Data Display: Stats (4), Description Lists (3), Calendars (3) = 10 blocks
- Lists: Stacked Lists (3), Grid Lists (2), Feeds (2), Tables (4) = 11 blocks

**Total Gallery Blocks: ~109 blocks**
- Added Feb 27: Button Groups (3), Dividers (3), Checkboxes (3), Toggles/Switches (3) = 12 new blocks

---

## Second Pass Checklist

For each component, the second pass verifies:

1. **shadcn v4 Compliance**
   - [x] Uses correct focus-visible pattern with ring
   - [x] Uses `data-slot` attributes for component identification
   - [x] Uses `data-state` for stateful components
   - [x] Has proper disabled states
   - [x] Uses correct shadow classes (`shadow-xs`, `shadow-sm`)
   - [x] Has proper dark mode support with zinc palette

2. **Premium Look & Feel**
   - [x] Smooth transitions (`transition-all duration-200`)
   - [x] Subtle hover effects
   - [x] Proper spacing and padding
   - [x] High contrast and readability
   - [x] Professional animations

3. **Code Quality**
   - [x] Clean, consistent class organization
   - [x] Documented class constants
   - [x] Proper ARIA attributes

---

## Second Pass Log

| Date | Component | Changes Made |
|------|-----------|--------------|
| 2024-02-22 | Button | Added Link variant, v4 focus ring, icon sizes |
| 2024-02-22 | Input | v4 focus ring pattern, shadow-xs |
| 2024-02-22 | Textarea | v4 focus ring pattern |
| 2024-02-22 | Checkbox | data-state, v4 styling |
| 2024-02-22 | Switch | data-state, v4 styling |
| 2024-02-22 | Select | Checkmark, animations |
| 2024-02-22 | Label | v4 disabled states |
| 2024-02-22 | Toggle | data-state, v4 sizes |
| 2024-02-22 | Badge | rounded variants |
| 2024-02-22 | Card | rounded-xl, data-slots |
| 2024-02-22 | Alert | data-slots, variants |
| 2024-02-22 | Avatar | size classes |
| 2024-02-22 | Divider | data-orientation |
| 2024-02-22 | Typography | prose variants |
| 2024-02-22 | Dialog | v4 overlay, animations |
| 2024-02-22 | Drawer | v4 animations, handle |
| 2024-02-22 | Popover | v4 animations |
| 2024-02-22 | Dropdown | v4 focus, shortcuts |
| 2024-02-22 | ContextMenu | data-slots, indicators |
| 2024-02-22 | Notification | v4 toast styles |
| 2024-02-22 | Tabs | data-state, orientation |
| 2024-02-22 | Accordion | v4 trigger, animations |
| 2024-02-22 | Breadcrumb | data-slots |
| 2024-02-22 | Pagination | v4 variants |
| 2024-02-22 | Table | data-slots, v4 cells |
| 2024-02-22 | DataTable | v4 cell styles |
| 2024-02-22 | Calendar | data-slots, premium nav, ARIA |
| 2024-02-22 | Carousel | data-slots, dots, ARIA roles |
| 2024-02-22 | Combobox | check icons, data-slots |
| 2024-02-22 | Command | premium rounded-xl, data-slots |
| 2024-02-22 | Collapsible | data-state, button trigger |
| 2024-02-22 | Resizable | data-slots, grip indicators |
| 2024-02-22 | DatePicker | v4 input, calendar styling |
| 2024-02-22 | TagInput | v4 input, tag badges |
| 2024-02-22 | MentionInput | v4 dropdown styling |
| 2024-02-22 | ColorPicker | premium container |
| 2024-02-22 | CopyButton | v4 outline button |
| 2024-02-22 | GroupButton | toggle group styling |

---

## Next Steps - Priority Order

### Completed (Feb 26, 2026)
- ✅ Forms - Form Layouts (4 patterns)
- ✅ Forms - Input Groups (5 patterns)
- ✅ Navigation - Tabs (3 examples)
- ✅ Navigation - Breadcrumbs (3 examples)
- ✅ Navigation - Pagination (2 examples)
- ✅ Lists - Tables (4 examples)
- ✅ Data Display - Calendars (3 examples)
- ✅ Overlays - Slide-overs (3 examples)
- ✅ Overlays - Notifications (2 examples)
- ✅ Feedback - Empty States (3 examples)

### Remaining Work

| Priority | Category | Subcategory | Est. Effort | Components to Use |
|----------|----------|-------------|-------------|-------------------|
| 🟢 LOW | Forms | Radio Groups | Medium | Raw `<input type="radio">` (no component) |
| ✅ DONE | Forms | Checkboxes | Small | `Checkbox` - 3 patterns |
| ✅ DONE | Forms | Toggles | Small | `Switch` or `Toggle` - 3 patterns |
| 🟢 LOW | Navigation | Command Palettes | Medium | `Command` |
| 🟢 LOW | Navigation | Progress Bars | Small | Custom (no component yet) |
| 🟢 LOW | Elements | Dropdowns | Small | `Dropdown` |
| ✅ DONE | Elements | Button Groups | Small | `GroupButton` - 3 patterns |
| 🟢 LOW | Layout | Containers | Small | `Container` (already exists) |
| 🟢 LOW | Layout | Panels | Medium | `Card` |
| ✅ DONE | Elements | Dividers | Small | `Divider` - 3 patterns |

### Medium Priority: Component Styling Verification

Components that may need final visual polish to match shadcn exactly:

1. **Sidebar** (`wonopui-sidebar`) - Currently marked N/A but could have shadcn styling for header/footer borders
2. **Topbar** (`wonopui-topbar`) - Could have better shadow/border styling
3. **Marketing blocks** - Verify Card usage is consistent

### Low Priority: App Screens

Full application screen examples (85+ screens from blocks_old) - these are large, showcase-style demos:

- AI GPU Marketplace (13 screens)
- AI Fitness (9 screens)
- E-Commerce (4 screens)
- Finance (3 screens)
- Healthcare (4 screens)
- etc.

---

## Key Shadcn v4 Patterns Applied

### Focus Ring Pattern
```rust
focus-visible:border-zinc-950 
dark:focus-visible:border-zinc-300 
focus-visible:ring-zinc-950/50 
dark:focus-visible:ring-zinc-300/50 
focus-visible:ring-[3px] 
focus-visible:outline-none
```

### Button Pattern
```rust
inline-flex items-center justify-center gap-2 whitespace-nowrap 
text-sm font-medium transition-all duration-200 
disabled:pointer-events-none disabled:opacity-50 
[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4
```

### Popover/Dropdown Pattern
```rust
overflow-hidden rounded-xl border border-zinc-200 bg-white 
text-zinc-950 shadow-lg 
dark:border-zinc-800 dark:bg-zinc-950 dark:text-zinc-50 
data-[state=open]:animate-in data-[state=closed]:animate-out 
data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 
data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95
```

### Input Pattern
```rust
flex h-9 w-full rounded-md border border-zinc-200 bg-transparent 
px-3 py-2 text-sm shadow-xs transition-all duration-200 
placeholder:text-zinc-500 
focus-visible:border-zinc-950 focus-visible:ring-zinc-950/50 
focus-visible:ring-[3px] focus-visible:outline-none 
disabled:cursor-not-allowed disabled:opacity-50 
dark:border-zinc-800 dark:placeholder:text-zinc-400 
dark:focus-visible:border-zinc-300 dark:focus-visible:ring-zinc-300/50
```
