# Dioxus 0.7.3 - Guia Completo da API

## Estrutura do Projeto
- Workspace Rust com ~55 pacotes em `packages/`
- Crate principal: `dioxus` (reexporta tudo)
- Plataformas: desktop (wry/tao), web (wasm), native (blitz/winit), mobile, liveview, fullstack (SSR+hydration), server
- CLI: `dx serve`, `dx serve --platform web --features web`
- Rust edition 2024, MSRV 1.85.0

## RSX Macro
```rust
use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        div { class: "container", id: "main",
            h1 { "Hello {variable}" }
            p { color: "blue", font_weight: "bold", "styled text" }
            // Conditional attributes
            class: if condition { "active" },
            // Conditional rendering
            if show { div { "visible" } }
            // Iteration
            for item in items { li { "{item}" } }
            // Match
            match value { true => rsx!(h1 {"yes"}), false => rsx!(h1 {"no"}) }
            // Fragments
            Fragment { div {"A"} div {"B"} }
            // Dangerous inner HTML
            div { dangerous_inner_html: "<p>raw html</p>" }
        }
    }
}
```

## Entry Points
```rust
// Simples
fn main() { dioxus::launch(app); }

// Com config de plataforma
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(desktop!({
            use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
            Config::new().with_window(
                WindowBuilder::default()
                    .with_title("App")
                    .with_inner_size(LogicalSize::new(800.0, 600.0)),
            )
        }))
        .launch(app);
}

// Específico de plataforma
dioxus::LaunchBuilder::desktop().launch(app);
```

## Signals / State Management

### use_signal - Estado local reativo (Copy)
```rust
let mut count = use_signal(|| 0);
count += 1;                    // AddAssign
count.set(10);                 // Set direto
count.toggle();                // Para bools
let val = count();             // Leitura
count.write()[i] = value;      // Acesso mutável (Vec, HashMap)
count.push(item);              // Métodos de Vec
count.read();                  // Referência de leitura
count.peek();                  // Leitura sem subscribe
```

### use_memo - Valor derivado memoizado
```rust
let doubled = use_memo(move || count() * 2);
// Recomputa automaticamente quando signals capturados mudam
```

### use_effect - Side effects reativos
```rust
use_effect(move || println!("Count changed to {count}"));
// Roda após mount e sempre que signals mudam
```

### GlobalSignal / GlobalMemo
```rust
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);
static DOUBLED: GlobalMemo<i32> = Memo::global(|| COUNT() * 2);
// Acessível de qualquer componente via COUNT.read(), *COUNT.write() += 1
```

### Context API
```rust
// Provider (componente pai)
use_context_provider(|| Signal::new(Theme::Light));

// Consumer (componente filho)
let theme: Signal<Theme> = use_context();
// ou try_use_context para Option
```

### Stores - Estado aninhado com fine-grained reactivity
```rust
#[derive(Store, PartialEq, Clone)]
struct AppState {
    todos: HashMap<u32, TodoItem>,
    filter: FilterState,
}

#[store]
impl<Lens> Store<AppState, Lens> {
    fn active_count(&self) -> usize { ... }
    fn toggle_all(&mut self) { ... }
}

let state = use_store(|| AppState { ... });
state.todos().get(id);      // Subscribe granular
state.filter().set(value);  // Write granular
```

## Componentes

### Definição com #[component]
```rust
#[component]
fn MyComponent(
    name: String,                          // Required
    #[props(default)] size: i32,           // Com default
    color: Option<String>,                 // Opcional (auto)
    #[props(!optional)] data: Option<T>,   // Forçar Some()
    #[props(optional)] extra: Option<T>,   // Explicitamente opcional
    count: ReadSignal<i32>,                // Signal readonly
    mut items: WriteSignal<Vec<T>>,        // Signal mutável
    onclick: EventHandler<MouseEvent>,     // Handler de evento
    children: Element,                     // Filhos
) -> Element {
    rsx! { div { {children} } }
}
```

### Props manuais com spread
```rust
#[derive(Props, PartialEq, Clone)]
struct Props {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    extra: String,
}

fn Component(props: Props) -> Element {
    rsx! { div { ..props.attributes, "{props.extra}" } }
}
```

