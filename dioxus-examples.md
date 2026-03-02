# Dioxus Examples Map

Todos os exemplos em `dioxus/examples/`, organizados por categoria.

## 01-app-demos - Aplicações completas

| Exemplo | Arquivo | Demonstra |
|---------|---------|-----------|
| hello_world | `hello_world.rs` | Mínimo: `dioxus::launch(app)`, `rsx!` básico |
| repo_readme | `repo_readme.rs` | Counter simples com `use_signal`, `+=` em signal |
| counters | `counters.rs` | `use_signal(Vec)`, `use_memo`, iteração com `for`, `asset!` CSS |
| calculator | `calculator.rs` | Closures para state, keyboard events, `LaunchBuilder`, config desktop/native |
| calculator_mutable | `calculator_mutable.rs` | State em struct única com `write()`, `#[component]`, `EventHandler` |
| todomvc | `todomvc.rs` | `HashMap` em signal, `use_memo` para filtros, `WriteSignal` como prop, `KeyboardEvent` |
| todomvc_store | `todomvc_store.rs` | `#[derive(Store)]`, `use_store`, `#[store] impl`, fine-grained reactivity |
| dog_app | `dog_app.rs` | `use_loader` com `?`, `use_action` com `.call()`, fetch API |
| crm | `crm.rs` | `Router`, `GlobalSignal`, `Link`, formulários, `onmounted` focus |
| weather_app | `weather_app.rs` | `use_loader` reativo, SVG inline, `Loading::Pending/Failed`, search com API |
| image_generator_openai | `image_generator_openai.rs` | `use_action` com reqwest POST, loading state com `pending()` |
| websocket_chat | `websocket_chat.rs` | `use_websocket`, `#[get]` com WebSocket, broadcast channel, `use_future` recv loop |
| **hackernews/** | `hackernews/src/main.rs` | Fullstack SSR, `SuspenseBoundary`, `ChildrenOrLoading`, recursive components |
| **ecommerce-site/** | `ecommerce-site/src/` | Multi-arquivo, Router, componentes modulares, Tailwind |
| **file-explorer/** | `file-explorer/src/main.rs` | Desktop filesystem, estado em struct, `PathBuf`, Material Icons |
| **hotdog/** | `hotdog/src/` | Fullstack completo: frontend + backend SQLite, `#[get]`/`#[post]`/`#[delete]`, favorites |
| **bluetooth-scanner/** | `bluetooth-scanner/src/main.rs` | Native plugin pattern |
| **geolocation/** | `geolocation-native-plugin/src/` | Plugin nativo com módulos de erro/models |

## 02-building-ui - Construção de UI

| Exemplo | Demonstra |
|---------|-----------|
| nested_listeners | Event bubbling, `stop_propagation()` |
| disabled | Atributos condicionais, `signal.toggle()`, signal como atributo direto |
| svg | SVG com namespace, `view_box`, `circle`, `rect`, random com `rand` |

## 03-assets-styling - Assets e Estilos

| Exemplo | Demonstra |
|---------|-----------|
| custom_assets | `asset!()` macro para imagens/CSS estáticos |
| dynamic_assets | `use_asset_handler` (desktop only), interceptar requests do webview |
| css_modules | `#[css_module]` macro, `Styles::class_name` |
| meta | `document::Meta` para og:tags |
| meta_elements | `Meta` direto do prelude |

## 04-managing-state - Gerenciamento de Estado

| Exemplo | Demonstra |
|---------|-----------|
| signals | `use_signal`, `use_memo`, `use_effect`, `use_future`, `use_resource`, early return, `ReadSignal` prop |
| context_api | `use_context_provider`, `use_context`, `try_use_context` |
| global | `GlobalSignal`, `GlobalMemo`, `Signal::global()`, `Memo::global()` |
| memo_chain | Memos encadeados, componentes recursivos, pausa de memos |
| reducer | Reducer pattern com struct + enum de actions |
| error_handling | `ErrorBoundary`, `handle_error`, `?` em event handlers, panic catching, Router |

## 05-using-async - Programação Assíncrona

| Exemplo | Demonstra |
|---------|-----------|
| future | `use_future` (loop), `use_effect` com `spawn`, async onclick |
| backgrounded_futures | Pausa de futures em early return, `use_future`/`use_effect` lifecycle |
| clock | Timer com `Instant::now()`, formatting de tempo |
| streams | `Stream` + `StreamExt::next()` dentro de `use_future` |
| suspense | `SuspenseBoundary`, `ErrorBoundary`, `use_loader` com `?` |

## 06-routing - Roteamento

| Exemplo | Demonstra |
|---------|-----------|
| simple_router | `#[derive(Routable)]`, `#[layout]`, `Link`, `Outlet`, path params |
| router | Rotas aninhadas, `#[nest]`, `#[redirect]`, `#[end_layout]`, 404 |
| flat_router | Layout flat com footer, múltiplas páginas simples |
| link | Links internos vs externos, `prevent_default()` |
| router_resource | `ReadSignal<i32>` em props de rota, `use_resource` reativo |
| query_segment_search | Query params (`?:query&:count`), `navigator().replace()` |
| router_restore_scroll | Restauração de scroll position |
| hash_fragment_state | Estado no hash fragment |

## 07-fullstack - Server Functions

| Exemplo | Demonstra |
|---------|-----------|
| fullstack_hello_world | `#[get]` básico com path + query params, `use_action` |
| server_functions | GET/POST, server-only extractors, `IntoResponse`, `FromResponse`, `#[server]` anônimo |
| streaming | `TextStream`, `Streaming<T, JsonEncoding>`, byte streams, channels |
| websocket | `Websocket<Client, Server, CborEncoding>`, `use_websocket`, auto-reconnect |
| login_form | `Form<T>`, `SetCookie`, `SetHeader`, `TypedHeader<Cookie>`, auth flow |
| middleware | Middleware em server functions |
| custom_error_page | Páginas de erro customizadas |
| redirect | Redirects no server |
| header_map | Acesso a headers |
| query_params | Query params no fullstack |
| multipart_form | Upload multipart |
| streaming_file_upload | Upload de arquivo via streaming |
| server_sent_events | SSE |
| custom_axum_serve | Axum router customizado |
| dog_app_self_hosted | App self-hosted |
| through_reqwest | Chamada via reqwest direto |
| server_state | Estado persistente com SQLx |
| handling_errors | Tratamento de erros tipados |
| full_request_access | Acesso completo ao request |
| **hello-world/** | Projeto fullstack mínimo |
| **router/** | Fullstack com router |
| **desktop/** | Fullstack no desktop |
| **auth/** | Sistema de auth completo |
| **ssr-only/** | SSR sem hydration |

## 08-apis - APIs da Plataforma

| Exemplo | Demonstra |
|---------|-----------|
| form | Formulários HTML completos, oninput, onsubmit, checkbox, radio, select, file |
| eval | `document::eval()`, comunicação bidirecional Rust <-> JS |
| file_upload | `evt.files()`, `FileData::read_string()`, drag-and-drop zone |
| drag_and_drop | Kanban board, `data_transfer().set_data/get_data`, draggable |
| control_focus | `MountedData`, `set_focus()`, `use_future` loop |
| read_size | `MountedData::get_client_rect()`, dimensões de elementos |
| on_visible | `onvisible` (IntersectionObserver), animações CSS on scroll |
| on_resize | Evento de resize |
| video_stream | `use_asset_handler` para streaming de vídeo, HTTP range requests |
| multiwindow | `window().new_window()`, VirtualDom separado |
| multiwindow_with_tray_icon | Tray icon + múltiplas janelas |
| window_event | `window().drag/minimize/fullscreen/close/decorations/title` |
| window_popup | Popup windows |
| window_focus | Controle de foco da janela |
| window_zoom | Zoom da janela |
| custom_html | HTML customizado no desktop |
| custom_menu | `Menu`, `Submenu`, `MenuItem`, `use_muda_event_handler` |
| shortcut | `use_global_shortcut("ctrl+s", handler)` |
| overlay | Janela overlay |
| ssr | Server-side rendering manual |
| title | `document::Title` |
| logging | Logger do Dioxus |
| scroll_to_top | Scroll programático |
| scroll_to_offset | Scroll com offset |
| wgpu_child_window | Integração WGPU |

## 09-reference - Referência

| Exemplo | Demonstra |
|---------|-----------|
| rsx_usage | Tour completo do RSX: formatação, conditionals, iteradores, fragments, componentes genéricos, spread |
| optional_props | `#[props(default)]`, `#[props(!optional)]`, `#[props(optional)]` |
| spread | `#[props(extends = GlobalAttributes)]`, `..props.attributes` |
| generic_component | Componentes com type parameters |
| shorthand | Sintaxe abreviada |
| simple_list | Listas simples |
| all_events | Todos os eventos com logging |
| web_component | Web components customizados |
| xss_safety | Proteção XSS |

## 10-integrations - Integrações

| Exemplo | Demonstra |
|---------|-----------|
| **tailwind/** | Tailwind CSS com `asset!`, classes condicionais |
| **pwa/** | Progressive Web App |
| **wgpu-texture/** | Renderização WGPU com texturas |
| **native-headless/** | Dioxus native sem janela |
| **native-headless-in-bevy/** | Dioxus headless dentro do Bevy |
| **bevy/** | Integração completa Dioxus + Bevy game engine |
