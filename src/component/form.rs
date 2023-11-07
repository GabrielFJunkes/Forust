use maud::{Markup, html, PreEscaped};

#[derive(Clone)]
pub enum FormElem {
    TextArea,
    Input
}

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub form_elem: FormElem,
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

fn create_input_elem(input_data: Input) -> Markup {
    html!(
        @match input_data.form_elem {
            FormElem::TextArea => (PreEscaped("<textarea ")),
            FormElem::Input => {
                (PreEscaped("<input "))
                "type='"(input_data.input_type)"'"
            },
        };
        ("class='shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline'")
        "id='"(input_data.id)"'"
        "name='"(input_data.id)"'"
        "placeholder='"(input_data.placeholder)"'"
        (PreEscaped(">"))
        @match input_data.form_elem {
            FormElem::TextArea => (PreEscaped("</textarea>")),
            FormElem::Input => (PreEscaped("</input>")),
        };
    )
}

pub fn create_form(form_data: Form) -> Markup {
    html!(
        form 
        action=(form_data.action)
        method=(form_data.method)
        class="bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            @for input_data in form_data.inputs {
                div class="mb-6" {
                    label class="block text-gray-700 text-sm font-bold mb-2" for=(input_data.id) {
                        (input_data.name)
                    }
                    (create_input_elem(input_data))
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