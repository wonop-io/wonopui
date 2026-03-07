use wonopui::*;
use yew::prelude::*;

use super::layout1::AppLayout;

#[function_component(ProductList)]
pub fn product_list() -> Html {
    let group_actions = vec![
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

    let item_actions = vec![
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
        <DialogProvider>
            <Dialog id="create_new">
                <DialogHeader>
                    <DialogTitle>{"New product"}</DialogTitle>
                </DialogHeader>
                <DialogBody>
                    {"This is the description for Dialog 1."}
                </DialogBody>
                <DialogFooter>
                    <DialogClose>
                        <Button>{"Close"}</Button>
                    </DialogClose>
                </DialogFooter>
            </Dialog>
            <AppLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
                <div class="flex justify-between items-start">
                    <H1 class="mb-8">{"Products"}</H1>
                    <div class="flex space-x-2">
                        <Dropdown items={group_actions.clone()} position={PopoverPosition::SouthEnd}>
                            <Button variant={ButtonVariant::Ghost}>{"."}</Button>
                        </Dropdown>
                        <DialogTrigger id="create_new">
                            <Button variant={ButtonVariant::Primary}>{"New"}</Button>
                        </DialogTrigger>
                    </div>
                </div>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell>{"Name"}</TableCell>
                            <TableCell>{"Price (incl. VAT)"}</TableCell>
                            <TableCell>{"Stock status"}</TableCell>
                            <TableCell class="text-left flex justify-end">{"Action"}</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        <TableRow>
                            <TableCell>{"Protein bar"}</TableCell>
                            <TableCell>{"109 NOK"}</TableCell>
                            <TableCell><Badge badge_type={BadgeType::Warning} label={"Few left"} /></TableCell>
                            <TableCell class="text-left flex justify-end">
                                <Dropdown items={item_actions.clone()} position={PopoverPosition::SouthEnd}>
                                    <Button variant={ButtonVariant::Ghost}>{"."}</Button>
                                </Dropdown>
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>{"Protein powder"}</TableCell>
                            <TableCell>{"189 NOK"}</TableCell>
                            <TableCell><Badge badge_type={BadgeType::Success} label={"Available"} /></TableCell>
                            <TableCell class="text-left flex justify-end">
                                <Dropdown items={item_actions.clone()} position={PopoverPosition::SouthEnd}>
                                    <Button variant={ButtonVariant::Ghost}>{"."}</Button>
                                </Dropdown>
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>{"White the"}</TableCell>
                            <TableCell>{"229 NOK"}</TableCell>
                            <TableCell><Badge badge_type={BadgeType::Error} label={"Out of stock"} /></TableCell>
                            <TableCell class="text-left flex justify-end">
                                <Dropdown items={item_actions.clone()} position={PopoverPosition::SouthEnd}>
                                    <Button variant={ButtonVariant::Ghost}>{"."}</Button>
                                </Dropdown>
                            </TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </AppLayout>
        </DialogProvider>
    }
}
