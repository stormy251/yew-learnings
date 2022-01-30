use yew::{function_component, html, Properties};

// This is an example of a functional component WITHOUT props
#[function_component(ChatContainer)]
pub fn chat_container() -> Html {
    html! {
        <div class="hello-world">{"Hello World welcome V2"}</div>
    }
}

// This is an example of a functional component with props
#[derive(Properties, PartialEq, Clone)]
pub struct MyTestProps {
    pub name: String
}

#[function_component(MyTest)]
pub fn my_test(props: &MyTestProps) -> Html {

    // This html! macros value is not terminated with a `;` therefore it is the return value of the publict my_test function
    html! {
        <div>{"Hello there: "} {&props.name}</div>
    }
}
