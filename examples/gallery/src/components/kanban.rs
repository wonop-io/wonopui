use super::example_code::ExampleCode;
use crate::api_section::ApiSection;
use crate::features_section::Features;
use crate::notes_section::NotesSection;
use crate::styling_section::StylingSection;
use std::cmp::Ordering;
use wonopui::prelude::{BrandGuideType as BrandGuide, ClassesStr, ComponentEditor};
use wonopui::*;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[function_component(KanbanThemeEditor)]
pub fn kanban_theme_editor() -> Html {
    let fields = vec![
        (
            "kanban_container".to_string(),
            "Kanban Container".to_string(),
        ),
        ("kanban_column".to_string(), "Kanban Column".to_string()),
        (
            "kanban_column_header".to_string(),
            "Column Header".to_string(),
        ),
        ("kanban_column_body".to_string(), "Column Body".to_string()),
        (
            "kanban_column_over".to_string(),
            "Column Drop Highlight".to_string(),
        ),
        ("kanban_card".to_string(), "Kanban Card".to_string()),
        ("kanban_card_title".to_string(), "Card Title".to_string()),
        (
            "kanban_card_content".to_string(),
            "Card Content".to_string(),
        ),
        (
            "kanban_card_dragging".to_string(),
            "Card Dragging State".to_string(),
        ),
        (
            "kanban_card_drag_target".to_string(),
            "Card Drop Target Highlight".to_string(),
        ),
    ];

    let preview = html! {
        <Kanban class="h-[400px]">
            <KanbanColumn id="todo" title="To Do">
                <KanbanCard id="task1" title="Task 1" order={0}>
                    {"Complete the project documentation"}
                </KanbanCard>
                <KanbanCard id="task2" title="Task 2" order={1}>
                    {"Set up CI/CD pipeline"}
                </KanbanCard>
            </KanbanColumn>
            <KanbanColumn id="progress" title="In Progress">
                <KanbanCard id="task3" title="Task 3" order={0}>
                    {"Implement user authentication"}
                </KanbanCard>
            </KanbanColumn>
            <KanbanColumn id="done" title="Done">
                <KanbanCard id="task4" title="Task 4" order={0}>
                    {"Project setup"}
                </KanbanCard>
            </KanbanColumn>
        </Kanban>
    };

    html! {
        <ComponentEditor
            fields={fields}
            preview={preview}
        />
    }
}

