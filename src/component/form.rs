use maud::{Markup, html};

pub struct Input {
    pub name: String,
    pub id: String,
    pub input_type: String,
    pub placeholder: String
}

pub struct Form {
    pub inputs: Vec<Input>,
    pub button_title: String,
    pub button_type: String,
    pub action: String,
    pub method: String,
    pub rest: Option<Markup>
}

pub fn create_form(form_data: Form) -> Markup {
    html!(
        form 
        action=(form_data.action)
        method=(form_data.method)
        class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 max-w-md" {
            @for input_data in form_data.inputs {
                div class="mb-6" {
                    label class="block text-gray-700 text-sm font-bold mb-2" for="username" {
                        (input_data.name)
                    }
                    input 
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 
                    leading-tight focus:outline-none focus:shadow-outline"
                    id=(input_data.id)
                    name=(input_data.id) 
                    type=(input_data.input_type)
                    placeholder=(input_data.placeholder) {}
                }
            }
            div class="flex items-center justify-between" {
                button 
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded 
                focus:outline-none focus:shadow-outline mx-auto w-full" 
                type=(form_data.button_type) {
                    (form_data.button_title)
                }
                @if let Some(rest) = form_data.rest{
                    (rest)
                }
            }
        }
    )
}