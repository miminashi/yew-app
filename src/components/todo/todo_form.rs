use yew::{Callback, InputEvent, function_component, html, Html, use_state, MouseEvent, Properties, TargetCast};
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
    pub on_add: Callback<String>
}

#[function_component(TodoForm)]
pub fn todo_item(props: &TodoFormProps) -> Html {
    let title = use_state(|| "".to_string());

    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            // チュートリアルのコードそのままだと日本語入力に難があったので、以下URLを参考に修正した
            //   https://tech.isid.co.jp/entry/2022/10/11/%E3%80%90Yew%E3%80%91Rust%E3%81%A7%E3%83%95%E3%83%AD%E3%83%B3%E3%83%88%E3%82%A8%E3%83%B3%E3%83%89%E9%96%8B%E7%99%BA_-_Rust%E3%81%AE%E3%83%9E%E3%82%AF%E3%83%AD%E3%82%92%E7%B4%90%E8%A7%A3%E3%81%8F_-
            //   https://github.com/ISID/wasm-md-editor/blob/main/src/components/editor.rs#L41
            let input: HtmlInputElement = e.target_unchecked_into();
            log::info!("title: {:?}", input.value());
            title.set(input.value());
        })
    };

    let onclick = {
        let on_add = props.on_add.clone();
        let title = title.clone();
        Callback::from(move |e: MouseEvent| {
            //log::info!("title: {:?}", *title);
            e.prevent_default();
            title.set("".to_string());
            on_add.emit((*title).clone());
        })
    };

    html! {
        <form class="mb-5">
          <div class="mb-3">
            <label for="title" class="form-label">{"タイトル"}</label>
            <input type="text" value={(*title).clone()} oninput={oninput} class="form-control" id="title" />
          </div>
          //<div class="mb-3">
          //  {&*title}
          //</div>
          <button type="submit" onclick={onclick} class="btn btn-primary">{"追加"}</button>
        </form>
    }
}