### Genéricos
```rust
#[component]
fn Label<T: Clone + PartialEq + Display + 'static>(text: T) -> Element {
    rsx! { p { "{text}" } }
}
```

## Async

### use_future - Task de background
```rust
use_future(move || async move {
    loop {
        sleep(Duration::from_secs(1)).await;
        count += 1;
    }
});
// Retorna UseFuture handle (pause, resume, restart, cancel)
```

### use_resource - Future que retorna valor
```rust
let data = use_resource(move || async move {
    sleep(Duration::from_secs(1)).await;
    count() * 2
});
// data() retorna Option<T>
```

### use_loader - Fetch com Suspense/SSR
```rust
let data = use_loader(move || async move {
    reqwest::get("url").await?.json::<T>().await
})?;  // ? propaga para SuspenseBoundary/ErrorBoundary
// data.read(), data.restart(), data.loading()
```

### use_action - Ação disparada manualmente
```rust
let mut action = use_action(move |input: String| async move {
    reqwest::post("url").body(input).send().await
});
action.call("hello".into());
action.value();    // Option<Result<T>>
action.pending();  // bool
action.reset();
action.cancel();
```

### Event handlers async
```rust
button {
    onclick: move |_| async move {
        let result = fetch_data().await;
        state.set(result);
    }
}
```

## Router

### Definição de rotas
```rust
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: String },

        #[nest("/admin")]
        #[layout(AdminLayout)]
            #[route("/")]
            Dashboard {},
            #[route("/users")]
            Users {},
        #[end_layout]
        #[end_nest]

    #[end_layout]

    #[redirect("/old", || Route::Home {})]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
```

### Query params
```rust
#[route("/search?:query&:count")]
Search { query: String, count: usize },

// Componente recebe ReadSignal para reatividade
#[component]
fn Search(query: ReadSignal<String>, count: ReadSignal<usize>) -> Element { ... }
```

### Navegação
```rust
Link { to: Route::Blog { id: "post-1".into() }, "Read post" }
router().push(Route::Home {});
navigator().replace(Route::Search { query: "x".into(), count: 1 });
```

### Layout
```rust
#[component]
fn NavBar() -> Element {
    rsx! {
        nav { Link { to: Route::Home {}, "Home" } }
        Outlet::<Route> {}  // Renderiza a rota filha aqui
    }
}
```

## Fullstack / Server Functions

### Endpoints tipados
```rust
#[get("/api/data")]
async fn get_data() -> Result<MyData> { Ok(MyData { ... }) }

#[post("/api/echo")]
async fn echo(body: String) -> Result<String> { Ok(body) }

#[post("/api/{user_id}/chat?room_id")]
async fn chat(user_id: u32, room_id: Option<u32>) -> Result<String> { ... }

#[delete("/api/items/{id}")]
async fn delete_item(id: usize) -> Result<()> { ... }

// Server-only extractors (não enviados pelo client)
#[get("/api/secure", header: TypedHeader<Cookie>)]
async fn secure() -> Result<String> { ... }

// Anônimo
#[server]
async fn anon() -> Result<String> { ... }
```

### Streaming
```rust
#[get("/api/stream")]
async fn text_stream() -> Result<TextStream> {
    let (tx, rx) = futures::channel::mpsc::unbounded();
    tokio::spawn(async move { loop { tx.unbounded_send("msg".into()); } });
    Ok(Streaming::new(rx))
}
// Client: let mut stream = text_stream().await?; while let Some(Ok(text)) = stream.next().await { ... }
```

### WebSocket
```rust
#[get("/api/ws?name")]
async fn ws(name: String, options: WebSocketOptions) -> Result<Websocket<ClientMsg, ServerMsg>> {
    Ok(options.on_upgrade(move |mut socket| async move {
        while let Ok(msg) = socket.recv().await {
            _ = socket.send(ServerMsg::Reply(msg)).await;
        }
    }))
}
// Client: let mut socket = use_websocket(move || ws(name(), WebSocketOptions::new()));
```

### Forms
```rust
#[post("/api/login")]
async fn login(form: Form<LoginData>) -> Result<SetHeader<SetCookie>> {
    Ok(SetHeader::new(format!("token={}", generate_token()))?)
}
```

## Desktop APIs

