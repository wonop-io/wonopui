use wonopui::code_editor::{DiffView, DiffViewMode};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

pub struct LanguageExample {
    pub name: &'static str,
    pub extension: &'static str,
    pub old_code: &'static str,
    pub new_code: &'static str,
}

pub fn get_language_examples() -> Vec<LanguageExample> {
    vec![
        LanguageExample {
            name: "Rust",
            extension: "rust",
            old_code: r#"fn process_data(items: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in items {
        sum += item;
    }
    sum
}"#,
            new_code: r#"fn process_data(items: Vec<i32>) -> i32 {
    items.iter().sum()
}"#,
        },
        LanguageExample {
            name: "JavaScript",
            extension: "javascript",
            old_code: r#"function fetchData(url) {
    return fetch(url)
        .then(response => response.json())
        .then(data => {
            console.log(data);
            return data;
        });
}"#,
            new_code: r#"async function fetchData(url) {
    try {
        const response = await fetch(url);
        const data = await response.json();
        console.log('Fetched:', data);
        return data;
    } catch (error) {
        console.error('Error fetching data:', error);
        throw error;
    }
}"#,
        },
        LanguageExample {
            name: "Python",
            extension: "python",
            old_code: r#"def calculate_average(numbers):
    total = 0
    count = 0
    for num in numbers:
        total += num
        count += 1
    return total / count"#,
            new_code: r#"def calculate_average(numbers):
    """Calculate the average of a list of numbers."""
    if not numbers:
        raise ValueError("Cannot calculate average of empty list")
    return sum(numbers) / len(numbers)"#,
        },
        LanguageExample {
            name: "Go",
            extension: "go",
            old_code: r#"func processItems(items []int) int {
    var result int
    for i := 0; i < len(items); i++ {
        result += items[i]
    }
    return result
}"#,
            new_code: r#"func processItems(items []int) int {
    result := 0
    for _, item := range items {
        result += item
    }
    return result
}"#,
        },
        LanguageExample {
            name: "TypeScript",
            extension: "typescript",
            old_code: r#"class User {
    name: string;
    age: number;
    
    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }
}"#,
            new_code: r#"interface UserData {
    name: string;
    age: number;
    email?: string;
}

class User implements UserData {
    constructor(
        public name: string,
        public age: number,
        public email?: string
    ) {}
    
    getInfo(): string {
        return `${this.name} (${this.age})`;
    }
}"#,
        },
        LanguageExample {
            name: "Java",
            extension: "java",
            old_code: r#"public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
}"#,
            new_code: r#"public class Calculator {
    private static final Logger logger = LoggerFactory.getLogger(Calculator.class);
    
    public int add(int a, int b) {
        logger.debug("Adding {} and {}", a, b);
        int result = a + b;
        logger.debug("Result: {}", result);
        return result;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
}"#,
        },
        LanguageExample {
            name: "HTML",
            extension: "html",
            old_code: r#"<div class="container">
    <h1>Welcome</h1>
    <p>Hello, world!</p>
</div>"#,
            new_code: r#"<div class="container mx-auto px-4">
    <header>
        <h1 class="text-3xl font-bold">Welcome</h1>
    </header>
    <main>
        <p class="text-gray-700">Hello, world!</p>
        <button class="btn btn-primary">Click me</button>
    </main>
</div>"#,
        },
        LanguageExample {
            name: "CSS",
            extension: "css",
            old_code: r#".container {
    width: 100%;
    padding: 20px;
}

h1 {
    color: black;
}"#,
            new_code: r#".container {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
}

h1 {
    color: #333;
    font-size: 2rem;
    margin-bottom: 1rem;
}

@media (max-width: 768px) {
    .container {
        padding: 10px;
    }
}"#,
        },
        LanguageExample {
            name: "JSON",
            extension: "json",
            old_code: r#"{
    "name": "example",
    "version": "1.0.0"
}"#,
            new_code: r#"{
    "name": "example",
    "version": "2.0.0",
    "description": "An example package",
    "author": "Developer",
    "dependencies": {
        "lodash": "^4.17.21"
    }
}"#,
        },
        LanguageExample {
            name: "YAML",
            extension: "yaml",
            old_code: r#"name: Build
on: push
jobs:
  build:
    runs-on: ubuntu-latest"#,
            new_code: r#"name: Build and Test
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Build
        run: cargo build
      - name: Test
        run: cargo test"#,
        },
    ]
}

#[function_component(LanguageShowcase)]
pub fn language_showcase() -> Html {
    let selected_lang = use_state(|| 0usize);
    let examples = get_language_examples();
    
    let on_language_change = {
        let selected_lang = selected_lang.clone();
        Callback::from(move |idx: usize| {
            selected_lang.set(idx);
        })
    };
    
    let current_example = &examples[*selected_lang];
    
    html! {
        <div class="space-y-4">
            <div class="flex flex-wrap gap-2 mb-4">
                {
                    for examples.iter().enumerate().map(|(idx, example)| {
                        let onclick = {
                            let on_language_change = on_language_change.clone();
                            Callback::from(move |_| on_language_change.emit(idx))
                        };
                        
                        html! {
                            <button
                                class={classes!(
                                    "px-3", "py-1", "rounded", "text-sm", "font-medium",
                                    "transition-colors", "duration-200",
                                    if idx == *selected_lang {
                                        "bg-blue-500 text-white"
                                    } else {
                                        "bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
                                    }
                                )}
                                onclick={onclick}
                            >
                                { example.name }
                            </button>
                        }
                    })
                }
            </div>
            
            <div class="border border-gray-300 dark:border-gray-700 rounded-lg overflow-hidden">
                <div class="bg-gray-100 dark:bg-gray-800 px-3 py-1 border-b border-gray-300 dark:border-gray-700">
                    <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
                        { format!("{} Example", current_example.name) }
                    </span>
                </div>
                <DiffView
                    old_text={current_example.old_code}
                    new_text={current_example.new_code}
                    language={current_example.extension}
                    mode={wonopui::code_editor::DiffViewMode::SideBySide}
                    show_line_numbers={true}
                    theme="auto"
                    font_size={13}
                    line_height={1.5}
                />
            </div>
        </div>
    }
}