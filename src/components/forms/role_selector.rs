use std::collections::HashSet;
use yew::prelude::*;

/// A role or option to select
#[derive(Clone, Debug, PartialEq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

impl Role {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            color: None,
            icon: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Properties for the role selector component
#[derive(Properties, PartialEq, Clone)]
pub struct RoleSelectorProps {
    /// Available roles to select from
    pub roles: Vec<Role>,

    /// Currently selected role IDs
    #[prop_or_default]
    pub selected: Vec<String>,

    /// Callback when selection changes
    pub on_change: Callback<Vec<String>>,

    /// Whether multiple roles can be selected
    #[prop_or(false)]
    pub multiple: bool,

    /// Custom CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Theme name (light or dark)
    #[prop_or_else(|| "light".to_string())]
    pub theme: String,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Display mode: "grid" or "list"
    #[prop_or_else(|| "grid".to_string())]
    pub display_mode: String,
}

#[function_component(RoleSelector)]
pub fn role_selector(props: &RoleSelectorProps) -> Html {
    let selected_set = use_state(|| {
        let mut set = HashSet::new();
        for id in &props.selected {
            set.insert(id.clone());
        }
        set
    });

    let theme_class = if props.theme == "dark" {
        "dark"
    } else {
        "light"
    };

    // Update selected set when props change
    {
        let selected_set = selected_set.clone();
        let prop_selected = props.selected.clone();
        use_effect_with(prop_selected.clone(), move |_| {
            let mut set = HashSet::new();
            for id in &prop_selected {
                set.insert(id.clone());
            }
            selected_set.set(set);
            || ()
        });
    }

    let on_role_click = {
        let selected_set = selected_set.clone();
        let on_change = props.on_change.clone();
        let multiple = props.multiple;
        let disabled = props.disabled;

        Callback::from(move |role_id: String| {
            if disabled {
                return;
            }

            let mut new_set = (*selected_set).clone();

            if multiple {
                if new_set.contains(&role_id) {
                    new_set.remove(&role_id);
                } else {
                    new_set.insert(role_id);
                }
            } else {
                new_set.clear();
                new_set.insert(role_id);
            }

            let selected_vec: Vec<String> = new_set.iter().cloned().collect();
            selected_set.set(new_set);
            on_change.emit(selected_vec);
        })
    };

    let grid_class = if props.display_mode == "grid" {
        "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3"
    } else {
        "flex flex-col gap-2"
    };

    html! {
        <div
            class={classes!(
                props.class.clone(),
                "role-selector",
                theme_class
            )}
        >
            <div class={grid_class}>
                { for props.roles.iter().map(|role| {
                    let is_selected = selected_set.contains(&role.id);
                    let role_id = role.id.clone();
                    let on_click = {
                        let on_role_click = on_role_click.clone();
                        let role_id = role_id.clone();
                        Callback::from(move |_e: MouseEvent| {
                            on_role_click.emit(role_id.clone());
                        })
                    };

                    html! {
                        <div
                            class={classes!(
                                "role-card",
                                "p-4",
                                "border-2",
                                "rounded-lg",
                                "cursor-pointer",
                                "transition-all",
                                if is_selected {
                                    "border-blue-500 bg-blue-50 dark:bg-blue-900"
                                } else {
                                    "border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 hover:border-gray-400 dark:hover:border-gray-600"
                                },
                                if props.disabled {
                                    "opacity-50 cursor-not-allowed"
                                } else {
                                    ""
                                }
                            )}
                            onclick={if !props.disabled { Some(on_click) } else { None }}
                        >
                            <div class="flex items-start gap-3">
                                // Icon or colored circle
                                <div class="flex-shrink-0">
                                    if let Some(icon) = &role.icon {
                                        <div class="text-2xl">{ icon }</div>
                                    } else if let Some(color) = &role.color {
                                        <div
                                            class="w-8 h-8 rounded-full"
                                            style={format!("background-color: {}", color)}
                                        />
                                    } else {
                                        <div class="w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600" />
                                    }
                                </div>

                                // Role info
                                <div class="flex-grow">
                                    <div class="flex items-center gap-2">
                                        <h3 class="font-semibold text-gray-900 dark:text-white">
                                            { &role.name }
                                        </h3>
                                        if is_selected {
                                            <svg
                                                class="w-5 h-5 text-blue-500"
                                                fill="currentColor"
                                                viewBox="0 0 20 20"
                                            >
                                                <path
                                                    fill-rule="evenodd"
                                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                                    clip-rule="evenodd"
                                                />
                                            </svg>
                                        }
                                    </div>

                                    if let Some(description) = &role.description {
                                        <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                                            { description }
                                        </p>
                                    }
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
