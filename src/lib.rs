#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate mecab;

use mecab::Tagger;
use std::vec::Vec;

#[napi(object)]
pub struct NodeInfo {
  pub id: u32,
  pub surface: String,
  pub feature: String,
  pub len: u16,
  pub rc_attr: u16,
  pub lc_attr: u16,
  pub posid: u16,
  pub char_type: u8,
  pub stat: u8,
  pub isbest: i32,
  pub alpha: f64,
  pub beta: f64,
  pub prob: f64,
  pub cost: i64,
}

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn parse_as_node(input: String) -> Vec<NodeInfo> {
  let mut tagger = Tagger::new("");
  let my_input: &str = input.as_str();  // 转换为 &str
  // for node in tagger.parse_to_node(my_input).iter_next() {
  //   match node.stat as i32 {
  //     mecab::MECAB_BOS_NODE => {
  //       print!("{} BOS ", node.id);
  //     },
  //     mecab::MECAB_EOS_NODE => {
  //       print!("{} EOS ", node.id);
  //     },
  //     _ => {
  //       print!("{} {} ", node.id, &(node.surface)[..(node.length as usize)]);
  //     }
  //   }

  //   println!("{} {} {} {} {} {} {} {} {} {} {} {} {}",
  //     node.feature,
  //     input.len() as isize - node.surface.len() as isize,
  //     input.len() as isize - node.surface.len() as isize  + node.length as isize,
  //     node.rcattr,
  //     node.lcattr,
  //     node.posid,
  //     node.char_type,
  //     node.stat,
  //     node.isbest,
  //     node.alpha,
  //     node.beta,
  //     node.prob,
  //     node.cost);
  // }

  let mut result = Vec::new();
  for node in tagger.parse_to_node(my_input).iter_next() {
      let node_info = NodeInfo {
        id: node.id,
        surface: (node.surface)[..(node.length as usize)].to_string(),
        feature: node.feature.to_string(),
        len: node.length,
        rc_attr: node.rcattr,
        lc_attr: node.lcattr,
        posid: node.posid,
        char_type: node.char_type,
        stat: node.stat,
        isbest: if node.isbest { 1 } else { 0 },
        alpha: node.alpha as f64,
        beta: node.beta as f64,  
        prob: node.prob as f64,
        cost: node.cost,
    };
    result.push(node_info);
  }
  result
}

// #[napi]
// fn parse_as_node_by_buf(buf: Buffer) {
//   let buf: Vec<u8> = buf.into();
//   // do something
// }