#[function_component(KanbanExample)]
pub fn kanban_example() -> Html {
    let cards = use_state(|| {
        vec![
            (
                String::from("task1"),
                String::from("todo"),
                String::from("Project Setup"),
                String::from("Configure repository and install dependencies"),
                0,
            ),
            (
                String::from("task2"),
                String::from("todo"),
                String::from("Design UI"),
                String::from("Create wireframes and mockups"),
                1,
            ),
            (
                String::from("task3"),
                String::from("progress"),
                String::from("Implementation"),
                String::from("Start coding the core features"),
                0,
            ),
            (
                String::from("task4"),
                String::from("progress"),
                String::from("API Integration"),
                String::from("Connect to backend services"),
                1,
            ),
            (
                String::from("task5"),
                String::from("done"),
                String::from("Requirements"),
                String::from("Gather project requirements"),
                0,
            ),
            (
                String::from("task6"),
                String::from("done"),
                String::from("Planning"),
                String::from("Create project roadmap"),
                1,
            ),
        ]
    });

    let ondrop = {
        let cards = cards.clone();
        Callback::from(
            move |(card_id, column_id, target_card_id): (String, String, Option<String>)| {
                let mut updated_cards = (*cards).clone();

                // Find maximum order for the destination column
                let max_order = updated_cards
                    .iter()
                    .filter(|(_, col, _, _, _)| *col == column_id)
                    .map(|(_, _, _, _, order)| *order)
                    .max()
                    .unwrap_or(0)
                    + 1;

                // Find the card index and process the target position before modifying card
                let card_index = updated_cards
                    .iter()
                    .position(|(id, _, _, _, _)| *id == card_id);

                // Process target_card_id and get target order if applicable
                let target_order = if let Some(target_id) = &target_card_id {
                    updated_cards
                        .iter()
                        .position(|(id, col, _, _, _)| id == target_id && *col == column_id)
                        .map(|pos| updated_cards[pos].4)
                } else {
                    None
                };

                // Now apply the changes using the card index
                if let Some(index) = card_index {
                    // Update column
                    updated_cards[index].1 = column_id.clone();

                    if let Some(order) = target_order {
                        // Shift all cards with order >= target_order up by 1
                        for c in updated_cards.iter_mut() {
                            if c.1 == column_id && c.0 != card_id && c.4 >= order {
                                c.4 += 1;
                            }
                        }

                        // Set the dropped card to the target order
                        updated_cards[index].4 = order;
                    } else {
                        // If no target or target not found, put at end
                        updated_cards[index].4 = max_order;
                    }
                }

                cards.set(updated_cards);
            },
        )
    };

    let render_cards = |column_id: &str| -> Vec<VChild<KanbanCard>> {
        let mut column_cards: Vec<_> = cards
            .iter()
            .filter(|(_, column, _, _, _)| column == column_id)
            .collect();

        // Sort by order
        column_cards.sort_by(|(_, _, _, _, order1), (_, _, _, _, order2)| order1.cmp(order2));

        column_cards
            .into_iter()
            .map(|(id, _, title, content, order)| {
                let id = id.clone();
                let title = title.clone();
                let content = content.clone();
                let order = *order;
                html_nested! {
                    <KanbanCard id={id} title={title} order={order}>
                        {content}
                    </KanbanCard>
                }
            })
            .collect::<Vec<_>>()
    };

    html! {
        <div class="w-full">
            <div class="mb-4 p-4 bg-green-50 dark:bg-green-900/30 rounded-md">
                <p class="text-sm text-green-800 dark:text-green-500">
                    {"Try dragging the cards between columns to see the Kanban board in action!"}
                </p>
            </div>
            <Kanban>
                <KanbanColumn id="todo" title="To Do" ondrop={ondrop.clone()}>
                    {for render_cards("todo")}
                </KanbanColumn>
                <KanbanColumn id="progress" title="In Progress" ondrop={ondrop.clone()}>
                    {for render_cards("progress")}
                </KanbanColumn>
                <KanbanColumn id="done" title="Done" ondrop={ondrop.clone()}>
                    {for render_cards("done")}
                </KanbanColumn>
            </Kanban>
        </div>
    }
}

