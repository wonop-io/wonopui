use wonopui::code_editor::{DiffView, DiffViewMode};
use wonopui::prelude::*;
use yew::prelude::*;
use web_sys::HtmlSelectElement;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct DiffExample {
    pub title: &'static str,
    pub description: &'static str,
    pub old_text: &'static str,
    pub new_text: &'static str,
    pub language: &'static str,
    pub show_unified: bool,
    pub context_lines: usize,
}

impl DiffExample {
    pub fn render(&self, mode: DiffViewMode, show_line_numbers: bool, theme: &str) -> Html {
        html! {
            <div>
                <h3 class="text-lg font-semibold mb-2 text-zinc-800 dark:text-zinc-200">
                    { self.title }
                </h3>
                <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-3">
                    { self.description }
                </p>
                <DiffView
                    old_text={self.old_text.to_string()}
                    new_text={self.new_text.to_string()}
                    language={self.language.to_string()}
                    mode={mode.clone()}
                    show_line_numbers={show_line_numbers}
                    unified_diff={self.show_unified}
                    context_lines={self.context_lines}
                    theme={theme.to_string()}
                    font_size={"13px"}
                    line_height={"1.5"}
                    class="shadow-md"
                />
            </div>
        }
    }
}

pub fn get_use_case_examples() -> Vec<DiffExample> {
    vec![
        // Code Review Example
        DiffExample {
            title: "Code Review - Performance Optimization",
            description: "Shows a typical performance optimization during code review, replacing a loop with a more efficient iterator method.",
            old_text: r#"// Calculate the sum of squares
fn sum_of_squares(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..numbers.len() {
        result += numbers[i] * numbers[i];
    }
    result
}

// Find the maximum value
fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    let mut max = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] > max {
            max = numbers[i];
        }
    }
    Some(max)
}"#,
            new_text: r#"// Calculate the sum of squares using iterator
fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.iter()
        .map(|&n| n * n)
        .sum()
}

// Find the maximum value using iterator
fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().max()
}"#,
            language: "rust",
            show_unified: true,
            context_lines: 2,
        },
        
        // Bug Fix Example
        DiffExample {
            title: "Bug Fix - Null Pointer Exception",
            description: "Demonstrates fixing a potential null pointer exception by adding proper error handling.",
            old_text: r#"class UserService {
    private UserRepository repository;
    
    public User getUser(Long id) {
        User user = repository.findById(id);
        user.setLastAccessed(new Date());
        return user;
    }
    
    public String getUserEmail(Long id) {
        User user = repository.findById(id);
        return user.getEmail().toLowerCase();
    }
}"#,
            new_text: r#"class UserService {
    private UserRepository repository;
    
    public User getUser(Long id) {
        User user = repository.findById(id);
        if (user != null) {
            user.setLastAccessed(new Date());
        }
        return user;
    }
    
    public String getUserEmail(Long id) {
        User user = repository.findById(id);
        if (user == null || user.getEmail() == null) {
            return null;
        }
        return user.getEmail().toLowerCase();
    }
}"#,
            language: "java",
            show_unified: true,
            context_lines: 3,
        },
        
        // API Migration Example
        DiffExample {
            title: "API Migration - Async/Await Update",
            description: "Shows migration from callback-based API to modern async/await syntax.",
            old_text: r#"function fetchUserData(userId, callback) {
    fetch(`/api/users/${userId}`)
        .then(response => response.json())
        .then(data => {
            callback(null, data);
        })
        .catch(error => {
            callback(error, null);
        });
}

function processUser(userId) {
    fetchUserData(userId, (error, data) => {
        if (error) {
            console.error('Failed to fetch user:', error);
            return;
        }
        console.log('User data:', data);
        updateUI(data);
    });
}"#,
            new_text: r#"async function fetchUserData(userId) {
    const response = await fetch(`/api/users/${userId}`);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
}

