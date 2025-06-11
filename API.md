# Wonop UI Components

A comprehensive reference guide to the available components in Wonop UI.

## Accordion

The Accordion component is a collapsible section of content that allows users to toggle the visibility of content sections, providing an efficient way to organize and present information.

### Usage

```rust
<Accordion title="Accordion example">
    <p>This is the content of the accordion. It can be expanded or collapsed.</p>
</Accordion>
```

### Props

| Prop     | Type     | Description                                                    |
| -------- | -------- | -------------------------------------------------------------- |
| title    | String   | The title of the accordion section                             |
| children | Children | The child elements to be rendered inside the accordion content |
| class    | Classes  | Additional CSS classes to apply to the accordion container     |

### Styling Classes

| Class Name          | Description                                       |
| ------------------- | ------------------------------------------------- |
| accordion_container | For the main container of the accordion component |
| accordion_header    | For the header section of the accordion           |
| accordion_title     | For the title text within the accordion header    |
| accordion_content   | For the content container of the accordion        |

## Alert

The Alert component is used to display important messages to the user. It supports different types of alerts such as info, success, warning, and error.

### Usage

```rust
<Alert alert_type={AlertType::Info}>
    <p>This is an info alert</p>
</Alert>

<Alert alert_type={AlertType::Success}>
    <p>This is a success alert</p>
</Alert>

<Alert alert_type={AlertType::Warning}>
    <p>This is a warning alert</p>
</Alert>

<Alert alert_type={AlertType::Error}>
    <p>This is an error alert</p>
</Alert>
```

### Props

| Prop       | Type         | Description                                                       |
| ---------- | ------------ | ----------------------------------------------------------------- |
| alert_type | AlertType    | The type of alert to be displayed (Info, Success, Warning, Error) |
| icon       | Option<Html> | An optional icon to be displayed in the alert                     |
| children   | Children     | Child elements to be rendered inside the alert                    |
| class      | Classes      | Additional CSS classes to apply to the alert container            |

### Styling Classes

| Class Name    | Description                       |
| ------------- | --------------------------------- |
| alert_base    | Base styles for all alert types   |
| alert_success | Styles specific to success alerts |
| alert_warning | Styles specific to warning alerts |
| alert_error   | Styles specific to error alerts   |
| alert_info    | Styles specific to info alerts    |

## Avatar

The Avatar component is used to display user profile images in different sizes. It supports small, medium, and large sizes, and allows for customization of the image URL and alt text.

### Usage

```rust
<Avatar src="https://example.com/avatar.jpg" alt="Small Avatar" size={AvatarSize::Small} />
<Avatar src="https://example.com/avatar.jpg" alt="Medium Avatar" size={AvatarSize::Medium} />
<Avatar src="https://example.com/avatar.jpg" alt="Large Avatar" size={AvatarSize::Large} />
```

### Props

| Prop | Type       | Description                                      |
| ---- | ---------- | ------------------------------------------------ |
| src  | String     | The URL of the image to be displayed             |
| alt  | String     | The alt text for the image                       |
| size | AvatarSize | The size of the avatar (Small, Medium, or Large) |

### Styling Classes

| Class Name    | Description                      |
| ------------- | -------------------------------- |
| avatar_base   | Base styles for all avatar sizes |
| avatar_small  | Styles for small avatars         |
| avatar_medium | Styles for medium avatars        |
| avatar_large  | Styles for large avatars         |


## Badge

The Badge component is used to display a small, inline element with a short label. It's often used to highlight status, counts, or categories in a compact and visually distinct way.

### Usage

```rust
<Badge label="New" badge_type={BadgeType::Success} />
<Badge label="Warning" badge_type={BadgeType::Warning} />
<Badge label="Error" badge_type={BadgeType::Error} />
<Badge label="Info" badge_type={BadgeType::Info} />
<Badge label="Default" badge_type={BadgeType::Default} />
```

### Props

| Prop       | Type      | Description                                                          |
| ---------- | --------- | -------------------------------------------------------------------- |
| label      | String    | The text to be displayed inside the badge                            |
| badge_type | BadgeType | The type of badge to be displayed (Success, Warning, Error, Info, Default) |

### Styling Classes

| Class Name     | Description                         |
| -------------- | ----------------------------------- |
| badge_base     | Base styling applied to all badges  |
| badge_success  | Styling for success-type badges     |
| badge_warning  | Styling for warning-type badges     |
| badge_error    | Styling for error-type badges       |
| badge_info     | Styling for info-type badges        |
| badge_default  | Styling for default-type badges     |

## Breadcrumb

The Breadcrumb component is used to display a navigation trail for users. It helps users understand their location within the hierarchy of a website or application and provides easy navigation to parent pages.

### Usage

```rust
<Breadcrumb>
    <BreadcrumbItem label="Home" href="#" />
    <BreadcrumbItem label="Library" href="#" />
    <BreadcrumbItem label="Data" />
</Breadcrumb>
```

### Props

#### Breadcrumb

| Prop           | Type         | Description                                                      |
| -------------- | ------------ | ---------------------------------------------------------------- |
| children       | Children     | The child elements to be rendered inside the breadcrumb component |
| separator_icon | Option<Html> | An optional custom separator icon between breadcrumb items       |

#### BreadcrumbItem

| Prop  | Type           | Description                                                                       |
| ----- | -------------- | --------------------------------------------------------------------------------- |
| label | String         | The text to be displayed for the breadcrumb item                                  |
| href  | Option<String> | An optional URL for the breadcrumb item. If provided, the item will be rendered as a link |

### Styling Classes

| Class Name          | Description                                      |
| ------------------- | ------------------------------------------------ |
| breadcrumb_nav      | For the main navigation container of the breadcrumb |
| breadcrumb_list     | For the ordered list containing breadcrumb items |
| breadcrumb_item     | For individual breadcrumb items                  |
| breadcrumb_separator | For the separator between breadcrumb items      |

## Button

The Button component is a versatile and customizable button element that can be used in various parts of your application. It supports different variants, sizes, and can handle click events.

### Usage

```rust
<Button variant={ButtonVariant::Primary} onclick={Callback::from(|_| console::log!("Primary clicked!"))}>
    {"Primary"}
</Button>

<Button variant={ButtonVariant::Secondary} onclick={Callback::from(|_| console::log!("Secondary clicked!"))}>
    {"Secondary"}
</Button>

<Button variant={ButtonVariant::Primary} size={ButtonSize::Small}>
    {"Small"}
</Button>
```

### Props

| Prop     | Type              | Description                                                             |
| -------- | ----------------- | ----------------------------------------------------------------------- |
| onclick  | Callback<MouseEvent> | The callback to be executed when the button is clicked               |
| variant  | ButtonVariant     | The variant of the button (Primary, Secondary, Success, Warning, Danger, Ghost, Default) |
| size     | ButtonSize        | The size of the button (Small, Medium, Large)                           |
| class    | Classes           | Additional CSS classes to be applied to the button                       |
| children | Children          | The child elements to be rendered inside the button                      |
| disabled | bool              | Whether the button is disabled or not                                    |
| kind     | Option<String>    | The type of the button, defaults to 'button' if not specified           |

### Styling Classes

| Class Name       | Description                            |
| ---------------- | -------------------------------------- |
| button_base      | Base styles applied to all buttons     |
| button_primary   | Styles for primary buttons             |
| button_secondary | Styles for secondary buttons           |
| button_success   | Styles for success buttons             |
| button_warning   | Styles for warning buttons             |
| button_danger    | Styles for danger buttons              |
| button_ghost     | Styles for ghost buttons               |
| button_default   | Styles for default buttons             |
| button_small     | Styles for small-sized buttons         |
| button_medium    | Styles for medium-sized buttons        |
| button_large     | Styles for large-sized buttons         |


## Calendar

The Calendar component displays a monthly calendar view, allowing users to select a date. It provides a flexible and responsive interface for date selection tasks.

### Usage

```rust
<Calendar
    year={today.year()}
    month={today.month()}
    on_date_click={on_date_click.clone()}
    selected_date={selected_date.map(|(y, m, d)| chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap_or_default())}
/>
```

### Props

| Prop          | Type                       | Description                                                                                           |
| ------------- | -------------------------- | ----------------------------------------------------------------------------------------------------- |
| year          | i32                        | The year to display. Defaults to the current year if 0.                                               |
| month         | u32                        | The month to display (1-12). Defaults to the current month if 0.                                      |
| on_date_click | Callback<(i32, u32, u32)>  | A callback function that is called when a date is clicked, with the year, month, and day.             |
| selected_date | Option<NaiveDate>          | The currently selected date, if any.                                                                  |

### Styling Classes

| Class Name           | Description                                 |
| -------------------- | ------------------------------------------- |
| calendar_container   | Main container of the calendar component    |
| calendar_wrapper     | Wrapper for the calendar content            |
| calendar_header      | Header section of the calendar              |
| calendar_title       | Title area containing month/year display    |
| calendar_month_year  | Display for current month and year          |
| calendar_nav         | Container for navigation buttons            |
| calendar_grid        | Grid layout for the calendar days           |
| calendar_thead       | Header of the calendar table                |
| calendar_weekdays    | Row containing weekday abbreviations        |
| calendar_weekday     | Individual weekday cell                     |
| calendar_tbody       | Body of the calendar table                  |
| calendar_week        | Row representing a week                     |
| calendar_day         | Cell for each day in the calendar           |
| calendar_day_button  | Button for selecting a specific day         |
| calendar_day_selected | Styling for the selected day               |

## Card

The Card component is a versatile container that can hold various types of content. It is composed of several subcomponents to provide a flexible and customizable experience.

### Usage

```rust
<Card>
    <CardHeader>
        <CardTitle>{"Card Title"}</CardTitle>
    </CardHeader>
    <CardContent>
        {"This is the card content."}
    </CardContent>
</Card>
```

### Props

#### Card

| Prop     | Type                          | Description                                                 |
| -------- | ----------------------------- | ----------------------------------------------------------- |
| children | Children                      | The child elements to be rendered inside the card.          |
| class    | Classes                       | Additional CSS classes to apply to the card.                |
| onclick  | Option<Callback<MouseEvent>>  | Optional callback for click events on the card.             |

#### CardHeader

| Prop     | Type     | Description                                               |
| -------- | -------- | --------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the card header. |
| class    | Classes  | Additional CSS classes to apply to the card header.       |

#### CardTitle

| Prop     | Type     | Description                                              |
| -------- | -------- | -------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the card title. |

#### CardContent

| Prop     | Type     | Description                                                |
| -------- | -------- | ---------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the card content. |

### Styling Classes

| Class Name     | Description                                          |
| -------------- | ---------------------------------------------------- |
| card_container | Styles for the main card container                   |
| card_header    | Styles for the card header section                   |
| card_title     | Styles for the card title                            |
| card_body      | Styles for the card content area                     |

## Carousel

The Carousel component is a dynamic slideshow for cycling through a series of content. It provides an engaging way to display multiple items in a limited space, with automatic cycling and manual navigation options.

