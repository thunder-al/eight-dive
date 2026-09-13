use crate::components::sample_cluster_info::SampleClusterInfo;
use crate::k8s::KubeConnection;
use crate::tokio_runtime_wrapper::TokioRuntimeHandle;
use gpui_kit::component::*;
use gpui_kit::*;

pub struct AppUi {
    view: Option<AnyView>,
}

impl Render for AppUi {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // cheap router
        // TODO: migrate to something better. enum or proper router mod maybe?

        let theme = cx.theme();

        match &self.view {
            Some(view) => view.clone().into_any_element(),
            None => label::Label::new("View is not defined")
                .text_xl()
                .font_semibold()
                .text_color(theme.danger)
                .into_any_element(),
        }
    }
}

impl AppUi {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            // only view/page for now. foundation for a router/layout system
            view: Some(AnyView::from(
                cx.new(|cx| SampleClusterInfo::new(window, cx)),
            )),
        }
    }

    /// init gpui + gpui-components, create window and open it.
    /// all dependencies are manually wired
    pub fn launch(kube_client: KubeConnection, tokio_rt_handle: TokioRuntimeHandle) {
        application().with_assets(assets::Assets).run(move |cx| {
            // init gpui components
            gpui_kit::init(cx);

            // TODO: hook up gpui kit theme to system's + dark/light switch

            // register dependencies/services
            cx.set_global(tokio_rt_handle);
            cx.set_global(kube_client);

            let win_size = gpui::Size::new(px(1200.0), px(760.0));
            let window_options = WindowOptions {
                window_bounds: Some(WindowBounds::centered(win_size, cx)),
                ..Default::default()
            };

            // gpui async rt entry
            cx.spawn(async move |cx| {
                // expected only one window for the app
                // refactor `AppUi` into window manager if multiple needed
                cx.open_window(window_options, |window, app| {
                    Self::sync_main_window_title(window, app);

                    // make a root app view
                    let view = app.new(|cx| Self::new(window, cx));

                    // app view -> gpui kit wrapper -> render
                    app.new(|cx| Root::new(view, window, cx))
                })
                    .expect("fail to open the window");
            })
                .detach();
        });
    }

    fn sync_main_window_title(window: &mut Window, app: &mut App) {
        let sync_title = |window: &mut Window, app: &mut App| {
            let cluster_name = KubeConnection::global(app).get_display_cluster_name();

            if let Some(cluster_name) = cluster_name {
                window.set_window_title(format!("EightDive: {cluster_name}").as_str());
            } else {
                window.set_window_title("EightDive");
            }
        };

        window
            .observe_global::<KubeConnection>(app, sync_title)
            .detach();

        sync_title(window, app);
    }
}
