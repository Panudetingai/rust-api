use governor::middleware::NoOpMiddleware;
use std::{sync::Arc, time::Duration};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
};

// คืนค่าเป็น GovernorLayer โดยใช้ Default Generics (PeerIpKeyExtractor)
pub fn ratelimitapi() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware, axum::body::Body> {
    // 1. สร้าง Config
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .finish()
        .unwrap();

    let governor_conf_arc = Arc::new(governor_conf);
    // 3. ใช้ tokio::spawn แทน std::thread ในงาน Async (Axum)
    let governor_conf_for_task = governor_conf_arc.clone();
    tokio::spawn(async move {
        let interval = Duration::from_secs(60);
        let mut interval_timer = tokio::time::interval(interval);

        loop {
            interval_timer.tick().await;
            let limiter = governor_conf_for_task.limiter().clone();
            tracing::info!("rate limiting storage size: {}", limiter.len());
            limiter.retain_recent();
        }
    });

    // 4. ส่ง Config ตัวต้นฉบับให้ Layer
    GovernorLayer::new(governor_conf_arc)
}