async function processUser(userId) {
    try {
        const data = await fetchUserData(userId);
        console.log('User data:', data);
        updateUI(data);
    } catch (error) {
        console.error('Failed to fetch user:', error);
        showErrorMessage(error.message);
    }
}"#,
            language: "javascript",
            show_unified: true,
            context_lines: 2,
        },
        
        // Configuration Update Example
        DiffExample {
            title: "Configuration Update - Docker Optimization",
            description: "Optimizing a Dockerfile for better caching and smaller image size.",
            old_text: r#"FROM node:16
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build
EXPOSE 3000
CMD ["npm", "start"]"#,
            new_text: r#"FROM node:16-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

FROM node:16-alpine
WORKDIR /app
COPY --from=builder /app/node_modules ./node_modules
COPY . .
RUN npm run build
EXPOSE 3000
USER node
CMD ["npm", "start"]"#,
            language: "dockerfile",
            show_unified: false,
            context_lines: 1,
        },
        
        // Schema Evolution Example
        DiffExample {
            title: "Database Schema Evolution",
            description: "Adding new fields and constraints to a database table definition.",
            old_text: r#"CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100),
    created_at TIMESTAMP
);"#,
            new_text: r#"CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);"#,
            language: "sql",
            show_unified: true,
            context_lines: 3,
        },
        
        // CSS Refactoring Example
        DiffExample {
            title: "CSS Modernization - Grid Layout",
            description: "Refactoring old float-based layout to modern CSS Grid.",
            old_text: r#".container {
    width: 100%;
    overflow: hidden;
}

.sidebar {
    float: left;
    width: 250px;
    margin-right: 20px;
}

.main-content {
    float: left;
    width: calc(100% - 270px);
}

.clearfix::after {
    content: "";
    display: table;
    clear: both;
}"#,
            new_text: r#".container {
    display: grid;
    grid-template-columns: 250px 1fr;
    gap: 20px;
    width: 100%;
}

.sidebar {
    /* Sidebar styles */
}

.main-content {
    /* Main content styles */
}

@media (max-width: 768px) {
    .container {
        grid-template-columns: 1fr;
    }
}"#,
            language: "css",
            show_unified: false,
            context_lines: 2,
        },
        
        // Type Safety Example
        DiffExample {
            title: "TypeScript - Adding Type Safety",
            description: "Converting JavaScript to TypeScript with proper type annotations.",
            old_text: r#"function processOrder(order) {
    const total = order.items.reduce((sum, item) => {
        return sum + (item.price * item.quantity);
    }, 0);
    
    const tax = total * 0.08;
    const shipping = total > 100 ? 0 : 10;
    
    return {
        subtotal: total,
        tax: tax,
        shipping: shipping,
        total: total + tax + shipping
    };
}"#,
            new_text: r#"interface OrderItem {
    id: string;
    name: string;
    price: number;
    quantity: number;
}

interface Order {
    id: string;
    customerId: string;
    items: OrderItem[];
}

interface OrderSummary {
    subtotal: number;
    tax: number;
    shipping: number;
    total: number;
}

function processOrder(order: Order): OrderSummary {
    const subtotal = order.items.reduce((sum, item) => {
        return sum + (item.price * item.quantity);
    }, 0);
    
    const tax = subtotal * 0.08;
    const shipping = subtotal > 100 ? 0 : 10;
    
    return {
        subtotal,
        tax,
        shipping,
        total: subtotal + tax + shipping
    };
}"#,
            language: "typescript",
            show_unified: true,
            context_lines: 3,
        },
        
        // Test Update Example
        DiffExample {
            title: "Test Enhancement - Better Assertions",
            description: "Improving test coverage and assertion quality.",
            old_text: r#"def test_user_creation():
    user = User("john", "john@example.com")
    assert user.username == "john"
    assert user.email == "john@example.com""#,
            new_text: r#"import pytest
from datetime import datetime

def test_user_creation():
    # Arrange
    username = "john"
    email = "john@example.com"
    
    # Act
    user = User(username, email)
    
    # Assert
    assert user.username == username
    assert user.email == email
    assert user.id is not None
    assert isinstance(user.created_at, datetime)
    assert user.is_active is True

def test_user_creation_with_invalid_email():
    # Test that invalid email raises ValueError
    with pytest.raises(ValueError, match="Invalid email format"):
        User("john", "invalid-email")

def test_user_creation_with_empty_username():
    # Test that empty username raises ValueError
    with pytest.raises(ValueError, match="Username cannot be empty"):
        User("", "john@example.com")"#,
            language: "python",
            show_unified: true,
            context_lines: 2,
        },
    ]
}

