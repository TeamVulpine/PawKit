use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::{
    js_sys::{ArrayBuffer, JsString, Uint8Array},
    BinaryType, MessageEvent, WebSocket,
};

use crate::{WebsocketError, WebsocketMessage};

struct WebsocketEventQueue<TRecieve, TEvent>
where
    TRecieve: Sized,
    TEvent: Sized + FromWasmAbi + 'static,
{
    reciever: UnboundedReceiver<TRecieve>,
    callback: Arc<Closure<dyn FnMut(TEvent)>>,
}

impl<TRecieve, TEvent> WebsocketEventQueue<TRecieve, TEvent>
where
    TRecieve: Sized,
    TEvent: Sized + FromWasmAbi + 'static,
{
    fn new<FCallback>(mut f: FCallback) -> Self
    where
        FCallback: FnMut(UnboundedSender<TRecieve>) -> Box<dyn FnMut(TEvent)>,
    {
        let (tx, rx) = mpsc::unbounded_channel();

        let callback = {
            let tx = tx.clone();

            Closure::wrap(f(tx) as Box<dyn FnMut(_)>)
        };

        return Self {
            reciever: rx,
            callback: Arc::new(callback),
        };
    }

    async fn recv(&mut self) -> Option<TRecieve> {
        return self.reciever.recv().await;
    }
}

#[derive(Clone)]
struct CallbackFuture<F>
where
    F: FnMut() -> bool,
{
    callback: F,
}

impl<F> CallbackFuture<F>
where
    F: FnMut() -> bool + Unpin,
{
    async fn wait_for(callback: F) {
        Self { callback }.await;
    }
}

impl<F> Future for CallbackFuture<F>
where
    F: FnMut() -> bool + Unpin,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if (self.get_mut().callback)() {
            return Poll::Ready(());
        }

        return Poll::Pending;
    }
}

pub struct Websocket {
    raw_sock: WebSocket,
    message_queue: WebsocketEventQueue<WebsocketMessage, MessageEvent>,
}

impl Websocket {
    pub async fn new(url: &str) -> Option<Self> {
        let Ok(sock) = WebSocket::new(url) else {
            return None;
        };

        sock.set_binary_type(BinaryType::Arraybuffer);

        let message_queue = WebsocketEventQueue::new(|tx| {
            Box::new(move |event: MessageEvent| {
                if let Ok(text) = event.data().dyn_into::<JsString>() {
                    let _ = tx.send(WebsocketMessage::String(text.as_string().unwrap()));
                } else if let Ok(buf) = event.data().dyn_into::<ArrayBuffer>() {
                    let array = Uint8Array::new(&buf);
                    let _ = tx.send(WebsocketMessage::Array(array.to_vec()));
                }
            })
        });

        sock.set_onmessage(Some(
            message_queue.callback.as_ref().as_ref().unchecked_ref(),
        ));

        CallbackFuture::wait_for(|| sock.ready_state() == WebSocket::OPEN).await;

        return Some(Self {
            raw_sock: sock,
            message_queue,
        });
    }

    pub fn is_open(&self) -> bool {
        return self.raw_sock.ready_state() == WebSocket::OPEN;
    }

    pub async fn close(&mut self) {
        self.raw_sock.set_onmessage(None);

        self.raw_sock.close().unwrap();
    }

    pub async fn recv(&mut self) -> Option<WebsocketMessage> {
        if !self.is_open() {
            return None;
        }

        self.message_queue.recv().await
    }

    pub async fn send(&mut self, message: WebsocketMessage) -> Result<(), WebsocketError> {
        if !self.is_open() {
            return Err(WebsocketError::NotOpen);
        }

        match &message {
            WebsocketMessage::Array(arr) => {
                if self.raw_sock.send_with_u8_array(arr).is_err() {
                    return Err(WebsocketError::InvalidState);
                }
            }

            WebsocketMessage::String(str) => {
                if self.raw_sock.send_with_str(str).is_err() {
                    return Err(WebsocketError::InvalidState);
                }
            }
        }

        return Ok(());
    }
}

impl Drop for Websocket {
    fn drop(&mut self) {
        self.raw_sock.set_onmessage(None);

        self.raw_sock.close().unwrap();
    }
}
