use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;

use dropdown1::Dropdown1;
use dropdown2::Dropdown2;
use dropdown3::Dropdown3;
use dropdown4::Dropdown4;

#[function_component(Dropdowns)]
pub fn dropdowns() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"DropDowns"} />
            </Breadcrumb>
            <ExampleBlock title={"Dropdown 1"} isolate={true}>
                <Dropdown1 />
            </ExampleBlock>
            <ExampleBlock title={"Dropdown 2"} isolate={true}>
                <Dropdown2 />
            </ExampleBlock>
            <ExampleBlock title={"Dropdown 3"} isolate={true}>
                <Dropdown3 />
            </ExampleBlock>
            <ExampleBlock title={"Dropdown 4"} isolate={true}>
                <Dropdown4 />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod dropdown1 {
    use super::*;

    #[function_component(Dropdown1)]
    pub fn dropdown1() -> Html {
        let items = vec![
            DropdownItemProps {
                label: "Option 1".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Option 2".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Option 3".to_string(),
                ..Default::default()
            },
        ];

        html! {
            <Dropdown items={items}>
                <Button>{"Dropdown 1"}</Button>
            </Dropdown>
        }
    }
}

pub mod dropdown2 {
    use super::*;

    #[function_component(Dropdown2)]
    pub fn dropdown2() -> Html {
        let items = vec![
            DropdownItemProps {
                label: "Item A".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Item B".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Item C".to_string(),
                ..Default::default()
            },
        ];

        html! {
            <table>
                <tr>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::NorthStart}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User NorthStart" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::NorthMiddle}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User NorthMiddle" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::NorthEnd}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User NorthEnd" />
                        </Dropdown>
                    </td>
                </tr>
                <tr>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::SouthStart}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User SouthStart" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::SouthMiddle}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User SouthMiddle" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::SouthEnd}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User SouthEnd" />
                        </Dropdown>
                    </td>
                </tr>
                <tr>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::EastStart}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User EastStart" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::EastMiddle}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User EastMiddle" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::EastEnd}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User EastEnd" />
                        </Dropdown>
                    </td>
                </tr>
                <tr>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::WestStart}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User WestStart" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::WestMiddle}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User WestMiddle" />
                        </Dropdown>
                    </td>
                    <td>
                        <Dropdown items={items.clone()} position={PopoverPosition::WestEnd}>
                            <Avatar src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User WestEnd" />
                        </Dropdown>
                    </td>
                </tr>
            </table>
        }
    }
}

pub mod dropdown3 {
    use super::*;

    #[function_component(Dropdown3)]
    pub fn dropdown3() -> Html {
        let items = vec![
            DropdownItemProps {
                label: "First".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Second".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Third".to_string(),
                ..Default::default()
            },
        ];

        html! {
            <Dropdown items={items} position={PopoverPosition::EastEnd}>
                <div class="flex items-center space-x-4 items-center justify-between p-2 border rounded border-zinc-100 hover:bg-zinc-100">
                    <Avatar  src="https://avatars.githubusercontent.com/u/10000?v=4" alt="User" />
                    <div class="flex flex-col space-y-1">
                        <span>{"John Doe"}</span>
                        <span class="text-xs text-zinc-500">{"john.doe@example.com"}</span>
                    </div>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6">
                      <path stroke-linecap="round" stroke-linejoin="round" d="m8.25 4.5 7.5 7.5-7.5 7.5" />
                    </svg>

                </div>
            </Dropdown>
        }
    }
}

pub mod dropdown4 {
    use super::*;

    #[function_component(Dropdown4)]
    pub fn dropdown4() -> Html {
        let items = vec![
            DropdownItemProps {
                label: "Alpha".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Beta".to_string(),
                ..Default::default()
            },
            DropdownItemProps {
                label: "Gamma".to_string(),
                ..Default::default()
            },
        ];

        html! {
            <Dropdown items={items}>
                <Button>{"Dropdown 4"}</Button>
            </Dropdown>
        }
    }
}