### Window
```rust
use dioxus::desktop::window;
window().drag();              // Arrastar janela
window().set_minimized(true);
window().set_fullscreen(true);
window().set_always_on_top(true);
window().set_decorations(false);
window().set_title("New Title");
window().close();
window().new_window(VirtualDom::new(popup), Default::default());
```

### Asset Handler (interceptar requests do webview)
```rust
use_asset_handler("videos", |request, responder| {
    tokio::task::spawn(async move {
        let file = tokio::fs::File::open("video.mp4").await.unwrap();
        responder.respond(build_response(file));
    });
});
rsx! { video { src: "/videos/my_video.mp4", controls: true } }
```

### Global Shortcuts
```rust
use_global_shortcut("ctrl+s", move |state| {
    if state == HotKeyState::Pressed { toggled.toggle(); }
});
```

### Custom Menu
```rust
let menu = Menu::new();
let edit = Submenu::new("Edit", true);
edit.append_items(&[&MenuItem::with_id("action", "Do Thing", true, None)]);
menu.append(&edit);
let config = Config::new().with_menu(menu);

use_muda_event_handler(move |event| {
    if event.id() == "action" { /* handle */ }
});
```

## Assets & Styling

### asset! macro
```rust
const STYLE: Asset = asset!("/path/to/style.css");
static IMG: Asset = asset!("/path/to/image.png");
rsx! {
    Stylesheet { href: STYLE }
    img { src: IMG }
}
```

### CSS Modules
```rust
#[css_module("/path/to/module.css")]
struct Styles;
rsx! { div { class: Styles::container } }
```

### Inline styles
```rust
div { color: "blue", font_weight: "bold", margin_left: "10px" }
```

### External CSS
```rust
Stylesheet { href: "https://cdn.example.com/style.css" }
document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
```

## Error Handling
```rust
ErrorBoundary {
    handle_error: |ctx: ErrorContext| rsx! {
        h1 { "Error!" }
        for err in ctx.error() {
            if let Some(e) = err.downcast_ref::<ParseIntError>() {
                p { "Parse error: {e}" }
            }
        }
    },
    ChildComponent {}
}

SuspenseBoundary {
    fallback: |_| rsx! { "Loading..." },
    AsyncComponent {}
}
```

## DOM APIs

### onmounted + MountedData
```rust
let mut element = use_signal(|| None::<Rc<MountedData>>);
input {
    onmounted: move |cx| element.set(Some(cx.data())),
}
// Depois: element.read().as_ref().unwrap().set_focus(true).await;
// element.get_client_rect().await
```

### Eval (JS interop)
```rust
let mut eval = document::eval(r#"
    dioxus.send("from JS");
    let msg = await dioxus.recv();
    return "result";
"#);
eval.send("from Rust").unwrap();
let result: String = eval.recv().await.unwrap();
```

### Events
- Mouse: onclick, ondoubleclick, onmousedown, onmouseup, onmousemove
- Keyboard: onkeydown, onkeyup, onkeypress
- Form: oninput, onchange, onsubmit
- Focus: onfocusin, onfocusout
- Drag: ondragstart, ondragover, ondragleave, ondrop
- Scroll: onscroll
- Visibility: onvisible (IntersectionObserver)
- Resize: onresize
- File: evt.files() em onchange/ondrop
- Stop propagation: evt.stop_propagation()
- Prevent default: evt.prevent_default()
- Data transfer: evt.data_transfer().set_data/get_data

### File Upload
```rust
input {
    r#type: "file", multiple: true, accept: ".txt,.rs",
    onchange: move |evt| async move {
        for file in evt.files() {
            let contents = file.read_string().await;
        }
    }
}
```

## Document/Meta
```rust
document::Meta { property: "og:title", content: "My Site" }
document::Meta { name: "description", content: "Description" }
document::Title { "Page Title" }
document::Stylesheet { href: asset!("/style.css") }
```

## Padrões Comuns

### Reducer
```rust
let mut state = use_signal(|| MyState::new());
state.write().reduce(Action::DoSomething);
```

### Conditional platform config
```rust
desktop!({ /* config desktop */ })
native!({ /* config native */ })
server_only!({ /* config server */ })
```

### Platform-specific launch
```rust
#[cfg(not(feature = "server"))]
dioxus::fullstack::set_server_url("https://api.example.com");
```