### Usage

```rust
<Carousel interval={3000}>
    <CarouselItem>
        <div>{"Item 1"}</div>
    </CarouselItem>
    <CarouselItem>
        <div>{"Item 2"}</div>
    </CarouselItem>
    <CarouselItem>
        <div>{"Item 3"}</div>
    </CarouselItem>
</Carousel>
```

### Props

#### Carousel

| Prop     | Type     | Description                                                           |
| -------- | -------- | --------------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the carousel.                |
| interval | u32      | The interval in milliseconds for cycling through the items.           |
| next     | Html     | Optional custom next control button.                                  |
| prev     | Html     | Optional custom previous control button.                              |

#### CarouselItem

| Prop     | Type     | Description                                                  |
| -------- | -------- | ------------------------------------------------------------ |
| children | Children | The child elements to be rendered inside the carousel item.  |

### Styling Classes

| Class Name           | Description                                           |
| -------------------- | ----------------------------------------------------- |
| carousel_container   | For the main container of the carousel                |
| carousel_inner       | For the inner container that holds all carousel items |
| carousel_item        | For individual carousel items                         |
| carousel_item_active | For the currently active (visible) carousel item      |
| carousel_controls    | For the navigation controls container                 |
| carousel_control_prev | For the previous control button                      |
| carousel_control_next | For the next control button                          |

## Checkbox

The Checkbox component is a versatile input element that allows users to toggle between checked and unchecked states. It can be used in forms, settings, and anywhere a binary choice is needed.

### Usage

```rust
<Checkbox
    id="terms"
    checked={false}
    on_toggle={Callback::from(|_| {})}
/>
<label for="terms">
    {"Accept terms and conditions"}
</label>
```

### Props

| Prop      | Type                 | Description                                           |
| --------- | -------------------- | ----------------------------------------------------- |
| id        | String               | The unique identifier for the checkbox.               |
| checked   | bool                 | The checked state of the checkbox.                    |
| on_toggle | Callback<MouseEvent> | The callback to be called when the checkbox is toggled. |
| disabled  | bool                 | Whether the checkbox is disabled.                     |

### Styling Classes

| Class Name        | Description                                                                             |
| ----------------- | --------------------------------------------------------------------------------------- |
| checkbox_base     | Base styles for the checkbox, including size, border, and background                    |
| checkbox_checked  | Styles applied when the checkbox is checked, typically including a checkmark or fill color |
| checkbox_unchecked | Styles applied when the checkbox is unchecked                                          |
| checkbox_disabled | Styles applied when the checkbox is disabled, usually reducing opacity                  |
| checkbox_label    | Styles for the checkbox label, including font size, color, and spacing                  |

## CodeEditor

The CodeEditor component is a high-performance syntax highlighting editor that supports code editing, line numbers, inline diffs, annotations, and type hints. It uses syntect for syntax highlighting.

### Usage

```rust
<CodeEditor
    code="function example() {\n  console.log('Hello world');\n}"
    language="javascript"
    theme="light"
    show_line_numbers={true}
/>
```

### Props

| Prop               | Type                                                 | Description                                              |
| ------------------ | ---------------------------------------------------- | -------------------------------------------------------- |
| code               | String                                               | The code content to edit/display.                        |
| language           | String                                               | The language for syntax highlighting.                    |
| theme              | String                                               | The theme name ('light' or 'dark').                      |
| show_line_numbers  | bool                                                 | Whether to show line numbers (default: true).            |
| font_size          | u8                                                   | Font size in pixels (default: 14).                       |
| font_family        | String                                               | Font family (default: 'JetBrains Mono, monospace').      |
| line_height        | f32                                                  | Line height multiplier (default: 1.5).                   |
| class              | Classes                                              | Additional CSS classes.                                  |
| style              | String                                               | Inline CSS styles.                                       |
| on_change          | Option<Callback<String>>                             | Callback when code changes.                              |
| diffs              | Vec<Diff>                                            | Line-based diffs to highlight changes.                   |
| annotations        | Vec<Annotation>                                      | Code annotations for errors, warnings, etc.              |
| type_hints         | Vec<TypeHint>                                        | Type information displays.                               |
| enable_multi_cursor | bool                                                | Enable multiple cursors with Alt+Click.                  |
| enable_keymap      | bool                                                 | Enable custom keymap support.                            |
| keymap             | Option<HashMap<String, Callback<KeyboardEvent>>>     | Custom keyboard shortcuts.                               |

### Styling Classes

| Class Name         | Description                                                                              |
| ------------------ | ---------------------------------------------------------------------------------------- |
| editor-container   | Container for the entire editor                                                          |
| line-numbers       | Line numbers column                                                                      |
| code-display       | Main code display area                                                                   |
| cursor-element     | Editor cursor                                                                            |
| diff-added         | Added line highlight                                                                     |
| diff-removed       | Removed line highlight                                                                   |
| diff-modified      | Modified line highlight                                                                  |
| annotation-error   | Error annotation                                                                         |
| annotation-warning | Warning annotation                                                                       |
| annotation-info    | Info annotation                                                                          |

## Col

The Col component is a flexible container that arranges its children in a vertical layout. It can be customized with additional classes and an HTML tag of choice.

### Usage

```rust
<Col>
    <div>{ "Child 1" }</div>
    <div>{ "Child 2" }</div>
    <div>{ "Child 3" }</div>
</Col>
```

### Props

| Prop     | Type     | Description                                                |
| -------- | -------- | ---------------------------------------------------------- |
| children | Children | Elements to be displayed inside the Col.                   |
| class    | Classes  | Additional CSS classes to apply to the Col.                |
| tag      | String   | The HTML tag to use for the Col component (default: div).  |

### Styling Classes

| Class Name | Description                                      |
| ---------- | ------------------------------------------------ |
| col_base   | For the main container of the col component      |

## Collapsible

The Collapsible component provides an expandable and collapsible container for content. It's useful for creating accordion-like structures or hiding content that doesn't need to be visible all the time.

### Usage

```rust
<Collapsible open={is_open} on_open_change={on_open_change}>
    <CollapsibleHeader>
        <CollapsibleTitle>{"@peduarte starred 3 repositories"}</CollapsibleTitle>
        <CollapsibleTrigger as_child=true onclick={toggle_open}>
            <svg>/* ... */</svg>
            <span class="sr-only">{"Toggle"}</span>
        </CollapsibleTrigger>
    </CollapsibleHeader>
    <CollapsibleContent is_open={is_open}>
        <CollapsibleItem>{"@radix-ui/primitives"}</CollapsibleItem>
        <CollapsibleItem>{"@radix-ui/colors"}</CollapsibleItem>
        <CollapsibleItem>{"@stitches/react"}</CollapsibleItem>
    </CollapsibleContent>
</Collapsible>
```

### Props

#### Collapsible

| Prop          | Type           | Description                                                |
| ------------- | -------------- | ---------------------------------------------------------- |
| open          | bool           | Whether the collapsible is open or closed.                 |
| on_open_change| Callback<bool> | Callback function called when the open state changes.      |
| class         | String         | Additional CSS classes to apply to the collapsible container. |
| children      | Children       | Child components to be rendered inside the collapsible.    |

#### CollapsibleTrigger

| Prop          | Type                | Description                                            |
| ------------- | ------------------- | ------------------------------------------------------ |
| as_child      | bool                | Whether to render the trigger as a child component.    |
| children      | Children            | Child components to be rendered inside the trigger.    |
| onclick       | Callback<MouseEvent>| Callback function called when the trigger is clicked.  |

#### CollapsibleContent

| Prop          | Type           | Description                                                |
| ------------- | -------------- | ---------------------------------------------------------- |
| class         | String         | Additional CSS classes to apply to the content container.  |
| children      | Children       | Child components to be rendered inside the content area.   |
| is_open       | bool           | Whether the content should be visible.                     |

### Styling Classes

| Class Name             | Description                                                       |
| ---------------------- | ----------------------------------------------------------------- |
| collapsible_container  | Styles for the main container of the collapsible component        |
| collapsible_button     | Styles for the trigger button that toggles the collapsible state  |
| collapsible_content    | Styles for the content container that appears when open           |
| collapsible_header     | Styles for the header section of the collapsible                  |
| collapsible_title      | Styles for the title text within the collapsible header           |
| collapsible_item       | Styles for individual items within the collapsible content        |

## ColorPicker

The ColorPicker component provides an interactive interface for users to select a color from a canvas. It allows for precise color selection and displays the currently selected color in multiple formats.

### Usage

```rust
<ColorPicker
    value={"#FF0000".to_string()}
    width={200}
    height={200}
    onchange={Callback::from(|color: String| {
        // Handle color change
    })}
/>
```

### Props

| Prop     | Type              | Description                                              |
| -------- | ----------------- | -------------------------------------------------------- |
| value    | String            | The initial color value of the picker (e.g., "#FF0000"). |
| onchange | Callback<String>  | A callback function called when the color is changed.    |
| width    | u32               | The width of the color picker canvas. Defaults to 200.   |
| height   | u32               | The height of the color picker canvas. Defaults to 200.  |

### Styling Classes

| Class Name                    | Description                                                          |
| ----------------------------- | -------------------------------------------------------------------- |
| color_picker_container        | Styles the main container of the color picker component              |
| color_picker_canvas           | Styles the canvas element where colors are displayed                 |
| color_picker_indicator        | Styles the indicator that shows the currently selected color         |
| color_picker_input_container  | Styles the container for the color swatch and input field            |
| color_picker_swatch           | Styles the swatch displaying the currently selected color            |
| color_picker_input            | Styles the input field where the color value can be edited           |

## Combobox

The Combobox component is a versatile dropdown component that combines the functionality of a text input and a select dropdown. It allows users to choose from a predefined list of options or enter custom values.

### Usage

```rust
<Combobox
    options={vec![
        ("1".to_string(), "Option 1".to_string()),
        ("2".to_string(), "Option 2".to_string()),
        ("3".to_string(), "Option 3".to_string())
    ]}
    on_select={Callback::from(|selected: String| {
        // Handle selection
    })}
/>
```

### Props

| Prop      | Type                  | Description                                                    |
| --------- | --------------------- | -------------------------------------------------------------- |
| id        | String                | The unique identifier for the combobox.                        |
| options   | Vec<(String, String)> | A list of options where each option is a tuple of (value, label). |
| on_select | Callback<String>      | The callback to be called when an option is selected.          |
| disabled  | bool                  | Whether the combobox is disabled. Default is false.            |

### Styling Classes

| Class Name                 | Description                                                            |
| -------------------------- | ---------------------------------------------------------------------- |
| combobox_button            | Styles for the main button of the combobox                             |
| combobox_button_open       | Additional styles applied when the combobox dropdown is open           |
| combobox_button_disabled   | Styles applied when the combobox is disabled                           |
| combobox_list              | Styles for the dropdown list container                                 |
| combobox_item              | Styles for individual items in the dropdown list                       |
| combobox_item_selected     | Styles applied to the currently selected item                          |


