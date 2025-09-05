use bitflags::bitflags;
use hid::reports::*;
use hid::descriptors::*;
use hid::descriptor_items::*;
use hut::*;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tonic::Request;
use usb_gadget::{Class, Config, default_udc, Id, Gadget, remove_all, Strings};
use usb_gadget::function::hid::Hid;
use btspeak_key_interceptor::btspeak_key_interceptor_client::BtspeakKeyInterceptorClient;
use btspeak_key_interceptor::{Empty, BrailleKeyEvent};
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
  remove_all().unwrap();
  let input_report = Report {
    ty: ReportType::Input,
    id: Some(1),
    fields: vec![
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot1.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot2.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot3.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot4.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot5.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot6.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot7.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardDot8.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 1,
        logical_minimum: 0,
        logical_maximum: 1,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::BrailleKeyboardSpace.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 7,
        logical_minimum: 0,
        logical_maximum: 127,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: None,
        constant: true,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: None,
        buffered_bytes: false,
      },
    ],
  };
  let output_report = Report {
    ty: ReportType::Output,
    id: Some(2),
    fields: vec![
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
      ReportField::Variable {
        size: 8,
        logical_minimum: 0,
        logical_maximum: 255,
        physical_minimum: None,
        physical_maximum: None,
        unit_exponent: None,
        unit: None,
        usages: Some(vec![BrailleDisplay::EightDotBrailleCell.usage_value()]),
        constant: false,
        relative: false,
        wrap: false,
        linear: true,
        preferred_state: true,
        null_state: false,
        volatile: Some(false),
        buffered_bytes: false,
      },
    ],
  };
  let descriptor = Descriptor {
    items: vec![
      MainItem::Collection(Collection {
        ty: CollectionType::Application,
        usage: Some(BrailleDisplay::BrailleDisplay.usage_value()),
        items: vec![
          MainItem::Collection(Collection {
            ty: CollectionType::Logical,
            usage: None,
            items: vec![
              MainItem::Report(input_report.clone()),
            ],
          }),
          MainItem::Collection(Collection {
            ty: CollectionType::Logical,
            usage: Some(BrailleDisplay::BrailleRow.usage_value()),
            items: vec![
              MainItem::Report(output_report.clone()),
            ],
          }),
        ],
      }),
    ],
  };
  let items = descriptor.into_descriptor_items();
  let data = items_into_bitvec(items);
  let mut builder = Hid::builder();
  builder.sub_class = 0;
  builder.protocol = 0;
  builder.report_desc = data.into();
  builder.report_len = 21;
  let (_function, function_handle) = builder.build();
  let mut config = Config::new("HID gadget")
    .with_function(function_handle);
  config.max_power = 250;
  let _gadget = Gadget::new(Class::interface_specific(), Id::new(0x0525, 0xa4ac), Strings::new("Blazie Technologies", "BT Speak", "btspeak-braille-terminal"))
    .with_config(config)
    .bind(&default_udc().unwrap())
    .unwrap();
  let (event_tx, mut event_rx) = mpsc::channel::<BrailleKeyEvent>(32);
  let device = File::options().read(true).write(true).open("/dev/hidg0").await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
  let (mut reader, mut writer) = io::split(device);
  tokio::spawn(async move {
    let mut keys = KeyFlags::empty();
    while let Some(event) = event_rx.recv().await {
      let flag = match event.dot {
        1 => KeyFlags::Dot1,
        2 => KeyFlags::Dot2,
        3 => KeyFlags::Dot3,
        4 => KeyFlags::Dot4,
        5 => KeyFlags::Dot5,
        6 => KeyFlags::Dot6,
        7 => KeyFlags::Dot7,
        8 => KeyFlags::Dot8,
        _ => KeyFlags::Space,
      };
      if event.release {
        keys -= flag;
      }
      else {
        keys |= flag;
      };
      let mut report = write_report(input_report.clone(), vec![
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot1) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot2) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot3) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot4) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot5) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot6) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot7) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Dot8) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(if keys.contains(KeyFlags::Space) { 1 } else { 0 })),
        ReportFieldValue::UnsignedVariable(Some(0)),
      ]);
      let mut report_data = report.split_off(8);
      for byte in report_data.chunks_mut(8) {
        byte.reverse();
      };
      report.append(&mut report_data);
      let report: Vec<u8> = report.into();
      writer.write(&report).await.unwrap();
    };
  });
  tokio::spawn(async move {
    let mut buffer = vec![0; 64];
    while let Ok(count) = reader.read(&mut buffer).await {
      buffer.resize(count, 0);
      println!("Report received");
      println!("{:?}", read_report(output_report.clone(), buffer.clone().try_into().unwrap()));
      buffer.resize(64, 0);
    };
  });
  let addr = "http://127.0.0.1:54123";
  let mut client = BtspeakKeyInterceptorClient::connect(addr).await.unwrap();
  let mut stream = client.grab_key_events(Request::new(Empty {})).await.unwrap().into_inner();
  tokio::spawn(async move {
    while let Some(event) = stream.message().await.unwrap() {
      event_tx.send(event).await.unwrap();
    };
  }).await.unwrap();
}
