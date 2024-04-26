use wasm_bindgen::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
  num::ParseIntError,
  str::FromStr,
  fmt::{self, Display, Formatter},
  {
    collections::{HashMap, VecDeque},
    convert::{TryFrom}
  },
};
use core::convert::TryInto;
// use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use hex;


mod rune;
use rune::*;

mod rune_id;
use rune_id::*;

mod edict;
use edict::*;

mod flaw;
use flaw::*;

mod message;
use message::*;

mod tag;
use tag::*;

mod varint;
use varint::*;

mod cenotaph;
use cenotaph::*;

mod etching;
use etching::*;

mod terms;
use terms::*;

mod flag;
use flag::*;

mod runestone;
use runestone::*;

mod artifact;
use artifact::*;

mod opcodes;
use opcodes::*;

fn default<T: Default>() -> T {
  Default::default()
}

mod push_bytes;
use push_bytes::*;

mod instruction;
use instruction::*;


enum Payload {
  Valid(Vec<u8>),
  Invalid(Flaw),
}



#[wasm_bindgen]
pub fn decipher(output_len: u32, script_pubkey_string: String) -> String {
  let script_bytes = match hex::decode(script_pubkey_string) {
    Ok(bytes) => bytes,
    Err(_) => {
      return "error".to_string();
    }
  };
  let mut instructions = Instructions {
      data: script_bytes.iter(),
      enforce_minimal: false,
  };

  if let Some(Ok(Instruction::Op(opcodes::all::OP_RETURN))) = instructions.next() {

  } else {
    return "error".to_string();
  }

  if let Some(Ok(Instruction::Op(Runestone::MAGIC_NUMBER))) = instructions.next() {

  } else {
    return "error".to_string();
  }

  let mut payload = Vec::new();

  for result in instructions {
    match result {
      Ok(Instruction::PushBytes(push)) => {
        payload.extend_from_slice(push.as_bytes());
      }
      Ok(Instruction::Op(_)) => {
        let cenotaph = Artifact::Cenotaph(Cenotaph {
          flaw: Some(Flaw::Opcode),
          ..default()
        });
        match serde_json::to_string(&cenotaph) {
          Ok(json_str) => {
            return json_str;
          },
          Err(e) => {
            return "error".to_string();
          },
        }
      }
      Err(_) => {
        let cenotaph = Artifact::Cenotaph(Cenotaph {
          flaw: Some(Flaw::InvalidScript),
          ..default()
        });
        match serde_json::to_string(&cenotaph) {
          Ok(json_str) => {
            return json_str;
          },
          Err(e) => {
            return "error".to_string();
          },
        }
      }
    }
  }


  let artifact = Runestone::decipher(output_len, payload);

  match artifact {
    Some(artifact) => {
      match serde_json::to_string(&artifact) {
        Ok(json_str) => {
          return json_str;
        },
        Err(e) => {
          return "error".to_string();
        },
      }
    },
    None => {
      return "error".to_string();
    }
  }
}