## Command

The Command component is a versatile input component that combines search functionality with option selection. It provides an intuitive interface for users to quickly find and choose from a list of options, supporting both keyboard and mouse interactions.

### Usage

```rust
<Command<String>
    placeholder="Search commands..."
    options={vec![
        ("1".to_string(), "search, files".to_string(), "Search files".to_string(), Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-search"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
        })),
        ("2".to_string(), "create, document".to_string(), "Create new document".to_string(), Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-plus"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="12" x2="12" y1="18" y2="12"/><line x1="9" x2="15" y1="15" y2="15"/></svg>
        })),
        ("3".to_string(), "open, settings".to_string(), "Open settings".to_string(), Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
        }))
    ]}
    on_select={Callback::from(|selected| {
        // Handle the selection
    })}
/>
```

### Props

| Prop        | Type                            | Description                                                             |
| ----------- | ------------------------------- | ----------------------------------------------------------------------- |
| placeholder | String                          | The placeholder text to be displayed in the input field.                |
| options     | Vec<(T, String, String, Option<Html>)> | A list of options where each option is a tuple of (value, keywords, label, icon). |
| on_select   | Callback<T>                     | The callback to be called when an option is selected.                  |
| class       | Classes                         | Additional CSS classes to apply to the command container.              |

### Styling Classes

| Class Name             | Description                                                        |
| ---------------------- | ------------------------------------------------------------------ |
| command_container      | Styles the main container of the command component                 |
| command_input_wrapper  | Styles the wrapper around the input and search icon                |
| command_icon           | Styles the search icon                                             |
| command_input          | Styles the input field                                             |
| command_list           | Styles the list of options                                         |
| command_item           | Styles individual items in the list                                |
| command_item_icon      | Styles icons within list items                                     |
| command_selected_item  | Styles the currently selected item in the list                     |

## Container

The Container component is a versatile layout element that wraps content with optional padding, expanding, and size variations. It helps in creating responsive designs that adjust according to the screen size.

### Usage

```rust
<Container
    class={classes!("custom-class")}
    tag="section"
    expanding={true}
    padding_x={true}
    padding_y={true}
    variant={ContainerVariant::Large}
    style={Some("background-color: lightgrey;".to_string())}
>
    <p>{ "This is a container with the large variant." }</p>
</Container>
```

### Props

| Prop      | Type             | Description                                                      |
| --------- | ---------------- | ---------------------------------------------------------------- |
| children  | Children         | The child elements to be rendered inside the container.          |
| class     | Classes          | Additional classes to apply to the container.                    |
| tag       | String           | The HTML tag to use for the container. Default is 'div'.         |
| expanding | bool             | Whether the container should expand. Default is true.            |
| padding_x | bool             | Whether to apply horizontal padding. Default is true.            |
| padding_y | bool             | Whether to apply vertical padding. Default is true.              |
| variant   | ContainerVariant | The size variant of the container. Default is 'Responsive'.      |
| style     | Option<String>   | Optional inline styles for the container.                        |

### ContainerVariant

| Variant    | Description                       |
| ---------- | --------------------------------- |
| Small      | A small container                 |
| Narrow     | A narrow container                |
| Large      | A large container                 |
| Responsive | A responsive container (default)  |
| None       | No specific sizing applied        |

### Styling Classes

| Class Name            | Description                                         |
| --------------------- | --------------------------------------------------- |
| container_padding_x   | Controls the horizontal padding of the container    |
| container_padding_y   | Controls the vertical padding of the container      |
| container_expanding   | Makes the container expand to fill available space  |
| container_small       | Applies styles for the small variant                |
| container_narrow      | Applies styles for the narrow variant               |
| container_large       | Applies styles for the large variant                |
| container_responsive  | Applies styles for the responsive variant           |

## MainContent

The MainContent component is designed to wrap the main content of the application, with optional padding and an adjacent aside element for additional content. It provides a flexible layout structure for creating responsive designs.

### Usage

```rust
<MainContent expanding={true} padding_x={true} padding_y={true} aside={Some(html! { <p>{"Aside content here"}</p> })}>
    <p>{ "Main content goes here" }</p>
</MainContent>
```

### Props

| Prop      | Type         | Description                                                            |
| --------- | ------------ | ---------------------------------------------------------------------- |
| children  | Children     | The child elements to be rendered inside the main content.             |
| class     | Classes      | Additional CSS classes to be applied to the main container.            |
| expanding | bool         | Determines if the main content should expand to fill the available space. Default is true. |
| padding_x | bool         | If true, applies padding on the x-axis. Default is true.               |
| padding_y | bool         | If true, applies padding on the y-axis. Default is true.               |
| aside     | Option<Html> | Optional content to be rendered in an adjacent aside element.          |

### Styling Classes

| Class Name              | Description                                                        |
| ----------------------- | ------------------------------------------------------------------ |
| content_with_aside      | Applied to the main content when an aside is present               |
| content_aside           | Styles the aside container                                         |
| content_aside_container | Styles the inner container of the aside content                    |

## ContextMenu

The ContextMenu component presents a contextual menu that appears upon a right-click action. It is a versatile component suitable for providing additional options or actions related to specific content or interactions.

### Usage

```rust
<ContextMenu>
    <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
        { "Right click here" }
    </ContextMenuTrigger>
    <ContextMenuContent class="w-64">
        <ContextMenuItem inset=true>
            { "Back" }
            <ContextMenuShortcut>{ "⌘[" }</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuItem inset=true disabled=true>
            { "Forward" }
            <ContextMenuShortcut>{ "⌘]" }</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuSub>
            <ContextMenuSubTrigger inset=true>{ "More Tools" }</ContextMenuSubTrigger>
            <ContextMenuSubContent class="w-48">
                <ContextMenuItem>{ "Developer Tools" }</ContextMenuItem>
            </ContextMenuSubContent>
        </ContextMenuSub>
        <ContextMenuSeparator />
        <ContextMenuCheckboxItem checked=true>
            { "Show Bookmarks Bar" }
        </ContextMenuCheckboxItem>
        <ContextMenuRadioGroup value="pedro">
            <ContextMenuLabel inset=true>{ "People" }</ContextMenuLabel>
            <ContextMenuRadioItem value="pedro">{ "Pedro Duarte" }</ContextMenuRadioItem>
            <ContextMenuRadioItem value="colm">{ "Colm Tuite" }</ContextMenuRadioItem>
        </ContextMenuRadioGroup>
    </ContextMenuContent>
</ContextMenu>
```

### Components

#### ContextMenu

| Prop     | Type     | Description                                                           |
| -------- | -------- | --------------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the context menu component.  |

#### ContextMenuTrigger

| Prop     | Type     | Description                                                      |
| -------- | -------- | ---------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the trigger element.    |
| class    | Classes  | Additional CSS classes to apply to the trigger.                  |

#### ContextMenuContent

| Prop     | Type     | Description                                                        |
| -------- | -------- | ------------------------------------------------------------------ |
| children | Children | The child elements to be rendered inside the context menu content. |
| class    | Classes  | Additional CSS classes to apply to the content.                    |

#### ContextMenuItem

| Prop     | Type     | Description                                                       |
| -------- | -------- | ----------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the context menu item.   |
| inset    | bool     | Whether the item should be inset.                                 |
| disabled | bool     | Whether the item is disabled.                                     |

#### ContextMenuSub, ContextMenuSubTrigger, ContextMenuSubContent

| Prop     | Type     | Description                                                     |
| -------- | -------- | --------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the respective component. |
| inset    | bool     | For SubTrigger - whether the trigger should be inset.           |
| class    | Classes  | For SubContent - additional CSS classes to apply.               |

#### Other Components

- **ContextMenuSeparator**: A separator line in the context menu.
- **ContextMenuCheckboxItem**: A checkbox item with a `checked` prop.
- **ContextMenuRadioGroup**: A group of radio items with a `value` prop.
- **ContextMenuLabel**: A label for a group of items with an optional `inset` prop.
- **ContextMenuRadioItem**: A radio item with a `value` prop.
- **ContextMenuShortcut**: A keyboard shortcut displayed next to a menu item.

### Styling Classes

| Class Name               | Description                                    |
| ------------------------ | ---------------------------------------------- |
| context_menu_trigger     | For the trigger element                        |
| context_menu_content     | For the main content wrapper                   |
| context_menu_item        | For individual menu items                      |
| context_menu_sub         | For submenu containers                         |
| context_menu_sub_trigger | For submenu trigger elements                   |
| context_menu_sub_content | For submenu content wrappers                   |
| context_menu_separator   | For separator lines in the menu                |
| context_menu_checkbox_item | For checkbox items                           |
| context_menu_radio_group | For radio group containers                     |
| context_menu_label       | For labels in the menu                         |
| context_menu_radio_item  | For radio items                                |
| context_menu_shortcut    | For keyboard shortcut displays                 |


## CopyButton

The CopyButton component provides an easy way to copy text content to the clipboard with visual feedback for the user.

### Usage

```rust
<div className="flex items-center space-x-2">
    <code className="bg-gray-100 dark:bg-zinc-700 p-2 rounded">{ "npm install wonopui" }</code>
    <CopyButton copy_text="npm install wonopui" />
</div>
```

### Props

| Prop       | Type         | Description                                      |
| ---------- | ------------ | ------------------------------------------------ |
| copy_text  | String       | The text to copy to the clipboard                |
| copied_text | Option<String> | Optional text to display when copy is successful |
| children   | Option<Html> | Optional custom element to display instead of the default copy icon |

### Styling Classes

| Class Name       | Description                                 |
| ---------------- | ------------------------------------------- |
| copy_button      | Base styling for the copy button            |
| copy_button_icon | Styling for the default copy icon           |
| copied_indicator | Styling for the success indicator after copying |

## Dialog

The Dialog component is a versatile UI element that displays content in a modal dialog. It is composed of several subcomponents to provide a flexible and customizable experience.

### Usage

```rust
<DialogProvider>
    <DialogTrigger id="example-dialog">
        <Button>{"Open Dialog"}</Button>
    </DialogTrigger>
    <Dialog id="example-dialog">
        <DialogHeader>
            <DialogTitle>{"Dialog Title"}</DialogTitle>
        </DialogHeader>
        <DialogBody>{"This is a description inside the dialog."}</DialogBody>
        <DialogFooter>
            <DialogClose>
                <Button>{"Close"}</Button>
            </DialogClose>
        </DialogFooter>
    </Dialog>
</DialogProvider>
```

### Components

#### DialogProvider

| Prop     | Type     | Description                                         |
| -------- | -------- | --------------------------------------------------- |
| children | Children | Components that need access to the dialog context.  |

#### DialogTrigger

| Prop     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| id       | String   | A unique identifier for the dialog.                  |
| children | Children | The trigger element that will open the dialog.       |

#### Dialog

| Prop     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| id       | String   | A unique identifier matching the DialogTrigger id.   |
| children | Children | The content to display inside the dialog.            |

#### DialogHeader, DialogBody, DialogFooter

