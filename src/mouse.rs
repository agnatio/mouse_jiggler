use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use enigo::{Coordinate, Enigo, Mouse, Settings};

use crate::config::Settings as AppSettings;

pub struct MouseController {
    running: Arc<AtomicBool>,
    last_jiggle_ms: Arc<AtomicU64>,
    start_time: Instant,
}

impl MouseController {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            last_jiggle_ms: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }

    pub fn start(&mut self, settings: &AppSettings) {
        self.start_time = Instant::now();
        self.running.store(true, Ordering::SeqCst);
        self.last_jiggle_ms.store(0, Ordering::SeqCst);
        
        let running = Arc::clone(&self.running);
        let last_jiggle_ms = Arc::clone(&self.last_jiggle_ms);
        let start_time = self.start_time;
        
        let interval_secs = settings.interval_secs;
        let delay_ms = settings.delay_ms;
        let distance = settings.distance;

        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();

            while running.load(Ordering::SeqCst) {
                // Record jiggle time FIRST (before movements)
                let elapsed = start_time.elapsed().as_millis() as u64;
                last_jiggle_ms.store(elapsed, Ordering::SeqCst);

                // Jiggle: move right, then back left (relative to current position)
                enigo.move_mouse(distance, 0, Coordinate::Rel).unwrap();
                thread::sleep(Duration::from_millis(delay_ms));
                enigo.move_mouse(-distance, 0, Coordinate::Rel).unwrap();

                thread::sleep(Duration::from_secs(interval_secs));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn ms_since_last_jiggle(&self) -> u64 {
        let last = self.last_jiggle_ms.load(Ordering::SeqCst);
        if last == 0 {
            return 0;
        }
        let now = self.start_time.elapsed().as_millis() as u64;
        now.saturating_sub(last)
    }
}