#[function_component(KanbanDocumentation)]
pub fn kanban_documentation() -> Html {
    html! {
        <Container variant={ContainerVariant::Large} class="bg-white dark:bg-zinc-900 min-h-screen">
            <h1 class="text-3xl font-bold mb-4 text-zinc-900 dark:text-white">
                { "Kanban Component" }
            </h1>
            <p class="mb-6 text-zinc-600 dark:text-zinc-400">
                { "The Kanban component is a flexible task management system that allows users to organize tasks into columns and drag cards between them, following the Kanban methodology." }
            </p>

            <h2 class="text-2xl font-semibold mb-4 text-zinc-900 dark:text-white">
                { "Example" }
            </h2>
            <ExampleCode
                preview={html! {
                    <KanbanExample />
                }}
                customize={html! {
                    <KanbanThemeEditor />
                }}
                code={r#"
// First, define your cards state with ordering
let cards = use_state(|| vec![
    (String::from("task1"), String::from("todo"), String::from("Project Setup"), String::from("Configure repository and install dependencies"), 0),
    (String::from("task2"), String::from("todo"), String::from("Design UI"), String::from("Create wireframes and mockups"), 1),
    (String::from("task3"), String::from("progress"), String::from("Implementation"), String::from("Start coding the core features"), 0),
    (String::from("task4"), String::from("progress"), String::from("API Integration"), String::from("Connect to backend services"), 1),
    (String::from("task5"), String::from("done"), String::from("Requirements"), String::from("Gather project requirements"), 0),
    (String::from("task6"), String::from("done"), String::from("Planning"), String::from("Create project roadmap"), 1),
]);

// Create the drop handler to update card positions
let ondrop = {
    let cards = cards.clone();
    Callback::from(move |(card_id, column_id, target_card_id): (String, String, Option<String>)| {
        let mut updated_cards = (*cards).clone();

        // Find maximum order for the destination column
        let max_order = updated_cards
            .iter()
            .filter(|(_, col, _, _, _)| *col == column_id)
            .map(|(_, _, _, _, order)| *order)
            .max()
            .unwrap_or(0) + 1;

        // Find the card index and process the target position before modifying card
        let card_index = updated_cards
            .iter()
            .position(|(id, _, _, _, _)| *id == card_id);

        // Process target_card_id and get target order if applicable
        let target_order = if let Some(target_id) = &target_card_id {
            updated_cards
                .iter()
                .position(|(id, col, _, _, _)| id == target_id && *col == column_id)
                .map(|pos| updated_cards[pos].4)
        } else {
            None
        };

        // Now apply the changes using the card index
        if let Some(index) = card_index {
            // Update column
            updated_cards[index].1 = column_id.clone();

            if let Some(order) = target_order {
                // Shift all cards with order >= target_order up by 1
                for c in updated_cards.iter_mut() {
                    if c.1 == column_id && c.0 != card_id && c.4 >= order {
                        c.4 += 1;
                    }
                }

                // Set the dropped card to the target order
                updated_cards[index].4 = order;
            } else {
                // If no target or target not found, put at end
                updated_cards[index].4 = max_order;
            }
        }

        cards.set(updated_cards);
    })
};

// Helper function to render cards for a column
let render_cards = |column_id: &str| -> Vec<VChild<KanbanCard>> {
    let mut column_cards: Vec<_> = cards
        .iter()
        .filter(|(_, column, _, _, _)| column == column_id)
        .collect();

    // Sort by order
    column_cards.sort_by(|(_, _, _, _, order1), (_, _, _, _, order2)| order1.cmp(order2));

    column_cards
        .into_iter()
        .map(|(id, _, title, content, order)| {
            let id = id.clone();
            let title = title.clone();
            let content = content.clone();
            let order = *order;
            html_nested! {
                <KanbanCard id={id} title={title} order={order}>
                    {content}
                </KanbanCard>
            }
        })
        .collect::<Vec<_>>()
};

// Render the Kanban board
html! {
    <Kanban>
        <KanbanColumn id="todo" title="To Do" ondrop={ondrop.clone()}>
            {for render_cards("todo")}
        </KanbanColumn>
        <KanbanColumn id="progress" title="In Progress" ondrop={ondrop.clone()}>
            {for render_cards("progress")}
        </KanbanColumn>
        <KanbanColumn id="done" title="Done" ondrop={ondrop.clone()}>
            {for render_cards("done")}
        </KanbanColumn>
    </Kanban>
}"#.to_string()}
            />

            <Features features={vec![
                "Drag and drop cards between columns",
                "Proper card ordering within columns",
                "Customizable column and card styles",
                "Visual feedback during drag operations",
                "Support for multiple columns with custom titles",
                "Responsive design that works on both desktop and mobile",
                "Custom card content and titles"
            ]} />

            <h2 class="text-2xl font-semibold mt-8 mb-4 text-zinc-900 dark:text-white">
                { "API" }
            </h2>

            <ApiSection
                title="Kanban"
                description="The main container component for the Kanban board."
                props={vec![
                    ("children", "Children", "The columns to display on the board."),
                    ("class", "Classes", "Additional CSS classes for the Kanban container."),
                    ("allow_multiple_column_drops", "bool", "If true, allows dropping cards on multiple columns simultaneously. Default is false."),
                ]}
            />

            <ApiSection
                title="KanbanColumn"
                description="A column in the Kanban board that contains cards."
                props={vec![
                    ("children", "ChildrenWithProps<KanbanCard>", "The cards to display in the column."),
                    ("class", "Classes", "Additional CSS classes for the column."),
                    ("header_class", "Classes", "Additional CSS classes for the column header."),
                    ("body_class", "Classes", "Additional CSS classes for the column body."),
                    ("id", "AttrValue", "Unique identifier for the column."),
                    ("title", "AttrValue", "The title to display in the column header."),
                    ("ondragenter", "Option<Callback<DragEvent>>", "Callback when a draggable item enters the column."),
                    ("ondragleave", "Option<Callback<DragEvent>>", "Callback when a draggable item leaves the column."),
                    ("ondragover", "Option<Callback<DragEvent>>", "Callback when a draggable item is over the column."),
                    ("ondrop", "Option<Callback<(String, String, Option<String>)>>", "Callback when a card is dropped on the column. Receives (card_id, column_id, target_card_id)."),
                ]}
            />

            <ApiSection
                title="KanbanCard"
                description="A draggable card within a Kanban column."
                props={vec![
                    ("children", "Children", "The content to display in the card."),
                    ("class", "Classes", "Additional CSS classes for the card."),
                    ("id", "AttrValue", "Unique identifier for the card."),
                    ("column_id", "Option<AttrValue>", "ID of the column the card belongs to. This is set automatically by the KanbanColumn."),
                    ("title", "Option<AttrValue>", "Optional title to display at the top of the card."),
                    ("order", "Option<usize>", "Ordering position within the column. Cards are sorted by this value."),
                    ("ondragstart", "Option<Callback<DragEvent>>", "Callback when dragging of the card starts."),
                    ("ondragend", "Option<Callback<DragEvent>>", "Callback when dragging of the card ends."),
                    ("ondragover", "Option<Callback<DragEvent>>", "Callback when a draggable item is over the card."),
                    ("ondragenter", "Option<Callback<DragEvent>>", "Callback when a draggable item enters the card."),
                    ("ondragleave", "Option<Callback<DragEvent>>", "Callback when a draggable item leaves the card."),
                    ("onclick", "Option<Callback<MouseEvent>>", "Callback when the card is clicked."),
                ]}
            />

            <NotesSection
                title={"Usage Notes".to_string()}
                notes={vec![
                    "Each card must have a unique `id` to enable proper drag and drop functionality.".to_string(),
                    "Each column must have a unique `id` to properly identify where cards are dropped.".to_string(),
                    "Cards are automatically assigned the column_id of their parent column.".to_string(),
                    "The ondrop callback receives a tuple of (card_id, column_id, target_card_id) to support proper card ordering.".to_string(),
                    "Adding an order property to cards ensures they maintain proper positioning within columns.".to_string(),
                    "Cards will be reordered when dropped on or between other cards based on the target_card_id.".to_string(),
                    "The Kanban component uses the HTML5 Drag and Drop API, which provides native support across modern browsers.".to_string(),
                    "For accessibility, ensure you include proper ARIA attributes and keyboard navigation alternatives.".to_string(),
                    "The component provides visual feedback during drag operations through CSS classes that can be customized.".to_string(),
                ]}
            />

            <StylingSection
                component_name={"Kanban".to_string()}
                class_descriptions={vec![
                    ("kanban_container".to_string(), "Styles for the main Kanban board container.".to_string()),
                    ("kanban_column".to_string(), "Styles for each column in the Kanban board.".to_string()),
                    ("kanban_column_header".to_string(), "Styles for the column header containing the title.".to_string()),
                    ("kanban_column_body".to_string(), "Styles for the column body that contains the cards.".to_string()),
                    ("kanban_column_over".to_string(), "Styles applied to a column when a card is being dragged over it.".to_string()),
                    ("kanban_card".to_string(), "Styles for each card in the Kanban board.".to_string()),
                    ("kanban_card_title".to_string(), "Styles for the card title.".to_string()),
                    ("kanban_card_content".to_string(), "Styles for the card content area.".to_string()),
                    ("kanban_card_dragging".to_string(), "Styles applied to a card when it's being dragged.".to_string()),
                    ("kanban_card_drag_target".to_string(), "Styles applied to a card when another card is being dragged over it.".to_string()),
                ]}
            />
        </Container>
    }
}
