use crate::pages::example_block::ExampleBlock;
use wonopui::*;
use yew::prelude::*;
mod invoice_details;
mod layout1;
mod user_list;
mod user_settings;
mod wallet;

use detail_screen4::DetailScreen4;
use invoice_details::InvoiceDetail;
use user_list::UserList;
use user_settings::UserSettings;
use wallet::WalletDetail;

#[function_component(DetailScreens)]
pub fn detail_screens() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Blocks"} href="/blocks" />
                <BreadcrumbItem label={"Detail Screens"} />
            </Breadcrumb>
            <ExampleBlock title={"Detail Screen 1"}>
                <UserList />
            </ExampleBlock>
            <ExampleBlock title={"Detail Screen 2"}>
                <InvoiceDetail />
            </ExampleBlock>
            <ExampleBlock title={"Detail Screen 3"}>
                <WalletDetail />
            </ExampleBlock>
            <ExampleBlock title={"Detail Screen 4"}>
                <UserSettings />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod detail_screen2 {
    use super::*;

    #[function_component(DetailScreen2)]
    pub fn detail_screen2() -> Html {
        let options = vec![
            ("1".to_string(), "Option 1".to_string()),
            ("2".to_string(), "Option 2".to_string()),
            ("3".to_string(), "Option 3".to_string()),
        ];
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Detail Screen 2"}</h2>
                <p>{"This is the second detail screen example."}</p>
                <Combobox id="combobox2" options={options} on_select={Callback::from(|_| {})} disabled={false} />
            </div>
        }
    }
}

pub mod detail_screen3 {
    use super::*;

    #[function_component(DetailScreen3)]
    pub fn detail_screen3() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Detail Screen 3"}</h2>
                <p>{"This is the third detail screen example."}</p>
                <Checkbox id="option1" checked={false} on_toggle={Callback::from(|_| {})} />
                <label for="option1" class="checkbox-label">{"Option 1"}</label>
                <Checkbox id="option2" checked={false} on_toggle={Callback::from(|_| {})} />
                <label for="option2" class="checkbox-label">{"Option 2"}</label>
                <Checkbox id="option3" checked={false} on_toggle={Callback::from(|_| {})} />
                <label for="option3" class="checkbox-label">{"Option 3"}</label>
            </div>
        }
    }
}

pub mod detail_screen4 {
    use super::*;

    #[function_component(DetailScreen4)]
    pub fn detail_screen4() -> Html {
        html! {
            <div class="p-4 border rounded shadow">
                <h2 class="text-xl font-bold">{"Detail Screen 4"}</h2>
                <p>{"This is the fourth detail screen example."}</p>
                <Command options={vec![
                    ("1".to_string(), "Command 1".to_string(), None),
                    ("2".to_string(), "Command 2".to_string(), None),
                    ("3".to_string(), "Command 3".to_string(), None)
                ]} />
            </div>
        }
    }
}
