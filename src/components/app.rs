use gloo_console::log;
use gloo_timers::callback::Interval;
use yew::prelude::*;

const TEXT: &str = "恭喜你!成功使用Yew+tailwindcss";

#[function_component(App)]
pub fn app() -> Html {
    // State to keep track of the index of the enlarged character
    let enlarged_index = use_state(|| 0);
    let color_index = use_state(|| 0);
    let text_length = TEXT.chars().count(); // Get the character count of the text

    // Define a list of color classes to cycle through
    let color_classes = vec![
        "text-red-500",
        "text-blue-500",
        "text-green-500",
        "text-yellow-500",
        "text-purple-500",
        "text-pink-500",
        "text-indigo-500",
    ];
    let color_classes_len = color_classes.len();
    let enlarged_index_clone = enlarged_index.clone();
    let color_index_clone = color_index.clone();
    use_effect(
        move || {
            let enlarged_index = enlarged_index_clone.clone();
            let color_index = color_index_clone.clone();
            // Set up a timer that fires every half second
            let interval = Interval::new(166, move || {
                enlarged_index.set((*enlarged_index + 1) % text_length);
                color_index.set((*color_index + 1) % color_classes_len);
            });

            // Return a closure to clean up the interval when the component is destroyed
            move || {
                log!("Cleaning up interval");
                drop(interval)
            }
        },
    );

    // Convert the string message to an iterable of HTML elements, enlarging one character at a time
    let message: Html = TEXT
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let size_class = if index == *enlarged_index {
                "text-4xl md:text-6xl" // Responsive class for enlarged text with Tailwind CSS
            } else {
                "text-base md:text-xl" // Responsive class for base text size with Tailwind CSS
            };
            // Use modulo to cycle through color classes
            let color_class = color_classes[*color_index % color_classes_len];

            html! { <span class={classes!(size_class, color_class)}>{char}</span> }
        })
        .collect();

    html! {
        // `container` for maximum width, `mx-auto` for horizontal centering, `text-center` for text alignment
        // `min-h-screen` for minimum height of screen to center vertically using flex
        // `flex` to use Flexbox, `justify-center` and `items-center` to center content
        <div class={"container mx-auto text-center min-h-screen flex justify-center items-center px-4"}>
            { message }
        </div>
    }
}