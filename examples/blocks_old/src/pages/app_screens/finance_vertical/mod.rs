use crate::pages::example_block::ExampleBlock;
use yew::prelude::*;

use account_summary::AccountSummary;
use budget_planner::BudgetPlanner;
use transaction_history::TransactionHistory;
use wonopui::*;

#[function_component(FinanceVertical)]
pub fn finance_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"Finance"} />
            </Breadcrumb>

            <ExampleBlock title={"Account Summary"}>
                <AccountSummary />
            </ExampleBlock>
            <ExampleBlock title={"Transaction History"}>
                <TransactionHistory />
            </ExampleBlock>
            <ExampleBlock title={"Budget Planner"}>
                <BudgetPlanner />
            </ExampleBlock>
        </MainContent>
    }
}

pub mod account_summary;
pub mod budget_planner;
pub mod transaction_history;