| Prop     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| children | Children | The content to display in the respective section.    |

#### DialogTitle

| Prop     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| children | Children | The title content of the dialog.                     |

#### DialogClose

| Prop     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| children | Children | The element that will close the dialog when clicked. |

### Styling Classes

| Class Name         | Description                                                 |
| ------------------ | ----------------------------------------------------------- |
| dialog_container   | For the main dialog container with overlay                  |
| dialog_content     | For the dialog content wrapper                              |
| dialog_header      | For the header section of the dialog                        |
| dialog_title       | For the dialog title                                        |
| dialog_description | For the dialog description/body text                        |
| dialog_footer      | For the footer section containing action buttons            |

## Divider

The Divider component creates a horizontal separation between content with an optional text label in the middle.

### Usage

```rust
<p>{ "Content above the divider" }</p>
<Divider />
<p>{ "Content below the divider" }</p>

<p>{ "Content above the divider" }</p>
<Divider text="OR" />
<p>{ "Content below the divider" }</p>
```

### Props

| Prop  | Type   | Description                                             |
| ----- | ------ | ------------------------------------------------------- |
| class | String | Additional CSS classes                                  |
| style | String | Inline CSS styles                                       |
| text  | String | Optional text to display in the middle of the divider   |

## DragPoint

The DragPoint component creates a draggable element that provides callbacks for drag start and stop events, enabling custom behavior during these events.

### Usage

```rust
<DragPoint
    class={classes!("w-8", "h-8", "bg-blue-500", "rounded-full", "cursor-move")}
    onstart={Callback::from(|e: PointerEvent| {
        log::info!("Drag started at: ({}, {})", e.client_x(), e.client_y());
    })}
    onstop={Callback::from(|_| {
        log::info!("Drag stopped");
    })}
/>
```

### Props

| Prop    | Type               | Description                                      |
| ------- | ------------------ | ------------------------------------------------ |
| onstart | Callback<PointerEvent> | Callback executed when dragging starts       |
| onstop  | Callback<()>       | Callback executed when dragging stops            |
| class   | Classes            | CSS classes for styling the draggable element    |
| tag     | String             | HTML tag to use for the component's root element |

### Styling Classes

| Class Name  | Description                                        |
| ----------- | -------------------------------------------------- |
| drag_point  | Applied to the root element of the DragPoint component |

## Drawer

The Drawer component is a versatile UI element that slides in from the side of the screen, composed of several subcomponents to provide a flexible and customizable experience.

### Usage

```rust
<DrawerProvider<String> side={DrawerSide::Right} render={Callback::from(|_| html! {
    <Drawer<String>>
        <DrawerHeader>
            <DrawerTitle>{"Drawer Title"}</DrawerTitle>
            <DrawerClose<String>>{"X"}</DrawerClose<String>>
        </DrawerHeader>
        <p>{"Drawer Content"}</p>
        <DrawerFooter>
            <DrawerClose<String>>
                <Button>{"Close"}</Button>
            </DrawerClose<String>>
        </DrawerFooter>
    </Drawer<String>>
})}>
    <DrawerTrigger<String> drawer={"example".to_string()}>
        <Button>{"Open Drawer"}</Button>
    </DrawerTrigger<String>>
</DrawerProvider<String>>
```

### Components

#### DrawerProvider<T>

| Prop           | Type                | Description                                  |
| -------------- | ------------------- | -------------------------------------------- |
| children       | Children            | Child elements rendered inside the drawer component |
| side           | DrawerSide          | Side from which drawer slides in (Left, Right, Top, Bottom) |
| render         | Callback<T, Html>   | Callback to render the drawer content        |
| curtain        | bool                | Whether to show a curtain behind drawer      |
| curtain_content| Html                | Content to show in the curtain               |

#### DrawerTrigger<T>

| Prop           | Type                | Description                                  |
| -------------- | ------------------- | -------------------------------------------- |
| children       | Children            | Child elements rendered inside trigger element |
| drawer         | T                   | Identifier for the drawer to be opened       |

#### Drawer<T>

| Prop           | Type                | Description                                  |
| -------------- | ------------------- | -------------------------------------------- |
| children       | Children            | Child elements rendered inside drawer content |

#### Other Components

- **DrawerClose<T>**: Element that triggers closing of the drawer
- **DrawerHeader**: Header section of the drawer
- **DrawerTitle**: Title of the drawer
- **DrawerFooter**: Footer section of the drawer

### Styling Classes

| Class Name          | Description                                          |
| ------------------- | ---------------------------------------------------- |
| drawer_provider     | For the main container of the drawer component       |
| drawer_container    | For the content area of the drawer                   |
| drawer_header       | For the header section of the drawer                 |
| drawer_title        | For the title within the drawer header               |
| drawer_description  | For the description text in the drawer               |
| drawer_footer       | For the footer section of the drawer                 |
| drawer_right        | Applied when drawer slides from the right            |
| drawer_top          | Applied when drawer slides from the top              |
| drawer_bottom       | Applied when drawer slides from the bottom           |
| drawer_left         | Applied when drawer slides from the left             |

## Dropdown

The Dropdown component provides a versatile dropdown menu with customizable items, optional icons, separators, and custom widgets, integrating with the Popover component for positioning and display control.

### Usage

```rust
<Dropdown
    items={vec![
        DropdownItem::Action {
            label: "Item 1".into(),
            icon: Some(html! { <i class="fas fa-user"></i> }),
            onclick: Callback::from(|_| log::info!("Item 1 clicked")),
            disabled: false,
        },
        DropdownItem::Action {
            label: "Item 2".into(),
            icon: Some(html! { <i class="fas fa-cog"></i> }),
            onclick: Callback::from(|_| log::info!("Item 2 clicked")),
            disabled: false,
        },
        DropdownItem::Separator,
        DropdownItem::Action {
            label: "Item 3".into(),
            icon: Some(html! { <i class="fas fa-sign-out-alt"></i> }),
            onclick: Callback::from(|_| log::info!("Item 3 clicked")),
            disabled: false,
        },
        DropdownItem::Widget(html! { <input type="text" placeholder="Custom widget" /> }),
    ]}
    full_width={false}
>
    <Button>{ "Open Dropdown" }</Button>
</Dropdown>
```

### Props

| Prop      | Type             | Description                                             |
| --------- | ---------------- | ------------------------------------------------------- |
| items     | Vec<DropdownItem>| A vector of dropdown items                              |
| children  | Children         | The trigger for the dropdown, generally a button or link|
| position  | PopoverPosition  | Position of dropdown relative to its trigger            |
| class     | Classes          | Additional CSS classes for styling the dropdown         |
| full_width| bool             | Whether dropdown should take full width of container    |

### DropdownItem

| Variant   | Parameters                                           | Description                       |
| --------- | ---------------------------------------------------- | --------------------------------- |
| Action    | label: String, icon: Option<Html>, onclick: Callback<MouseEvent>, disabled: bool | Actionable item in dropdown |
| Widget    | Html                                                 | Custom widget to insert in dropdown |
| Separator | -                                                    | Separator line between items      |

### Styling Classes

| Class Name            | Description                                          |
| --------------------- | ---------------------------------------------------- |
| dropdown_content      | For the main dropdown content container              |
| dropdown_item         | For individual dropdown items                        |
| dropdown_item_icon    | For the icon container within a dropdown item        |
| dropdown_separator    | For separator items between groups                   |
| dropdown_item_disabled| For disabled dropdown items                          |
| dropdown_item_widget  | For custom widget items within the dropdown          |
| dropdown_heading      | TODO |

## GroupButton

The GroupButton component is designed to group multiple buttons together, allowing for quick toggling between different options. It is highly customizable with flex direction, default active value, and more.

### Usage

```rust
<GroupButton default_value="option1" class="w-[400px]" direction={FlexDirection::Row} on_change={Callback::from(|value: String| {
    // Handle value change
})}>
    <GroupButtonTrigger value="option1" onclick={Callback::noop()}>{"Option 1"}</GroupButtonTrigger>
    <GroupButtonTrigger value="option2" onclick={Callback::noop()}>{"Option 2"}</GroupButtonTrigger>
</GroupButton>
```

### Props

#### GroupButton

| Prop         | Type               | Description                                                  |
| ------------ | ------------------ | ------------------------------------------------------------ |
| children     | Children           | The child elements, typically GroupButtonTrigger components. |
| default_value| String             | The default active button value.                             |
| class        | Classes            | Additional CSS classes for the container.                    |
| direction    | FlexDirection      | The direction of the flex container (Row or Column).         |
| on_change    | Callback<String>   | Callback function that is called when an option is selected. |

#### GroupButtonTrigger

| Prop     | Type                 | Description                                     |
| -------- | -------------------- | ----------------------------------------------- |
| value    | String               | The value for the button.                       |
| children | Children             | Content to be displayed inside the button.      |
| class    | Classes              | Additional CSS classes for the button.          |
| onclick  | Callback<MouseEvent> | Callback function for the click event.          |

### Styling Classes

| Class Name                   | Description                                        |
| ---------------------------- | -------------------------------------------------- |
| group_button_container       | For the outermost container of the group button    |
| group_button_list            | For the container that wraps the group buttons     |
| group_button_trigger         | For individual button triggers                     |
| group_button_trigger_active  | For the active button trigger                      |
| group_button_trigger_inactive| For inactive button triggers                       |

## Iframe

The Iframe component allows embedding an iframe within a Yew application. It comes with support for injecting documents, event handling, and propagating events to the parent window, making it highly customizable and interactive.

### Usage

```rust
<Iframe
    class="w-full h-64"
    body_class="p-4"
    srcdoc={Some("<p>Hello, iframe content!</p>".to_string())}
    onkeydown={Callback::from(|_e: web_sys::KeyboardEvent| {
        log::info!("Key down event within iframe");
    })}
>
    { "Fallback content if no srcdoc is provided" }
</Iframe>
```

### Props

| Prop      | Type                                  | Description                                           |
| --------- | ------------------------------------- | ----------------------------------------------------- |
| children  | Children                              | Content to be displayed if no srcdoc is provided.     |
| class     | Classes                               | CSS class(es) for the iframe element.                 |
| body_class| String                                | CSS class for the iframe body element.                |
| srcdoc    | Option<String>                        | Optional HTML content to embed within the iframe.     |
| onkeydown | Option<Callback<web_sys::KeyboardEvent>>| Callback for handling 'keydown' events in the iframe.|

### Styling Classes

| Class Name        | Description                                    |
| ----------------- | ---------------------------------------------- |
| iframe_container  | For the main container of the iframe component |
| iframe_element    | For the iframe element itself                  |

## Input

The Input component is a versatile text input field, supporting multiple event handlers and customizable properties. It provides a controlled input experience with extensive callback support for various events.

### Usage

```rust
<Input
    value="Hello, World!"
    placeholder="Basic input"
    class="w-full"
/>

<Input
    value=""
    placeholder="Password input"
    kind="password"
    class="w-full"
/>

<Input
    value="5"
    kind="number"
    min="0"
    max="10"
    step="1"
    class="w-full"
/>
```

