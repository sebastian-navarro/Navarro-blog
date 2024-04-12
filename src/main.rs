use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1 class="text-lg font-sans"> {"100 % RUST"} </h1>
            <button 
                {onclick} 
                class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">{ "+1" }
            </button>
            <p>{ *counter }</p>
        </div>
    }
}

use std::fs;
use std::path::Path;

pub fn read_mdx_files(directory: &str) -> Result<Vec<String>, std::io::Error> {
    let mut posts = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "mdx" {
            let content = fs::read_to_string(&path)?;
            posts.push(content);
        }
    }
    Ok(posts)
}


fn main() {
    //yew::Renderer::<App>::new().render();

    println!("{:?}", read_mdx_files("content/rust/"))
}