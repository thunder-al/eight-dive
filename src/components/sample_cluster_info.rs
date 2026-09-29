use crate::k8s::{KubeConnection, call};
use crate::tokio_runtime_wrapper::TokioRuntimeHandle;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use gpui_kit::component::*;
use gpui_kit::*;
use k8s_openapi::api::core::v1::Namespace;

/// Sample low effort UI table view.
/// Primary goal is to demonstrate gpui/smol->tokio calls and data fetching
pub struct SampleClusterInfo {
    namespaces: Option<Vec<Namespace>>, // TODO: handle loading/error state
}

impl SampleClusterInfo {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { namespaces: None }
    }

    fn cluster_name(cx: &App) -> Option<String> {
        KubeConnection::global(cx).get_display_cluster_name()
    }

    fn load_namespaces(cx: &mut Context<Self>) -> Task<()> {
        /*
         * tokio spawn -> poll kube-rs call -> tokio pool loop -> pending + waker
         * tokio loop(kube-rs response -> tokio waker)
         * handoff tokio pending
         * <- tokio_handle = pending/result handle (thin progress holder. no real work on poll)
         * (other tokio calls)
         * --- wgpui/smol `cx.spawn` ---
         * gpui poll `tokio_handle.await` -> instant pending poll -> wire tokio result to wgpu waker
         * <- handoff gpui pending until tokio result (no wgpu polls, tokio does it)
         * (other gpui calls)
         * kube-rs response -> tokio waker -> tokio poll -> gpui waker -> gpui poll
         * <- `tokio_handle.await` resolves with the result
         */

        let client = KubeConnection::global(cx).client();
        let rt = TokioRuntimeHandle::global(cx);
        let tokio_handle = rt.spawn(call::list_namespaces(client));

        cx.spawn(async move |state, cx| {
            match tokio_handle.await {
                Ok(Ok(namespaces)) => {
                    state
                        .update(cx, |state, cx| {
                            state.namespaces = Some(namespaces);
                            cx.notify(); // finally, request repaint
                        })
                        .expect("mutation to be done");
                }
                Ok(Err(err)) => {
                    // real kube request error
                    // TODO: show error to user in ui
                    println!("{err}");
                }
                Err(err) if err.is_cancelled() => {
                    // tokio join error: request is cancelled
                    unreachable!() // at least for now
                    // TODO: implement request cancellation on main table view destruction later
                    //   see `cx.on_release` and `tokio_handle.abort`/`tokio_handle.abort_handle`.
                    // TODO: maybe implement react-query-like query/mutation abstraction.
                }
                Err(err) => {
                    // tokio join error: panic in async
                    println!("task panic: {err}");
                }
            }
        })
    }
}

impl Render for SampleClusterInfo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .h_flex()
            .size_full()
            .justify_center()
            .items_start()
            .px_3()
            .child(if let Some(name) = Self::cluster_name(cx) {
                div()
                    .v_flex()
                    .w(px(800.0))
                    .pt_3()
                    .gap_2()
                    .child(
                        label::Label::new(format!("Cluster {}", name))
                            .text_xl()
                            .font_semibold(),
                    )
                    .child(
                        Button::new("load-ns")
                            .label("Load namespaces")
                            .primary()
                            .on_click(cx.listener(|_, _, _, cx| {
                                // static call + detach: the context is within `cx`
                                Self::load_namespaces(cx).detach();
                            })),
                    )
                    .child(if let Some(namespaces) = &self.namespaces {
                        div().child(
                            Table::new()
                                .child(
                                    TableHeader::new().child(
                                        TableRow::new()
                                            .child(TableHead::new().child("Name"))
                                            .child(TableHead::new().child("Uid")),
                                    ),
                                )
                                .child(TableBody::new().children(namespaces.iter().cloned().map(
                                    |ns| {
                                        TableRow::new()
                                            .child(if let Some(name) = ns.metadata.name {
                                                TableCell::new().child(name)
                                            } else {
                                                TableCell::new()
                                                    .text_color(theme.muted_foreground)
                                                    .italic()
                                                    .child("(empty)")
                                            })
                                            .child(
                                                TableCell::new().child(
                                                    ns.metadata.uid.unwrap_or("".to_owned()),
                                                ),
                                            )
                                    },
                                ))),
                        )
                    } else {
                        div() // no items
                    })
            } else {
                div().h_flex().w(px(500.0)).pt_3().gap_2().child(
                    label::Label::new("No cluster found")
                        .text_xl()
                        .font_semibold()
                        .text_color(cx.theme().danger),
                )
            })
    }
}
