use super::*;

pub struct Message {
  pub flaw: Option<Flaw>,
  pub edicts: Vec<Edict>,
  pub fields: HashMap<u128, VecDeque<u128>>,
}

impl Message {
  pub fn from_integers(output_len: u32, payload: &[u128]) -> Self {
    let mut edicts = Vec::new();
    let mut fields = HashMap::<u128, VecDeque<u128>>::new();
    let mut flaw = None;

    for i in (0..payload.len()).step_by(2) {
      let tag = payload[i];

      if Tag::Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(output_len, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          edicts.push(edict);
        }
        break;
      }

      let Some(&value) = payload.get(i + 1) else {
        flaw.get_or_insert(Flaw::TruncatedField);
        break;
      };

      fields.entry(tag).or_default().push_back(value);
    }

    Self {
      flaw,
      edicts,
      fields,
    }
  }
}