#[function_component(UseCaseExamples)]
pub fn use_case_examples() -> Html {
    let examples = get_use_case_examples();
    let selected_example = use_state(|| 0usize);
    let mode = use_state(|| DiffViewMode::SideBySide);
    let show_line_numbers = use_state(|| true);
    let theme = use_state(|| "auto".to_string());
    
    let on_example_change = {
        let selected_example = selected_example.clone();
        Callback::from(move |idx: usize| {
            selected_example.set(idx);
        })
    };
    
    let on_mode_toggle = {
        let mode = mode.clone();
        Callback::from(move |_| {
            mode.set(match *mode {
                DiffViewMode::SideBySide | DiffViewMode::Split => DiffViewMode::Inline,
                DiffViewMode::Inline | DiffViewMode::Unified => DiffViewMode::SideBySide,
            });
        })
    };
    
    let on_line_numbers_toggle = {
        let show_line_numbers = show_line_numbers.clone();
        Callback::from(move |_| {
            show_line_numbers.set(!*show_line_numbers);
        })
    };
    
    let on_theme_change = {
        let theme = theme.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                theme.set(select.value());
            }
        })
    };
    
    html! {
        <div class="space-y-6">
            // Example selector
            <div class="flex flex-wrap gap-2">
                {
                    for examples.iter().enumerate().map(|(idx, example)| {
                        let onclick = {
                            let on_example_change = on_example_change.clone();
                            Callback::from(move |_| on_example_change.emit(idx))
                        };
                        
                        html! {
                            <button
                                class={classes!(
                                    "px-3", "py-2", "rounded-md", "text-sm", "font-medium",
                                    "transition-all", "duration-200",
                                    if idx == *selected_example {
                                        "bg-blue-500 text-white shadow-md transform scale-105"
                                    } else {
                                        "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700"
                                    }
                                )}
                                onclick={onclick}
                            >
                                { example.title }
                            </button>
                        }
                    })
                }
            </div>
            
            // Controls
            <div class="flex flex-wrap gap-4 p-4 bg-gray-100 dark:bg-gray-800 rounded-md">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded-sm hover:bg-blue-600 transition-colors"
                    onclick={on_mode_toggle}
                >
                    { format!("Mode: {}", if matches!(*mode, DiffViewMode::SideBySide) { "Side-by-Side" } else { "Inline" }) }
                </button>
                
                <button
                    class={classes!(
                        "px-4", "py-2", "rounded-sm", "transition-colors",
                        if *show_line_numbers {
                            "bg-green-500 text-white hover:bg-green-600"
                        } else {
                            "bg-gray-500 text-white hover:bg-gray-600"
                        }
                    )}
                    onclick={on_line_numbers_toggle}
                >
                    { format!("Line Numbers: {}", if *show_line_numbers { "On" } else { "Off" }) }
                </button>
                
                <select
                    class="px-4 py-2 rounded-sm border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700"
                    onchange={on_theme_change}
                    value={(*theme).clone()}
                >
                    <option value="auto">{"Auto Theme"}</option>
                    <option value="light">{"Light Theme"}</option>
                    <option value="dark">{"Dark Theme"}</option>
                </select>
            </div>
            
            // Render selected example
            <div class="border-2 border-gray-200 dark:border-gray-700 rounded-md overflow-hidden">
                { examples[*selected_example].render((*mode).clone(), *show_line_numbers, &theme) }
            </div>
        </div>
    }
}