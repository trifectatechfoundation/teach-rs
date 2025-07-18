use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

#[derive(Debug)]
pub enum SendError<T> {
    ReceiverDropped(T),
}

#[derive(Debug)]
pub enum RecvError {
    SenderDropped,
}

pub struct Inner<T> {
    /// The buffer containing the message.
    data: Option<T>,
    /// The waker used to wake the Receiver `Future`
    waker: Option<Waker>,
    /// Indicates whether the `Receiver` was dropped
    rx_dropped: bool,
    /// Indicates whether the `Sender` was dropped
    tx_dropped: bool,
}

pub struct Receiver<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        todo!("Implement me")
    }
}

pub struct Sender<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

impl<T> Sender<T> {
    pub fn send(self, value: T) -> Result<(), SendError<T>> {
        todo!("Implement me")
    }
}

/// Create a new broadcast channel
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        data: None,
        waker: None,
        tx_dropped: false,
        rx_dropped: false,
    };
    let inner = Arc::new(Mutex::new(inner));
    let tx = Sender {
        inner: inner.clone(),
    };
    let rx = Receiver { inner };
    (tx, rx)
}

#[cfg(test)]
mod tests {
    use tokio::task;

    use crate::oneshot::{channel, RecvError, SendError};

    #[tokio::test]
    async fn test_send_recv() {
        let (tx, rx) = channel();

        tx.send(123).expect("Error sending value");

        assert_eq!(rx.await.expect("Error receiving value"), 123);

        let (tx, rx) = channel();

        let recv_task = task::spawn(rx);

        tx.send(123).expect("Error sending value to task");

        assert_eq!(
            recv_task
                .await
                .expect("Error joining recv_task")
                .expect("Error receiving value in task"),
            123
        );
    }

    #[tokio::test]
    async fn test_drop() {
        let (tx, rx) = channel();
        drop(rx);
        assert!(matches!(tx.send(123), Err(SendError::ReceiverDropped(123))));

        let (tx, rx) = channel::<()>();
        drop(tx);
        assert!(matches!(rx.await, Err(RecvError::SenderDropped)))
    }
}