### Props

| Prop       | Type                    | Description                                            |
| ---------- | ----------------------- | ------------------------------------------------------ |
| value      | String                  | The current value of the input.                        |
| oninput    | Callback<InputEvent>    | Callback to handle the oninput event.                  |
| ontext     | Callback<String>        | Callback to handle text input changes.                 |
| onchange   | Callback<Event>         | Callback to handle the onchange event.                 |
| onkeypress | Callback<KeyboardEvent> | Callback to handle keypress events.                    |
| onkeydown  | Callback<KeyboardEvent> | Callback to handle keydown events.                     |
| onkeyup    | Callback<KeyboardEvent> | Callback to handle keyup events.                       |
| placeholder| String                  | Placeholder text for the input.                        |
| class      | Classes                 | CSS classes to apply to the input.                     |
| id         | String                  | HTML id attribute for the input.                       |
| name       | String                  | HTML name attribute for the input.                     |
| kind       | String                  | Type of input (text, password, number, etc.).          |
| maxlength  | Option<i32>             | Maximum length of the input value.                     |
| readonly   | bool                    | Whether the input is read-only.                        |
| min        | Option<String>          | Minimum value for the input (for number inputs).       |
| max        | Option<String>          | Maximum value for the input (for number inputs).       |
| step       | Option<String>          | Step value for the input (for number inputs).          |
| node_ref   | NodeRef                 | Reference to the input node.                           |

### Styling Classes

| Class Name | Description                                                                                                          |
| ---------- | -------------------------------------------------------------------------------------------------------------------- |
| input_base | Base styling for the input. Default: 'border rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-600' |

## Kanban

The Kanban component is a flexible task management system that allows users to organize tasks into columns and drag cards between them, following the Kanban methodology.

### Usage

```rust
<Kanban>
    <KanbanColumn id="todo" title="To Do" ondrop={ondrop.clone()}>
        <KanbanCard id="task1" title="Project Setup" order={0}>
            {"Configure repository and install dependencies"}
        </KanbanCard>
        <KanbanCard id="task2" title="Design UI" order={1}>
            {"Create wireframes and mockups"}
        </KanbanCard>
    </KanbanColumn>
    <KanbanColumn id="progress" title="In Progress" ondrop={ondrop.clone()}>
        <KanbanCard id="task3" title="Implementation" order={0}>
            {"Start coding the core features"}
        </KanbanCard>
    </KanbanColumn>
    <KanbanColumn id="done" title="Done" ondrop={ondrop.clone()}>
        <KanbanCard id="task4" title="Requirements" order={0}>
            {"Gather project requirements"}
        </KanbanCard>
    </KanbanColumn>
</Kanban>
```

### Props

#### Kanban

| Prop                     | Type     | Description                                                     |
| ------------------------ | -------- | --------------------------------------------------------------- |
| children                 | Children | The columns to display on the board.                            |
| class                    | Classes  | Additional CSS classes for the Kanban container.                |
| allow_multiple_column_drops | bool  | If true, allows dropping cards on multiple columns simultaneously. Default is false. |

#### KanbanColumn

| Prop          | Type                                  | Description                                               |
| ------------- | ------------------------------------- | --------------------------------------------------------- |
| children      | ChildrenWithProps<KanbanCard>         | The cards to display in the column.                       |
| class         | Classes                               | Additional CSS classes for the column.                    |
| header_class  | Classes                               | Additional CSS classes for the column header.             |
| body_class    | Classes                               | Additional CSS classes for the column body.               |
| id            | AttrValue                             | Unique identifier for the column.                         |
| title         | AttrValue                             | The title to display in the column header.                |
| ondragenter   | Option<Callback<DragEvent>>           | Callback when a draggable item enters the column.         |
| ondragleave   | Option<Callback<DragEvent>>           | Callback when a draggable item leaves the column.         |
| ondragover    | Option<Callback<DragEvent>>           | Callback when a draggable item is over the column.        |
| ondrop        | Option<Callback<(String, String, Option<String>)>> | Callback when a card is dropped on the column. Receives (card_id, column_id, target_card_id). |

#### KanbanCard

| Prop          | Type                      | Description                                                   |
| ------------- | ------------------------- | ------------------------------------------------------------- |
| children      | Children                  | The content to display in the card.                           |
| class         | Classes                   | Additional CSS classes for the card.                          |
| id            | AttrValue                 | Unique identifier for the card.                               |
| column_id     | Option<AttrValue>         | ID of the column the card belongs to.                         |
| title         | Option<AttrValue>         | Optional title to display at the top of the card.             |
| order         | Option<usize>             | Ordering position within the column.                          |
| ondragstart   | Option<Callback<DragEvent>> | Callback when dragging of the card starts.                  |
| ondragend     | Option<Callback<DragEvent>> | Callback when dragging of the card ends.                    |
| ondragover    | Option<Callback<DragEvent>> | Callback when a draggable item is over the card.            |
| ondragenter   | Option<Callback<DragEvent>> | Callback when a draggable item enters the card.             |
| ondragleave   | Option<Callback<DragEvent>> | Callback when a draggable item leaves the card.             |
| onclick       | Option<Callback<MouseEvent>> | Callback when the card is clicked.                         |

### Styling Classes

| Class Name                | Description                                              |
| ------------------------- | -------------------------------------------------------- |
| kanban_container          | Styles for the main Kanban board container.              |
| kanban_column             | Styles for each column in the Kanban board.              |
| kanban_column_header      | Styles for the column header containing the title.       |
| kanban_column_body        | Styles for the column body that contains the cards.      |
| kanban_column_over        | Styles applied when a card is being dragged over a column.|
| kanban_card               | Styles for each card in the Kanban board.                |
| kanban_card_title         | Styles for the card title.                               |
| kanban_card_content       | Styles for the card content area.                        |
| kanban_card_dragging      | Styles applied to a card when it's being dragged.        |
| kanban_card_drag_target   | Styles applied when another card is being dragged over it.|

## Label

The Label component is designed to provide accessible text labels for form elements, enhancing usability and accessibility. It can be styled with custom classes and linked to specific form elements using the `for_id` attribute.

### Usage

```rust
<Label for_id="example-input">{ "Username" }</Label>
<Input id="example-input" placeholder="Enter your username..." />

<Label for_id="required-field" class="flex items-center">
    { "Password " }
    <span class="text-red-500 ml-1">{ "*" }</span>
</Label>
<Input id="required-field" kind="password" />
```

### Props

| Prop     | Type     | Description                                              |
| -------- | -------- | -------------------------------------------------------- |
| children | Children | The content to be displayed inside the label.            |
| class    | Classes  | Additional CSS classes to style the label.               |
| for_id   | String   | The ID of the form element the label is associated with. |

### Styling Classes

| Class Name | Description                                                                         |
| ---------- | ----------------------------------------------------------------------------------- |
| label_base | Base styling for the label. Default: Basic styling for the label component.         |

## MediaQuery

The MediaQuery hook (`use_media_query`) allows components to respond to changes in media queries, enabling responsive design and adaptations to user preferences like dark mode.

### Usage

```rust
use wonopui::*;
use yew::prelude::*;

#[function_component(ResponsiveComponent)]
fn responsive_component() -> Html {
    let is_mobile = use_media_query("(max-width: 768px)");
    let is_dark_mode = use_media_query("(prefers-color-scheme: dark)");

    html! {
        <div>
            {
                if is_mobile {
                    html! { <p>{ "Mobile layout" }</p> }
                } else {
                    html! { <p>{ "Desktop layout" }</p> }
                }
            }
            {
                if is_dark_mode {
                    html! { <p>{ "Dark mode is enabled" }</p> }
                } else {
                    html! { <p>{ "Light mode is enabled" }</p> }
                }
            }
        </div>
    }
}
```

### Hook Parameters

| Parameter | Type   | Description                        |
| --------- | ------ | ---------------------------------- |
| query     | &str   | The CSS media query to match against. |

### Return Value

| Type     | Description                                                |
| -------- | ---------------------------------------------------------- |
| bool     | Whether the media query currently matches.                 |

## MultiColumnSidebar

The MultiColumnSidebar component provides a sophisticated multi-level navigation interface with multiple columns, ideal for complex application navigation hierarchies.

### Usage

```rust
<MultiColumnSidebar>
    <SidebarColumn>
        <div class="h-full flex flex-col">
            <div class="p-4 font-semibold border-b">{ "Main Menu" }</div>
            <div class="flex-1 overflow-y-auto p-2">
                <SidebarMenu>
                    <SidebarItem>{ "Dashboard" }</SidebarItem>
                    <SidebarItem>{ "Users" }</SidebarItem>
                    <SidebarItem>{ "Settings" }</SidebarItem>
                </SidebarMenu>
            </div>
        </div>
    </SidebarColumn>
    <SidebarColumn>
        <div class="h-full flex flex-col">
            <div class="p-4 font-semibold border-b">{ "User Management" }</div>
            <div class="flex-1 overflow-y-auto p-2">
                <SidebarMenu>
                    <SidebarItem>{ "List Users" }</SidebarItem>
                    <SidebarItem>{ "Create User" }</SidebarItem>
                    <SidebarItem>{ "User Groups" }</SidebarItem>
                </SidebarMenu>
            </div>
        </div>
    </SidebarColumn>
</MultiColumnSidebar>
```

### Props

#### MultiColumnSidebar

| Prop            | Type    | Description                                              |
| --------------- | ------- | -------------------------------------------------------- |
| children        | Children| Content for the sidebar columns.                         |
| curtain_content | Html    | Content for the backdrop when sidebar is open in mobile. |

#### SidebarColumn

| Prop            | Type        | Description                                           |
| --------------- | ----------- | ----------------------------------------------------- |
| children        | Children    | Content for the column.                               |
| width           | Option<i32> | Fixed width in pixels.                                |
| hide_when_folded| bool        | Whether to hide when sidebar is folded.               |
| header          | Option<Html>| Optional header content.                              |
| footer          | Option<Html>| Optional footer content.                              |
| class           | Classes     | Additional CSS classes.                               |

### Styling Classes

| Class Name              | Description                                           |
| ----------------------- | ----------------------------------------------------- |
| sidebar_column          | For individual columns within the multi-column layout |
| sidebar_column_header   | For the header section of a sidebar column            |
| sidebar_column_content  | For the main content area of a sidebar column         |
| sidebar_column_footer   | For the footer section of a sidebar column            |

## Notification

The Notification component provides a system for displaying temporary, non-intrusive messages to users. It requires a NotificationProvider at a higher level in the component tree and uses the use_notify hook for triggering notifications.

### Usage

```rust
#[function_component(App)]
fn app() -> Html {
    html! {
        <NotificationProvider>
            <MyComponent />
        </NotificationProvider>
    }
}

#[function_component(MyComponent)]
fn my_component() -> Html {
    let show_notification = use_notify();

    let on_click = {
        let show_notification = show_notification.clone();
        Callback::from(move |_| {
            show_notification.emit((
                "Success!".to_string(),
                "Your changes have been saved.".to_string(),
                Some(html! {
                    <button class="text-blue-500 hover:text-blue-700">{"View"}</button>
                }),
            ));
        })
    };

    html! {
        <button onclick={on_click}>
            { "Show Notification" }
        </button>
    }
}
```

