use bitflags::bitflags;
use tonic::Request;
use btspeak_key_interceptor::btspeak_key_interceptor_client::BtspeakKeyInterceptorClient;
use btspeak_key_interceptor::{Empty, BrailleKeyCombination, BrailleKeyCombinations, BrailleKeyEvent, BrailleKeyEvents};
mod btspeak_key_interceptor {
  tonic::include_proto!("btspeak_key_interceptor");
}
bitflags! {
  #[derive(Debug, PartialEq, Eq, Clone)]
  struct KeyFlags: u16 {
    const Dot1 = 1;
    const Dot2 = 1 << 1;
    const Dot3 = 1 << 2;
    const Dot4 = 1 << 3;
    const Dot5 = 1 << 4;
    const Dot6 = 1 << 5;
    const Dot7 = 1 << 6;
    const Dot8 = 1 << 7;
    const Space = 1 << 8;
  }
}
#[tokio::main]
async fn main() {
  let addr = "http://127.0.0.1:54123";
  let mut client = BtspeakKeyInterceptorClient::connect(addr).await.unwrap();
  let mut stream = client.grab_key_combinations(Request::new(Empty {})).await.unwrap().into_inner();
  let mut stream2 = client.grab_key_events(Request::new(Empty {})).await.unwrap().into_inner();
  tokio::spawn(async move {
    while let Some(combination) = stream.message().await.unwrap() {
      println!("{:?}", combination);
    };
  });
  tokio::spawn(async move {
    while let Some(event) = stream2.message().await.unwrap() {
      println!("{:?}", event);
    };
  });
  tokio::spawn(async move {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    client.set_excluded_key_combinations(Request::new(BrailleKeyCombinations { combinations: vec!(BrailleKeyCombination { dots: 14, space: false })})).await.unwrap();
    client.set_excluded_key_events(Request::new(BrailleKeyEvents { events: vec!(
      BrailleKeyEvent { dot: 1, release: false },
      BrailleKeyEvent { dot: 1, release: true },
    )})).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    client.release_keyboard(Request::new(Empty {})).await.unwrap();
  });
  tokio::time::sleep(tokio::time::Duration::from_secs(35)).await;
}
