mod activity_log;
mod community;
mod daily_program;
mod exercise_library;
mod goal_setting;
mod layout;
mod nutrition_plan;
mod progress_tracking;
mod workout_timer;

use crate::pages::example_block::ExampleBlock;
use yew::prelude::*;

use activity_log::ActivityLog;
use community::Community;
use daily_program::DailyProgram;
use exercise_library::ExerciseLibrary;
use goal_setting::GoalSetting;
use nutrition_plan::NutritionPlan;
use progress_tracking::ProgressTracking;
use wonopui::*;
use workout_timer::WorkoutTimer;

#[function_component(AiBasedFitnessVertical)]
pub fn ai_based_fitness_vertical() -> Html {
    html! {
        <MainContent>
            <Breadcrumb>
                <BreadcrumbItem label={"Screens"} href="/screens" />
                <BreadcrumbItem label={"AI Based Fitness"} />
            </Breadcrumb>

            <ExampleBlock title={"Daily Program"}>
                <DailyProgram />
            </ExampleBlock>
            <ExampleBlock title={"Workout Timer"}>
                <WorkoutTimer />
            </ExampleBlock>
            <ExampleBlock title={"Progress Tracking"}>
                <ProgressTracking />
            </ExampleBlock>
            <ExampleBlock title={"Exercise Library"}>
                <ExerciseLibrary />
            </ExampleBlock>
            <ExampleBlock title={"Nutrition Plan"}>
                <NutritionPlan />
            </ExampleBlock>
            <ExampleBlock title={"Goal Setting"}>
                <GoalSetting />
            </ExampleBlock>
            <ExampleBlock title={"Activity Log"}>
                <ActivityLog />
            </ExampleBlock>
            <ExampleBlock title={"Community"}>
                <Community />
            </ExampleBlock>
        </MainContent>
    }
}