### Components and Hooks

#### NotificationProvider

| Prop     | Type     | Description                                           |
| -------- | -------- | ----------------------------------------------------- |
| children | Children | The child elements rendered inside the provider.      |

#### use_notify Hook

Returns a callback that can be used to show notifications. The callback accepts a tuple with the following parameters:

| Parameter   | Type           | Description                                      |
| ----------- | -------------- | ------------------------------------------------ |
| title       | String         | The title of the notification.                   |
| description | String         | The description or message of the notification.  |
| action      | Option<Html>   | Optional action element (like a button or link). |

### Styling Classes

| Class Name                     | Description                                                    |
| ------------------------------ | -------------------------------------------------------------- |
| notification_container         | For individual notification containers                         |
| notification_content           | For the content area of a notification                         |
| notification_title             | For the notification title                                     |
| notification_description       | For the notification description/message                       |
| notification_timestamp         | For the relative time display                                  |
| notification_close_button      | For the close/dismiss button                                   |
| notification_close_icon        | For the icon within the close button                           |
| notification_action_container  | For the container of action elements                           |
| notification_list_container    | For the container of all notifications                         |

## PageContent

The PageContent component provides a standardized container for the main content area of your application, with support for paddings, backgrounds, and borders.

### Usage

```rust
<PageContent>
    <H3>{ "Page Title" }</H3>
    <Paragraph>{ "This is an example of basic page content with default styling. It provides proper spacing and a clean background." }</Paragraph>
    <Button class="mt-4">{ "Action Button" }</Button>
</PageContent>
```

### Props

| Prop     | Type     | Description                                             |
| -------- | -------- | ------------------------------------------------------- |
| children | Children | The content to display within the page content area     |
| class    | String   | Additional CSS classes                                  |

### Styling Classes

| Class Name     | Description                                          |
| -------------- | ---------------------------------------------------- |
| page_content   | For the main container of the page content area      |

## PageHeader

The PageHeader component provides a flexible header element for pages, which includes a title and optional child elements. This can be useful for providing consistent headers across different sections of an application.

### Usage

```rust
<PageHeader title="Simple Header" />

<PageHeader title="Header with Actions">
    <Button>{"Action 1"}</Button>
    <Button>{"Action 2"}</Button>
</PageHeader>
```

### Props

| Prop     | Type     | Description                                             |
| -------- | -------- | ------------------------------------------------------- |
| title    | String   | The title to be displayed in the header                 |
| children | Children | Optional child elements to be rendered alongside the title |

### Styling Classes

| Class Name            | Description                                          |
| --------------------- | ---------------------------------------------------- |
| page_header_container | For the main container of the page header            |
| page_header_title     | For the title text in the header                     |
| page_header_actions   | For the container of action elements                 |

## Pagination

The Pagination component helps with navigation between pages of content. It provides a user-friendly interface to browse through multiple pages of data.

### Usage

```rust
<Pagination
    total_pages={10}
    current_page={1}
    on_page_change={Callback::from(|page| {
        // Handle page change
    })}
/>

<Pagination
    total_pages={20}
    current_page={5}
    on_page_change={on_page_change}
    prev={html!(<span>{ "Previous" }</span>)}
    next={html!(<span>{ "Forward" }</span>)}
/>
```

### Props

| Prop          | Type                | Description                                             |
| ------------- | ------------------- | ------------------------------------------------------- |
| total_pages   | usize               | Total number of pages                                   |
| current_page  | usize               | Currently active page                                   |
| on_page_change| Callback<usize>     | Callback function that receives the new page number when a page is selected |
| next          | Option<Html>        | Custom content for the 'Next' button                    |
| prev          | Option<Html>        | Custom content for the 'Previous' button                |

### Styling Classes

| Class Name           | Description                                             |
| -------------------- | ------------------------------------------------------- |
| pagination_container | For the main pagination container                       |
| pagination_item      | For individual page number buttons                      |
| pagination_active    | For the currently active page button                    |
| pagination_ellipsis  | For the ellipsis indicator                              |
| pagination_prev      | For the previous page button                            |
| pagination_next      | For the next page button                                |

## PaintCanvas

The PaintCanvas component is a versatile drawing canvas that supports image loading and drawing with mouse events. It is useful for creating interactive web drawing applications.

### Usage

```rust
<PaintCanvas image_src={Some("https://example.com/image.png".to_string())} />
```

### Props

| Prop      | Type           | Description                                       |
| --------- | -------------- | ------------------------------------------------- |
| image_src | Option<String> | Optional image source URL to load into the canvas |

### Styling Classes

| Class Name          | Description                                          |
| ------------------- | ---------------------------------------------------- |
| paint_canvas_container | For the main container of the paint canvas           |
| paint_canvas        | For the canvas element itself                        |
| paint_canvas_image  | For the loaded image within the canvas               |

## Placeholder

The Placeholder component serves as a content placeholder with a dashed background pattern. It's useful for indicating where actual content will eventually go or for displaying loading states.

### Usage

```rust
<Placeholder text="Default Placeholder" />
<Placeholder text="Custom Class" class={classes!("mt-4", "h-32")} />
```

### Props

| Prop  | Type    | Description                                                   |
| ----- | ------- | ------------------------------------------------------------- |
| class | Classes | Additional CSS classes to apply to the Placeholder component  |
| text  | String  | The text to display inside the Placeholder                    |

### Styling Classes

| Class Name           | Description                                                |
| -------------------- | ---------------------------------------------------------- |
| placeholder_container| Styles for the main container of the placeholder           |
| placeholder_svg      | Styles for the SVG element that creates the dashed background |
| placeholder_text     | Styles for the text content within the placeholder         |


## Popover

The Popover component is a versatile UI element that displays content in a floating container when a trigger element is clicked. It's ideal for displaying additional information or actions without redirecting users away from their current context.

### Usage

```rust
<Popover>
    <PopoverTrigger>
        <Button>{"Default Popover"}</Button>
    </PopoverTrigger>
    <PopoverContent>
        {"Default Popover Content"}
    </PopoverContent>
</Popover>

<Popover>
    <PopoverTrigger>
        <Button variant={ButtonVariant::Secondary}>{"Custom Position"}</Button>
    </PopoverTrigger>
    <PopoverContent position={PopoverPosition::NorthStart}>
        {"Popover Content (North Start)"}
    </PopoverContent>
</Popover>
```

### Components

#### Popover

| Prop     | Type     | Description                                                           |
| -------- | -------- | --------------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the popover component.       |
| class    | Classes  | Additional CSS classes for styling the popover container.             |

#### PopoverTrigger

| Prop     | Type     | Description                                                      |
| -------- | -------- | ---------------------------------------------------------------- |
| children | Children | The child elements to be rendered inside the trigger element.    |
| as_child | bool     | Whether the trigger should be rendered as a child element.       |
| class    | Classes  | Additional CSS classes for styling the trigger.                  |

#### PopoverContent

| Prop     | Type            | Description                                                  |
| -------- | --------------- | ------------------------------------------------------------ |
| children | Children        | The child elements to be rendered inside the popover content.|
| class    | Classes         | Additional CSS classes for styling the content container.    |
| position | PopoverPosition | The position of the popover relative to its trigger.         |

### Styling Classes

| Class Name                     | Description                                                      |
| ------------------------------ | ---------------------------------------------------------------- |
| popover_container              | For the main container of the popover                            |
| popover_trigger                | For the trigger element                                          |
| popover_content                | For the content container                                        |
| popover_position_north_start   | For positioning the popover to the north start of the trigger    |
| popover_position_north_middle  | For positioning the popover to the north middle of the trigger   |
| popover_position_north_end     | For positioning the popover to the north end of the trigger      |
| popover_position_south_start   | For positioning the popover to the south start of the trigger    |
| popover_position_south_middle  | For positioning the popover to the south middle of the trigger   |
| popover_position_south_end     | For positioning the popover to the south end of the trigger      |
| popover_position_east_start    | For positioning the popover to the east start of the trigger     |
| popover_position_east_middle   | For positioning the popover to the east middle of the trigger    |
| popover_position_east_end      | For positioning the popover to the east end of the trigger       |
| popover_position_west_start    | For positioning the popover to the west start of the trigger     |
| popover_position_west_middle   | For positioning the popover to the west middle of the trigger    |
| popover_position_west_end      | For positioning the popover to the west end of the trigger       |

## Resizable

The Resizable component allows you to create resizable areas within your application. Users can resize these areas using various handles positioned around the element's edges and corners.

### Usage

```rust
// State to manage coordinates
let coordinates = use_state(|| (100., 100., 400., 400.));
let on_coordinates_change = {
    let coordinates = coordinates.clone();
    Callback::from(move |new_coords: (f64, f64, f64, f64)| {
        coordinates.set(new_coords);
    })
};

<Resizable
    coordinates={*coordinates}
    north=true
    east=true
    south=true
    west=true
    north_east=true
    south_east=true
    south_west=true
    north_west=true
    on_coordinates_change={on_coordinates_change}
>
    <div class="border-2 border-blue-500 border-dashed absolute bg-gray-500 p-4">
        { "Resizable Content" }
    </div>
</Resizable>
```

### Props

| Prop                  | Type                      | Description                                                     |
| --------------------- | ------------------------- | --------------------------------------------------------------- |
| coordinates           | (f64, f64, f64, f64)      | Initial coordinates of the resizable area: (start_x, start_y, end_x, end_y) |
| on_coordinates_change | Callback<(f64, f64, f64, f64)> | Callback function that is called when the coordinates change |
| children              | Children                  | The child elements to be rendered inside the resizable area     |
| north                 | bool                      | Enable the north handle for resizing                            |
| north_west            | bool                      | Enable the north-west corner handle for resizing                |
| north_east            | bool                      | Enable the north-east corner handle for resizing                |
| east                  | bool                      | Enable the east handle for resizing                             |
| south_east            | bool                      | Enable the south-east corner handle for resizing                |
| south                 | bool                      | Enable the south handle for resizing                            |
| south_west            | bool                      | Enable the south-west corner handle for resizing                |
| west                  | bool                      | Enable the west handle for resizing                             |

### Styling Classes

| Class Name               | Description                                            |
| ------------------------ | ------------------------------------------------------ |
| resizable_container      | For the main container of the resizable component      |
| resizable_box            | For the resizable box itself                           |
| resizable_handle_visible | For visible resize handles                             |
| resizable_handle_hidden  | For hidden resize handles                              |
| resizable_handle_nw      | For the northwest corner handle                        |
| resizable_handle_ne      | For the northeast corner handle                        |
| resizable_handle_sw      | For the southwest corner handle                        |
| resizable_handle_se      | For the southeast corner handle                        |
| resizable_handle_n       | For the north edge handle                              |
| resizable_handle_s       | For the south edge handle                              |
| resizable_handle_w       | For the west edge handle                               |
| resizable_handle_e       | For the east edge handle                               |

