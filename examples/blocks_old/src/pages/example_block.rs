use wonopui::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ExampleBlockProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub isolate: bool,
}

#[function_component(ExampleBlock)]
pub fn example_block(props: &ExampleBlockProps) -> Html {
    let height = 600;
    let dark_mode = use_state(|| false);
    let coordinates = use_state(|| (0., 0., 900., height as f64));
    let html = if props.isolate {
        html! {
            <div class="w-full h-full bg-white flex items-center justify-center p-16">
                {props.children.clone()}
            </div>
        }
    } else {
        html! {
            {props.children.clone()}
        }
    };

    let on_toggle = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            dark_mode.set(!*dark_mode);
        })
    };

    let on_coordinates_change = {
        let coordinates = coordinates.clone();
        Callback::from(move |new_coordinates: (f64, f64, f64, f64)| {
            coordinates.set(new_coordinates);
        })
    };

    let set_coordinates = {
        let coordinates = coordinates.clone();
        Callback::from(move |value: String| {
            let new_coordinates = match value.as_str() {
                "desktop" => (0., 0., 1200., height as f64),
                "tablet" => (0., 0., 800., height as f64),
                "mobile" => (0., 0., 400., height as f64),
                _ => (0., 0., 900., height as f64),
            };
            coordinates.set(new_coordinates);
        })
    };

    html! {
        <div class="my-4">
            <div class="flex justify-between py-4">
                <h1 class="text-2xl">{props.title.clone()}</h1>
                <div class="flex space-x-4 items-center">
                    <GroupButton default_value="view" class="w-[400px] inline-block">
                        <GroupButtonTrigger value="view">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                            </svg>
                        </GroupButtonTrigger>
                        <GroupButtonTrigger value="code">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75 22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3-4.5 16.5" />
                            </svg>
                        </GroupButtonTrigger>
                    </GroupButton>
                    <GroupButton default_value="none" class="w-[400px] inline-block">
                        <GroupButtonTrigger value="desktop" onclick={set_coordinates.reform(|_| "desktop".to_string())}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25" />
                            </svg>
                        </GroupButtonTrigger>
                        <GroupButtonTrigger value="tablet" onclick={set_coordinates.reform(|_| "tablet".to_string())}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5h3m-6.75 2.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-15a2.25 2.25 0 0 0-2.25-2.25H6.75A2.25 2.25 0 0 0 4.5 4.5v15a2.25 2.25 0 0 0 2.25 2.25Z" />
                            </svg>
                        </GroupButtonTrigger>
                        <GroupButtonTrigger value="mobile" onclick={set_coordinates.reform(|_| "mobile".to_string())}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-4 w-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 1.5H8.25A2.25 2.25 0 0 0 6 3.75v16.5a2.25 2.25 0 0 0 2.25 2.25h7.5A2.25 2.25 0 0 0 18 20.25V3.75a2.25 2.25 0 0 0-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3" />
                            </svg>
                        </GroupButtonTrigger>
                    </GroupButton>
                    <SwitchButton checked={*dark_mode} on_toggle={on_toggle} />
                </div>
            </div>
            <div class="flex items-stretch dark:bg-gray-600 dark:text-white w-full" style={format!("min-height: {}px; height: {}px;",height,height)}>
                <Resizable east={true} south_east={false} south={false} coordinates={*coordinates} on_coordinates_change={on_coordinates_change}>
                <Iframe body_class={if *dark_mode { "dark" } else { "" }} class="w-full h-full">
                    {html}
                </Iframe>
                </Resizable>
            </div>
        </div>
    }
}
