use super::layout::AIFitnessLayout;
use wonopui::*;
use yew::prelude::*;

#[function_component(WorkoutTimer)]
pub fn workout_timer() -> Html {
    html! {
        <AIFitnessLayout initial_state={LayoutState { sidebar_folded: false, ..LayoutState::new() }}>
                <div class="flex flex-col space-y-4">
                    <div class="text-4xl font-extrabold drop-shadow-lg">
                        {"Time Left: 00:30"}
                    </div>
                    <div class="w-full h-64 bg-gray-800 rounded-lg overflow-hidden shadow-lg flex items-center justify-center">
                        <video controls=true class="w-full h-full rounded-lg">
                            <source src="current_exercise.mp4" type="video/mp4" />
                            {"Your browser does not support the video tag."}
                        </video>
                    </div>
                    <div class="w-full space-y-6">
                        <div class="w-full bg-gray-700 rounded-full h-2.5 mb-4">
                            <div class="bg-green-500 h-2.5 rounded-full" style="width: 20%"></div>
                        </div>
                        <div class="text-xl font-bold">
                            {"Current Exercise: Jumping Jacks"}
                        </div>
                        <ul class="list-disc list-inside text-lg space-y-2">
                            <li>{"Exercise 1: Jumping Jacks - 1 min"}</li>
                            <li>{"Exercise 2: Push-ups - 2 mins"}</li>
                            <li>{"Exercise 3: Squats - 1.5 mins"}</li>
                            <li>{"Exercise 4: Lunges - 2 mins"}</li>
                            <li>{"Exercise 5: Plank - 1 min"}</li>
                        </ul>
                    </div>
                </div>
        </AIFitnessLayout>
    }
}
