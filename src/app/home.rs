use leptos::task::spawn_local;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Home() -> impl IntoView {
    let (input_path, set_input_path) = signal(String::new());
    let (output_path, set_output_path) = signal(String::new());
    let (mode, set_mode) = signal("extract".to_string());
    let (result, set_result) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let pick_input = move |_| {
        let set_input_path = set_input_path.clone();
        spawn_local(async move {
            let val = invoke("pick_file", JsValue::NULL).await;
            if let Some(path) = val.as_string() {
                set_input_path.set(path);
            }
        });
    };
    let pick_output = move |_| {
        let set_output_path = set_output_path.clone();
        spawn_local(async move {
            let val = invoke("pick_directory", JsValue::NULL).await;
            if let Some(path) = val.as_string() {
                set_output_path.set(path);
            }
        });
    };
    let do_action = move |_| {
        set_loading.set(true);
        set_result.set(String::new());
        let input = input_path.get_untracked();
        let output = output_path.get_untracked();
        let mode = mode.get_untracked();
        let set_result = set_result.clone();
        let set_loading = set_loading.clone();
        spawn_local(async move {
            if input.is_empty() || output.is_empty() {
                set_result.set("请选择输入和输出路径".to_string());
                set_loading.set(false);
                return;
            }
            let (cmd, args) = if mode == "extract" {
                ("extract_archive_cmd", serde_json::json!({"input": input, "output": output}))
            } else {
                ("compress_archive_cmd", serde_json::json!({"input": input, "output": output}))
            };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            let resp = invoke(cmd, args).await;
            if let Some(err) = resp.as_string() {
                if err.is_empty() {
                    set_result.set("操作成功".to_string());
                } else {
                    set_result.set(format!("操作失败: {}", err));
                }
            } else {
                set_result.set("操作完成".to_string());
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen p-4 bg-gray-50">
            <h1 class="text-3xl font-bold text-blue-600 mb-4">"RecFive 压缩/解压工具"</h1>
            <div class="w-full max-w-lg bg-white rounded-lg shadow p-6 flex flex-col gap-4">
                <div class="flex flex-col gap-2">
                    <label class="font-medium">"输入文件/目录"</label>
                    <div class="flex gap-2">
                        <input
                            class="flex-1 px-2 py-1 border rounded"
                            readonly
                            prop:value=input_path
                        />
                        <button
                            class="px-3 py-1 bg-blue-500 text-white rounded"
                            on:click=pick_input
                        >
                            "选择..."
                        </button>
                    </div>
                </div>
                <div class="flex flex-col gap-2">
                    <label class="font-medium">"输出目录"</label>
                    <div class="flex gap-2">
                        <input
                            class="flex-1 px-2 py-1 border rounded"
                            readonly
                            prop:value=output_path
                        />
                        <button
                            class="px-3 py-1 bg-blue-500 text-white rounded"
                            on:click=pick_output
                        >
                            "选择..."
                        </button>
                    </div>
                </div>
                <div class="flex gap-4 items-center">
                    <label class="font-medium">"操作类型"</label>
                    <select
                        class="border rounded px-2 py-1"
                        prop:value=mode
                        on:change=move |ev| set_mode.set(event_target_value(&ev))
                    >
                        <option value="extract">"解压"</option>
                        <option value="compress">"压缩"</option>
                    </select>
                </div>
                <button
                    class="w-full py-2 bg-green-600 text-white font-bold rounded disabled:opacity-50"
                    on:click=do_action
                    disabled=loading
                >
                    {move || {
                        if loading.get() {
                            "处理中...".to_string()
                        } else {
                            "开始".to_string()
                        }
                    }}
                </button>
                <div class="min-h-[2em] text-center text-gray-700">{move || result.get()}</div>
            </div>
        </div>
    }
}