## Select

The Select component is a customizable dropdown that allows users to choose an option from a list. It provides a clean and intuitive interface for selection tasks, with support for various data types.

### Usage

```rust
<Select<String>
    options={vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]}
    selected={None::<String>}
    onchange={Callback::from(|selected| {
        // Handle the selection
    })}
/>
```

### Props

| Prop     | Type          | Description                                           |
| -------- | ------------- | ----------------------------------------------------- |
| options  | Vec<T>        | A vector of options to display in the dropdown.       |
| selected | Option<T>     | The currently selected option, if any.                |
| onchange | Callback<T>   | A callback function that is called when an option is selected. |

### Template Parameters

| Parameter | Description                                                         |
| --------- | ------------------------------------------------------------------- |
| T         | The type of the options. Must implement Clone, PartialEq, ToString, and 'static. |

### Styling Classes

| Class Name                | Description                                          |
| ------------------------- | ---------------------------------------------------- |
| select_container          | For the main container of the select component       |
| select_trigger            | For the trigger button that opens the dropdown       |
| select_trigger_placeholder| For the placeholder text in the trigger button       |
| select_trigger_icon       | For the dropdown icon in the trigger button          |
| select_content_container  | For the container of the dropdown content            |
| select_content_list       | For the list of options in the dropdown              |
| select_item               | For individual items in the dropdown list            |

## Selectable

The Selectable component provides functionality for selecting and highlighting elements within a selectable area. It uses internal state management to maintain the selection and hover states.

### Usage

```rust
<SelectableArea select_mode={true}>
    <Selectable id="item1">
        { "Selectable Item 1" }
    </Selectable>
    <Selectable id="item2">
        { "Selectable Item 2" }
    </Selectable>
    <SelectableIndicator />
</SelectableArea>
```

### Components

#### Selectable

| Prop       | Type     | Description                                                     |
| ---------- | -------- | --------------------------------------------------------------- |
| id         | String   | The unique identifier for the selectable item.                  |
| children   | VNode    | The child elements to be rendered inside the selectable component. |
| class      | Classes  | CSS classes to apply to the component.                          |
| tag        | String   | The HTML tag to be used for this component.                     |
| select_mode| bool     | Indicates whether the selectable component is in selection mode.|
| style      | String   | Inline styles to apply to the component.                        |

#### SelectableArea

| Prop       | Type                      | Description                                     |
| ---------- | ------------------------- | ----------------------------------------------- |
| children   | Children                  | The child elements to be rendered inside the selectable area. |
| onselect   | Callback<Option<String>>  | Callback function to handle selection changes.  |
| select_mode| bool                      | Indicates whether the selectable area is in selection mode. |

#### SelectableIndicator

| Prop     | Type     | Description                                          |
| -------- | -------- | ---------------------------------------------------- |
| class    | Classes  | CSS classes to apply to the indicator.               |

#### SelectableVTag

| Prop         | Type                  | Description                                        |
| ------------ | --------------------- | -------------------------------------------------- |
| node         | Box<VTag>             | The VTag node to be wrapped by the selectable functionality. |
| node_ref     | NodeRef               | A reference to the DOM node.                        |
| onclick      | Callback<MouseEvent>  | Callback function for handling click events.        |
| id           | String                | The unique identifier for the VTag.                 |
| onmouseenter | Callback<MouseEvent>  | Callback function for handling mouse enter events.  |
| onmouseleave | Callback<MouseEvent>  | Callback function for handling mouse leave events.  |

### Styling Classes

| Class Name          | Description                                             |
| ------------------- | ------------------------------------------------------- |
| selectable_indicator| For the visual indicator of the selected area.          |
| selectable_hover    | For the hover effect on selectable items.               |
| selectable_selected | For the selected state of items.                        |
| selectable_cursor   | For the cursor style on selectable items.               |

## Sidebar

The Sidebar component provides a collapsible side navigation panel that can be used for application menus, navigation links, and other sidebar content.

### Usage

```rust
<Sidebar
    header={html!{
        <div class="p-4 font-semibold text-lg">{ "App Name" }</div>
    }}
    footer={html!{
        <div class="p-4 border-t">
            <div class="flex items-center space-x-2">
                <img src="https://via.placeholder.com/32" class="rounded-full" alt="User avatar" />
                <div>
                    <div class="font-medium">{ "John Doe" }</div>
                    <div class="text-sm text-gray-500">{ "john@example.com" }</div>
                </div>
            </div>
        </div>
    }}
>
    <SidebarHeading>{ "Main Navigation" }</SidebarHeading>
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
                <span>{ "Users" }</span>
            </div>
        </SidebarItem>
    </SidebarMenu>
</Sidebar>
```

### Props

| Prop     | Type          | Description                                          |
| -------- | ------------- | ---------------------------------------------------- |
| children | Children      | The content to display in the sidebar                |
| header   | Option<Html>  | Optional content to display in the sidebar header    |
| footer   | Option<Html>  | Optional content to display in the sidebar footer    |
| folded   | bool          | Whether the sidebar is collapsed (default: false)    |
| class    | String        | Additional CSS classes                               |

### Related Components

- **SidebarHeading** - Used to create section headings in the sidebar
- **SidebarMenu** - Container for sidebar menu items
- **SidebarItem** - Individual navigation item in the sidebar
- **SidebarLink** - Navigation link that supports Yew Router integration

## SwitchButton

The SwitchButton component is a customizable switch that can be toggled on or off. It includes support for custom icons and is designed with accessibility in mind.

### Usage

```rust
let checked = use_state(|| false);
let on_toggle = {
    let checked = checked.clone();
    Callback::from(move |_| {
        checked.set(!*checked);
    })
};

<SwitchButton
    id="switch-1"
    checked={*checked}
    on_toggle={on_toggle}
/>

<SwitchButton
    id="switch-2"
    checked={true}
    on_toggle={Callback::from(|_| {})}
    on_icon={Some(html! { <i class="fas fa-check"></i> })}
    off_icon={Some(html! { <i class="fas fa-times"></i> })}
/>

<SwitchButton
    id="switch-3"
    checked={false}
    on_toggle={Callback::from(|_| {})}
    disabled={true}
/>
```

### Props

| Prop      | Type                     | Description                                                 |
| --------- | ------------------------ | ----------------------------------------------------------- |
| id        | String                   | A unique identifier for the switch button.                  |
| checked   | bool                     | The initial checked state of the switch button.             |
| on_toggle | Callback<MouseEvent>     | A callback function that is called when the switch is toggled. |
| disabled  | bool                     | Disables the switch button if set to true.                  |
| on_icon   | Option<Html>             | An optional icon to display when the switch is in the 'on' state. |
| off_icon  | Option<Html>             | An optional icon to display when the switch is in the 'off' state. |

### Styling Classes

| Class Name                  | Description                                             |
| --------------------------- | ------------------------------------------------------- |
| switch_base                 | Base styling for the switch button container.           |
| switch_thumb                | Styling for the switch button thumb (the moving part).  |
| switch_checked              | Styling applied when the switch is in the checked state.|
| switch_unchecked            | Styling applied when the switch is in the unchecked state.|
| switch_translate_checked    | Controls the position of the thumb when checked.        |
| switch_translate_unchecked  | Controls the position of the thumb when unchecked.      |
| switch_disabled             | Styling applied when the switch is disabled.            |
| switch_label                | Styling for any associated label (visually hidden by default).|

## Table

The Table component provides a structured way to display tabular data with support for headers, body content, and footers.

### Usage

```rust
<Table>
    <TableHead>
        <TableRow>
            <TableCell>{"Name"}</TableCell>
            <TableCell>{"Age"}</TableCell>
        </TableRow>
    </TableHead>
    <TableBody>
        <TableRow>
            <TableCell>{"John"}</TableCell>
            <TableCell>{"30"}</TableCell>
        </TableRow>
        <TableRow>
            <TableCell>{"Jane"}</TableCell>
            <TableCell>{"25"}</TableCell>
        </TableRow>
    </TableBody>
    <TableFooter>
        <TableRow>
            <TableCell>{"Total"}</TableCell>
            <TableCell>{"2 entries"}</TableCell>
        </TableRow>
    </TableFooter>
</Table>
```

### Props

#### Table

| Prop      | Type      | Description                                         |
| --------- | --------- | --------------------------------------------------- |
| children  | Children  | The table rows, head, body, and footer components.  |
| class     | Classes   | Additional CSS classes for the table container.     |

#### TableHead

| Prop      | Type      | Description                                         |
| --------- | --------- | --------------------------------------------------- |
| children  | Children  | The table rows to be rendered in the header.        |
| class     | Classes   | Additional CSS classes for the table head.          |

#### TableBody

| Prop      | Type      | Description                                         |
| --------- | --------- | --------------------------------------------------- |
| children  | Children  | The table rows to be rendered in the body.          |
| class     | Classes   | Additional CSS classes for the table body.          |

#### TableRow

| Prop      | Type      | Description                                         |
| --------- | --------- | --------------------------------------------------- |
| children  | Children  | The table cells to be rendered in the row.          |
| class     | Classes   | Additional CSS classes for the table row.           |

#### TableCell

| Prop      | Type                    | Description                            |
| --------- | ----------------------- | -------------------------------------- |
| children  | Children                | The content for the table cell.        |
| class     | Classes                 | Additional CSS classes for the cell.   |
| colspan   | Option<u32>             | Number of columns this cell spans.     |
| rowspan   | Option<u32>             | Number of rows this cell spans.        |
| onclick   | Option<Callback<MouseEvent>> | Optional callback for click events. |

#### TableFooter

| Prop      | Type      | Description                                         |
| --------- | --------- | --------------------------------------------------- |
| children  | Children  | The table rows to be rendered in the footer.        |
| class     | Classes   | Additional CSS classes for the table footer.        |

### Styling Classes

| Class Name       | Description                                          |
| ---------------- | ---------------------------------------------------- |
| table_container  | For the outermost container of the table             |
| table            | For the actual table element                         |
| table_head       | For the header section of the table                  |
| table_row        | For individual rows                                  |
| table_cell       | For individual cells                                 |
| table_body       | For the main body of the table                       |
| table_footer     | For the footer section of the table                  |

## Tabs

The Tabs component provides a way to organize content into multiple sections that can be displayed one at a time.

### Usage

```rust
<Tabs default_value="tab1">
    <TabsList>
        <TabsTrigger value="tab1">{ "Tab 1" }</TabsTrigger>
        <TabsTrigger value="tab2">{ "Tab 2" }</TabsTrigger>
        <TabsTrigger value="tab3">{ "Tab 3" }</TabsTrigger>
    </TabsList>
    <TabsContent value="tab1">
        { "Tab 1 Content" }
    </TabsContent>
    <TabsContent value="tab2">
        { "Tab 2 Content" }
    </TabsContent>
    <TabsContent value="tab3">
        { "Tab 3 Content" }
    </TabsContent>
</Tabs>
```

