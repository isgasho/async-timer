use async_timer::oneshot::{Oneshot, Timer};

use std::time;

#[test]
fn test_oneshot() {
    cute_async::runtime::tokio(async {
        let work = Timer::new(time::Duration::from_secs(2));
        assert!(!work.is_expired());

        let before = time::SystemTime::now();
        work.await;
        let after = time::SystemTime::now();
        let diff = after.duration_since(before).unwrap();

        assert!(diff.as_millis() >= 1_500 && diff.as_millis() <= 2_500);
    });
}

#[test]
fn test_tons_oneshot() {
    cute_async::runtime::tokio(async {
        const NUM: usize = 1024;
        let mut jobs = Vec::with_capacity(NUM);

        for _ in 0..NUM {
            jobs.push(Timer::new(time::Duration::from_secs(2)));
        }

        let before = time::SystemTime::now();
        futures_util::future::join_all(jobs).await;
        let after = time::SystemTime::now();
        let diff = after.duration_since(before).unwrap();

        assert!(diff.as_millis() >= 1_500 && diff.as_millis() <= 2_500);
    });
}

#[test]
fn test_smol_oneshot() {
    cute_async::runtime::tokio(async {
        let work = Timer::new(time::Duration::from_millis(500));
        assert!(!work.is_expired());

        let before = time::SystemTime::now();
        work.await;
        let after = time::SystemTime::now();
        let diff = after.duration_since(before).unwrap();

        assert!(diff.as_millis() >= 250 && diff.as_millis() <= 750);
    });
}

#[test]
fn test_nano_oneshot() {
    cute_async::runtime::tokio(async {
        let work = Timer::new(time::Duration::from_nanos(604000));
        assert!(!work.is_expired());
        work.await;
    });
}
