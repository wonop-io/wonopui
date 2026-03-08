use super::layout1::AppLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(UserList)]
pub fn user_list() -> Html {
    let user_data = vec![
        ("John Doe", "john.doe@example.com", "Admin"),
        ("Jane Smith", "jane.smith@example.com", "User"),
        ("Alice Johnson", "alice.johnson@example.com", "Moderator"),
    ];

    html! {
        <AppLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
            <div class="container mx-auto p-4">
                <div class="flex justify-between items-center mb-4">
                    <h1 class="text-2xl font-bold">{"User Management"}</h1>
                    <Button variant={ButtonVariant::Primary}>{"Add New User"}</Button>
                </div>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell>{"Name"}</TableCell>
                            <TableCell>{"Email"}</TableCell>
                            <TableCell>{"Role"}</TableCell>
                            <TableCell class="text-right">{"Actions"}</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        { for user_data.iter().map(|(name, email, role)| html! {
                            <TableRow>
                                <TableCell>{*name}</TableCell>
                                <TableCell>{*email}</TableCell>
                                <TableCell>{*role}</TableCell>
                                <TableCell class="text-right">
                                    <Button variant={ButtonVariant::Ghost}>{"Edit"}</Button>
                                    <Button variant={ButtonVariant::Primary}>{"Delete"}</Button>
                                </TableCell>
                            </TableRow>
                        }) }
                    </TableBody>
                </Table>
            </div>
        </AppLayout>
    }
}
