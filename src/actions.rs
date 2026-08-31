use crate::render::{render, Layout};
use crate::settings::Settings;
use crate::state;
use crate::usage::{fetch_usage, percent_by_name};

use dashmap::DashMap;
use openaction::async_trait;
use openaction::{Instance, OpenActionResult};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;

const WINDOWS: [&str; 3] = ["5h", "week", "month"];

#[derive(Clone, Debug)]
enum Kind {
    Single(String),
    Rotate(Arc<DashMap<String, usize>>),
    Summary,
}

struct LoopState {
    handles: Arc<DashMap<String, JoinHandle<()>>>,
}

impl LoopState {
    fn new() -> Self {
        Self {
            handles: Arc::new(DashMap::new()),
        }
    }

    fn start(&self, instance_id: String, settings: Settings, kind: Kind) {
        if let Some((_, h)) = self.handles.remove(&instance_id) {
            h.abort();
        }
        let handles = self.handles.clone();
        let loop_id = instance_id.clone();
        let handle = tokio::spawn(async move {
            loop {
                let keep = match openaction::get_instance(loop_id.clone()).await {
                    Some(inst) => paint(&inst, &kind, &settings).await,
                    None => false,
                };
                if !keep {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(settings.refresh_seconds())).await;
            }
        });
        handles.insert(instance_id, handle);
    }

    fn stop(&self, instance_id: &str) {
        if let Some((_, h)) = self.handles.remove(instance_id) {
            h.abort();
        }
    }
}

async fn paint(instance: &Instance, kind: &Kind, settings: &Settings) -> bool {
    match kind {
        Kind::Single(w) => {
            refresh_single(instance, settings, w).await;
        }
        Kind::Rotate(index_map) => {
            let i = index_map
                .get(&instance.instance_id)
                .map(|v| *v)
                .unwrap_or(0);
            let window = WINDOWS[i % WINDOWS.len()];
            refresh_single(instance, settings, window).await;
        }
        Kind::Summary => {
            refresh_summary(instance, settings).await;
        }
    }
    true
}

async fn refresh_single(instance: &Instance, settings: &Settings, window: &str) {
    if settings.api_key.is_empty() {
        set_single(instance, settings, window, 0, (255, 255, 255)).await;
        return;
    }
    match fetch_usage(&settings.api_key).await {
        Ok(usage) => {
            state::store(usage.clone());
            let pct = percent_by_name(&usage, window).unwrap_or(0);
            set_single(instance, settings, window, pct, settings.font_rgb()).await;
        }
        Err(_) => {
            set_single(instance, settings, window, 0, (255, 60, 60)).await;
        }
    }
}

async fn set_single(
    instance: &Instance,
    settings: &Settings,
    window: &str,
    pct: u8,
    font: (u8, u8, u8),
) {
    let img = render(
        &Layout::single(pct, window),
        font,
        settings.threshold_yellow(),
        settings.threshold_red(),
    );
    let _ = instance.set_image(Some(img), None).await;
}

async fn refresh_summary(instance: &Instance, settings: &Settings) {
    if settings.api_key.is_empty() {
        set_summary(instance, settings, None, None, None, (255, 255, 255)).await;
        return;
    }
    match fetch_usage(&settings.api_key).await {
        Ok(usage) => {
            state::store(usage.clone());
            set_summary(
                instance,
                settings,
                usage.rolling.as_ref().map(|w| w.percent),
                usage.weekly.as_ref().map(|w| w.percent),
                usage.monthly.as_ref().map(|w| w.percent),
                settings.font_rgb(),
            )
            .await;
        }
        Err(_) => {
            set_summary(instance, settings, None, None, None, (255, 60, 60)).await;
        }
    }
}

async fn set_summary(
    instance: &Instance,
    settings: &Settings,
    rolling: Option<u8>,
    weekly: Option<u8>,
    monthly: Option<u8>,
    font: (u8, u8, u8),
) {
    let img = render(
        &Layout::summary(rolling, weekly, monthly),
        font,
        settings.threshold_yellow(),
        settings.threshold_red(),
    );
    let _ = instance.set_image(Some(img), None).await;
}

pub struct WindowAction {
    loops: LoopState,
}

impl Default for WindowAction {
    fn default() -> Self {
        Self {
            loops: LoopState::new(),
        }
    }
}

#[async_trait]
impl openaction::Action for WindowAction {
    const UUID: &'static str = "com.dahrkael.opencodeinfo.window";

    type Settings = Settings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.start(
            instance.instance_id.clone(),
            settings.clone(),
            Kind::Single(settings.window.clone()),
        );
        Ok(())
    }

    async fn did_receive_settings(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.start(
            instance.instance_id.clone(),
            settings.clone(),
            Kind::Single(settings.window.clone()),
        );
        Ok(())
    }

    async fn will_disappear(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.stop(&instance.instance_id);
        Ok(())
    }
}

pub struct RotateAction {
    loops: LoopState,
    index: Arc<DashMap<String, usize>>,
}

impl Default for RotateAction {
    fn default() -> Self {
        Self {
            loops: LoopState::new(),
            index: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl openaction::Action for RotateAction {
    const UUID: &'static str = "com.dahrkael.opencodeinfo.rotate";

    type Settings = Settings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.start(
            instance.instance_id.clone(),
            settings.clone(),
            Kind::Rotate(self.index.clone()),
        );
        Ok(())
    }

    async fn did_receive_settings(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.start(
            instance.instance_id.clone(),
            settings.clone(),
            Kind::Rotate(self.index.clone()),
        );
        Ok(())
    }

    async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        let mut i = self
            .index
            .get(&instance.instance_id)
            .map(|v| *v)
            .unwrap_or(0);
        i = (i + 1) % WINDOWS.len();
        self.index.insert(instance.instance_id.clone(), i);
        let window = WINDOWS[i];

        if let Some(usage) = state::get() {
            let pct = percent_by_name(&usage, window).unwrap_or(0);
            set_single(instance, settings, window, pct, settings.font_rgb()).await;
        } else {
            refresh_single(instance, settings, window).await;
        }
        Ok(())
    }

    async fn will_disappear(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.stop(&instance.instance_id);
        self.index.remove(&instance.instance_id);
        Ok(())
    }
}

pub struct SummaryAction {
    loops: LoopState,
}

impl Default for SummaryAction {
    fn default() -> Self {
        Self {
            loops: LoopState::new(),
        }
    }
}

#[async_trait]
impl openaction::Action for SummaryAction {
    const UUID: &'static str = "com.dahrkael.opencodeinfo.summary";

    type Settings = Settings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.start(
            instance.instance_id.clone(),
            settings.clone(),
            Kind::Summary,
        );
        Ok(())
    }

    async fn did_receive_settings(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.start(
            instance.instance_id.clone(),
            settings.clone(),
            Kind::Summary,
        );
        Ok(())
    }

    async fn will_disappear(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.loops.stop(&instance.instance_id);
        Ok(())
    }
}