### Props

#### Tabs

| Prop          | Type      | Description                                         |
| ------------- | --------- | --------------------------------------------------- |
| children      | Children  | The child elements (TabsList and TabsContent).      |
| default_value | String    | The value of the initially active tab.              |
| class         | Classes   | Additional CSS classes for the tabs container.      |

#### TabsList

| Prop          | Type           | Description                                      |
| ------------- | -------------- | ------------------------------------------------ |
| children      | Children       | The TabsTrigger elements.                        |
| class         | Classes        | Additional CSS classes for the tabs list.        |
| direction     | FlexDirection  | Direction of the tabs list (Row or Column).      |

#### TabsTrigger

| Prop          | Type      | Description                                         |
| ------------- | --------- | --------------------------------------------------- |
| value         | String    | The value of the tab to be activated when clicked.  |
| children      | Children  | The content displayed in the trigger.               |
| class         | Classes   | Additional CSS classes for the trigger.             |

#### TabsContent

| Prop          | Type      | Description                                         |
| ------------- | --------- | --------------------------------------------------- |
| value         | String    | The value of the tab this content belongs to.       |
| children      | Children  | The content to display when this tab is active.     |
| class         | Classes   | Additional CSS classes for the content.             |

### Styling Classes

| Class Name              | Description                                          |
| ----------------------- | ---------------------------------------------------- |
| tabs_container          | For the main container of the tabs component         |
| tabs_list               | For the list of tab triggers                         |
| tabs_trigger            | For the tab triggers/buttons                         |
| tabs_trigger_active     | For the currently active tab trigger                 |
| tabs_trigger_inactive   | For inactive tab triggers                            |
| tabs_content            | For the content area of each tab                     |

## TagInput

The TagInput component allows users to input and manage multiple tags or keywords with optional autocomplete suggestions.

### Usage

```rust
<TagInput
    placeholder="Type and press Enter to add a tag"
    default_value={vec!["React".to_string(), "Rust".to_string()]}
    candidates={Callback::from(|input: String| {
        let suggestions = vec!["React", "Rust", "Ruby", "Remix"];
        suggestions.into_iter()
            .filter(|&s| s.to_lowercase().contains(&input.to_lowercase()))
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    })}
    onupdate={Callback::from(|tags: Vec<String>| {
        // Handle updated tags
    })}
/>
```

### Props

| Prop          | Type                                | Description                                         |
| ------------- | ----------------------------------- | --------------------------------------------------- |
| id            | Option<String>                      | Optional ID for the input element.                  |
| default_value | Vec<String>                         | Initial tags to display.                            |
| candidates    | Option<Callback<String, Vec<String>>> | Callback that returns tag suggestions based on input. |
| onupdate      | Option<Callback<Vec<String>>>       | Callback triggered when tags are added or removed.  |
| placeholder   | String                              | Placeholder text for the input.                     |

### Styling Classes

| Class Name               | Description                                          |
| ------------------------ | ---------------------------------------------------- |
| tag_input_container      | For the main container of the tag input component    |
| tag_input_field          | For the text input field                             |
| tag_item                 | For individual tag items                             |
| tag_item_text            | For the text content of a tag                        |
| tag_item_remove_button   | For the remove button on each tag                    |
| tag_suggestions          | For the container of autocomplete suggestions        |
| tag_suggestion_item      | For individual suggestion items                      |

## TailwindColorPicker

The TailwindColorPicker component provides a color selection interface based on Tailwind CSS color classes, allowing users to pick both colors and shades.

### Usage

```rust
let color = use_state(|| "blue".to_string());
let shade = use_state(|| 500);

let oncolorchange = {
    let color = color.clone();
    let shade = shade.clone();
    Callback::from(move |(new_color, new_shade)| {
        color.set(new_color);
        shade.set(new_shade);
    })
};

<TailwindColorPicker
    color={(*color).clone()}
    shade={*shade}
    {oncolorchange}
/>
```

### Props

| Prop          | Type                      | Description                                         |
| ------------- | ------------------------- | --------------------------------------------------- |
| color         | String                    | Default selected color. Defaults to 'blue'.         |
| shade         | u16                       | Default shade of the color. Defaults to 500.        |
| oncolorchange | Callback<(String, u16)>   | Callback function called when a color is selected.  |

### Styling Classes

| Class Name                       | Description                                          |
| -------------------------------- | ---------------------------------------------------- |
| tailwind_color_picker_container  | For the main container of the color picker           |
| tailwind_color_picker_button     | For the button that toggles the palette              |
| tailwind_color_picker_palette    | For the dropdown container of the color palette      |
| tailwind_color_picker_color      | For individual color options in the palette          |
| tailwind_color_picker_shade      | For individual shade options in the palette          |

## Textarea

The Textarea component provides a multi-line text input field with support for various properties to customize its appearance and behavior.

### Usage

```rust
// Static Textarea
<Textarea
    value="Initial text"
    placeholder="Enter some text..."
    class={classes!("mb-4")}
/>

// Dynamic Textarea
let value = use_state(|| "".to_string());
let oninput = {
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
        let target = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
        value.set(target.value());
    })
};

<Textarea
    value={(*value).clone()}
    oninput={oninput}
    placeholder="Enter some text..."
/>
```

### Props

| Prop        | Type                 | Description                                         |
| ----------- | -------------------- | --------------------------------------------------- |
| value       | String               | The current value of the textarea.                  |
| oninput     | Callback<InputEvent> | A callback for handling input events.               |
| placeholder | String               | Placeholder text for the textarea.                  |
| class       | Classes              | Additional CSS classes for the textarea.            |
| id          | String               | The id attribute of the textarea.                   |
| name        | String               | The name attribute of the textarea.                 |
| disabled    | bool                 | If true, the textarea will be disabled.             |

### Styling Classes

| Class Name     | Description                                          |
| -------------- | ---------------------------------------------------- |
| textarea_base  | Base styling for the textarea component              |

## Toggle

The Toggle component provides a button that can be toggled between two states, often used for text formatting controls or feature toggles.

### Usage

```rust
let checked = use_state(|| false);
let on_toggle = {
    let checked = checked.clone();
    Callback::from(move |_| {
        checked.set(!*checked);
    })
};

<Toggle id="bold-toggle" checked={*checked} on_toggle={on_toggle}>
    <svg class="h-4 w-4" viewBox="0 0 24 24">
        <path d="M6 4h8a4 4 0 0 1 0 8H6z"></path>
        <path d="M6 12h8a4 4 0 0 1 0 8H6z"></path>
    </svg>
</Toggle>
```

### Props

| Prop      | Type                 | Description                                         |
| --------- | -------------------- | --------------------------------------------------- |
| id        | String               | The id of the toggle component.                     |
| checked   | bool                 | The initial checked state of the toggle.            |
| on_toggle | Callback<MouseEvent> | The callback to be called when the toggle is clicked. |
| disabled  | bool                 | Whether the toggle is disabled.                     |
| children  | Children             | The child elements to be rendered inside the toggle. |

### Styling Classes

| Class Name           | Description                                          |
| -------------------- | ---------------------------------------------------- |
| toggle_container     | For the main container of the toggle component       |
| toggle_base          | For the base styling of the toggle                   |
| toggle_checked       | For the toggle when it's in the checked state        |
| toggle_unchecked     | For the toggle when it's in the unchecked state      |
| toggle_disabled      | For the toggle when it's disabled                    |
| toggle_label         | For the label associated with the toggle             |
| toggle_icon          | For the icon inside the toggle                       |

## Topbar

The Topbar component provides a fixed navigation bar at the top of your application for branding, navigation controls, and user actions.

### Usage

```rust
<Topbar>
    <Container class="flex items-center justify-between py-2" variant={ContainerVariant::Large} padding_y={false}>
        <div class="font-semibold text-lg">{ "App Name" }</div>
        <div class="flex items-center space-x-4">
            <Button variant={ButtonVariant::Ghost}>{ "Dashboard" }</Button>
            <Button variant={ButtonVariant::Ghost}>{ "Projects" }</Button>
            <Button variant={ButtonVariant::Ghost}>{ "Team" }</Button>
        </div>
        <div>
            <Button>{ "Sign In" }</Button>
        </div>
    </Container>
</Topbar>
```

### Props

| Prop     | Type     | Description                                         |
| -------- | -------- | --------------------------------------------------- |
| children | Children | The content to display in the topbar.               |
| class    | String   | Additional CSS classes.                             |
| fixed    | bool     | Whether the topbar should be fixed to the top.      |

### Styling Classes

| Class Name     | Description                                          |
| -------------- | ---------------------------------------------------- |
| topbar         | For the main topbar component                        |
| topbar_fixed   | Applied when the topbar is fixed to the viewport     |
| topbar_content | For the content container within the topbar          |

## Typography

Typography components provide styled HTML heading and paragraph elements. Each component applies consistent styling based on your application's brand guidelines.

### Usage

```rust
<div>
    <H1 class={classes!("mb-4")}>{ "This is an H1 Heading" }</H1>
    <H2 class={classes!("mb-4")}>{ "This is an H2 Heading" }</H2>
    <H3 class={classes!("mb-4")}>{ "This is an H3 Heading" }</H3>
    <H4 class={classes!("mb-4")}>{ "This is an H4 Heading" }</H4>
    <H5 class={classes!("mb-4")}>{ "This is an H5 Heading" }</H5>
    <H6 class={classes!("mb-4")}>{ "This is an H6 Heading" }</H6>
    <Paragraph class={classes!("mb-4")}>{ "This is a paragraph with some example text." }</Paragraph>
</div>
```

### Props

| Prop     | Type     | Description                                              |
| -------- | -------- | -------------------------------------------------------- |
| children | Children | Child elements to be rendered inside the component.      |
| class    | Classes  | Additional CSS classes to apply.                         |

### Styling Classes

| Class Name     | Description                         |
| -------------- | ----------------------------------- |
| typography_h1  | For H1 elements                     |
| typography_h2  | For H2 elements                     |
| typography_h3  | For H3 elements                     |
| typography_h4  | For H4 elements                     |
| typography_h5  | For H5 elements                     |
| typography_h6  | For H6 elements                     |
| typography_p   | For paragraph elements              |

## WindowProvider

The WindowProvider component offers a context provider for managing the window object associated with an iframe. This enables descendant components to access and interact with the global window object effectively.

### Usage

```rust
<WindowProvider iframe_ref={NodeRef::default()}>
    <p>{ "Content inside WindowProvider" }</p>
</WindowProvider>
```

### Props

| Prop       | Type    | Description                                              |
| ---------- | ------- | -------------------------------------------------------- |
| children   | Children| The child elements to be rendered inside the provider.   |
| iframe_ref | NodeRef | A node reference to the iframe for which the window object is provided. |

### Styling Classes

| Class Name              | Description                                |
| ----------------------- | ------------------------------------------ |
| window_provider_container | For the main container                   |
