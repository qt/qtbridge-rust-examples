// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

use qtbridge::{QApp, QmlObject, qobject};
use std::time::Instant;
use tokio::runtime::Builder;

pub struct Backend {
    hostname: String,
    status: bool,
    latency: u64,
    // A dedicated single-threaded Tokio runtime for async HTTP requests.
    // Keeping it inside the struct ties its lifetime to the Backend.
    runtime: tokio::runtime::Runtime,
}

impl Default for Backend {
    fn default() -> Self {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        Self {
            hostname: "https://www.qt.io".to_string(),
            status: false,
            latency: 0,
            runtime
        }
    }
}

// #[qobject] exposes the struct to the QML side
#[qobject]
impl Backend {
    // qproperty! exposes a Rust field as a QML property.
    qproperty!("latency", Member = latency, Write = set_latency, Notify = latency_changed);
    qproperty!("status", Member = status, Write = set_status, Notify = status_changed);
    qproperty!("hostname", Member = hostname, Write = set_hostname, Notify = hostname_changed);

    fn set_status(&mut self, status: bool) {
        self.status = status;
        self.status_changed();
    }

    fn set_latency(&mut self, latency: u64) {
        self.latency = latency;
        self.latency_changed();
    }

    fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
        self.hostname_changed();
    }

    #[qsignal]
    fn hostname_changed(&mut self);

    #[qsignal]
    fn latency_changed(&mut self);

    #[qsignal]
    fn status_changed(&mut self);

    #[qslot]
    fn make_request(&self) {
        // QThreadHandle will be passed to the async task so it can schedule
        // a closure to run on this Backend on the Qt main thread after the
        // request completes.
        let handle = self.get_thread_handle();
        let hostname: String = self.hostname.clone();
        self.runtime.spawn(async move {
            let start = Instant::now();
            let result = match reqwest::get(hostname).await {
                Ok(resp) => resp.status().is_success(),
                Err(_) => false,
            };
            let latency = if result {
                start.elapsed().as_millis() as u64
            } else {
                0
            };

            // invoke_mut schedules the closure to run on the Qt main thread with
            // mutable access to the Backend; if the Backend has been dropped by
            // then, the closure is discarded.
            handle.invoke_mut(move |backend| backend.update_status(result, latency));
        });
    }

    fn update_status(&mut self, status: bool, latency: u64) {
        self.set_status(status);
        self.set_latency(latency);
    }
}

fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
