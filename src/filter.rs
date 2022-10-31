use ethers::types::{Address, Bytes, U256};
use hex;
use serde::Serialize;
use serde_repr::Serialize_repr;
use std::{cell::RefCell, rc::Rc};

// Sources
// * https://developerlife.com/2022/02/24/rust-non-binary-tree/

#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum Operator {
    AND = 1,
    OR = 2,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct Filter {
    pub root: Option<NodeRef>,
    #[serde(skip_serializing)]
    next: Option<NodeRef>,
    #[serde(skip_serializing)]
    last: Option<NodeRef>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct Node {
    pub operand: Option<FilterKV>,
    pub operator: Option<Operator>,
    pub nodes: Option<Vec<NodeRef>>,
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct FilterKV {
    pub key: String,

    #[serde(with = "base64")]
    pub value: Vec<u8>,
}

// The API server only accepts base64 encoding for bytes.
mod base64 {
    use serde::Serialize;
    use serde::Serializer;

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::encode(v);
        String::serialize(&base64, s)
    }
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            root: None,
            next: None,
            last: None,
        }
    }

    pub fn to<'a>(&'a mut self, to: &'a str) -> &'a mut Filter {
        let addr: Address = to.parse().unwrap();
        let new = Rc::new(RefCell::new(Node {
            operand: Some(FilterKV {
                key: String::from("to"),
                value: addr.as_bytes().to_vec(),
            }),
            operator: None,
            nodes: None,
        }));

        match &mut self.root {
            // If there's a root already, append this op to `next`'s children
            Some(_) => {
                let mut next = self.next.as_ref().unwrap().borrow_mut();
                match &mut next.nodes {
                    Some(children) => {
                        children.push(new);
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(new);
                        next.nodes = Some(v);
                    }
                }
            }
            // If no root, create it and point next to it.
            None => {
                self.root = Some(new.clone());
                self.next = Some(new);
            }
        };
        self
    }

    pub fn from<'a>(&'a mut self, from: &'a str) -> &'a mut Filter {
        let addr: Address = from.parse().unwrap();
        let new = Rc::new(RefCell::new(Node {
            operand: Some(FilterKV {
                key: String::from("from"),
                value: addr.as_bytes().to_vec(),
            }),
            operator: None,
            nodes: None,
        }));

        match &mut self.root {
            // If there's a root already, append this op to `next`'s children
            Some(_) => {
                let mut next = self.next.as_ref().unwrap().borrow_mut();
                match &mut next.nodes {
                    Some(children) => {
                        children.push(new);
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(new);
                        next.nodes = Some(v);
                    }
                }
            }
            // If no root, create it and point next to it.
            None => {
                self.root = Some(new.clone());
                self.next = Some(new);
            }
        };
        self
    }

    pub fn method_id<'a>(&'a mut self, id: &'a str) -> &'a mut Filter {
        let method_id: Bytes = id.parse().unwrap();
        let new = Rc::new(RefCell::new(Node {
            operand: Some(FilterKV {
                key: String::from("method"),
                value: method_id.to_vec(),
            }),
            operator: None,
            nodes: None,
        }));

        match &mut self.root {
            // If there's a root already, append this op to `next`'s children
            Some(_) => {
                let mut next = self.next.as_ref().unwrap().borrow_mut();
                match &mut next.nodes {
                    Some(children) => {
                        children.push(new);
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(new);
                        next.nodes = Some(v);
                    }
                }
            }
            // If no root, create it and point next to it.
            None => {
                self.root = Some(new.clone());
                self.next = Some(new);
            }
        };
        self
    }

    pub fn value<'a>(&'a mut self, v: U256) -> &'a mut Filter {
        let bytes = from_u256(v);
        let new = Rc::new(RefCell::new(Node {
            operand: Some(FilterKV {
                key: String::from("value"),
                value: bytes,
            }),
            operator: None,
            nodes: None,
        }));

        match &mut self.root {
            // If there's a root already, append this op to `next`'s children
            Some(_) => {
                let mut next = self.next.as_ref().unwrap().borrow_mut();
                match &mut next.nodes {
                    Some(children) => {
                        children.push(new);
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(new);
                        next.nodes = Some(v);
                    }
                }
            }
            // If no root, create it and point next to it.
            None => {
                self.root = Some(new.clone());
                self.next = Some(new);
            }
        };
        self
    }

    // Creates and AND node and enters it (i.e. anything after this will be appended)
    // as a child of this node. A reference to the last node will be saved in `last`, and you
    // can re-enter that level using `exit()`.
    pub fn and<'a>(&'a mut self) -> &'a mut Filter {
        let new = Rc::new(RefCell::new(Node {
            operand: None,
            operator: Some(Operator::AND),
            nodes: None,
        }));

        match &mut self.root {
            Some(_) => {
                // If there's a root already, append this op to `next`'s children
                let next = self.next.as_ref().unwrap();
                let mut next_ptr = next.borrow_mut();
                match &mut next_ptr.nodes {
                    Some(children) => {
                        children.push(new.clone());
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(new.clone());
                        next_ptr.nodes = Some(v);
                    }
                }
            }
            // If no root, create it and point next to it.
            None => {
                self.root = Some(new.clone());
            }
        };

        self.last = self.next.clone();
        self.next = Some(new);
        self
    }

    // Creates and OR node and enters it (i.e. anything after this will be appended)
    // as a child of this node. A reference to the last node will be saved in `last`, and you
    // can re-enter that level using `exit()`.
    pub fn or<'a>(&'a mut self) -> &'a mut Filter {
        let new = Rc::new(RefCell::new(Node {
            operand: None,
            operator: Some(Operator::OR),
            nodes: None,
        }));

        match &mut self.root {
            Some(_) => {
                // If there's a root already, append this op to `next`'s children
                let next = self.next.as_ref().unwrap();
                let mut next_ptr = next.borrow_mut();
                match &mut next_ptr.nodes {
                    Some(children) => {
                        children.push(new.clone());
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(new.clone());
                        next_ptr.nodes = Some(v);
                    }
                }
            }

            // If no root, create it and point next to it.
            None => {
                self.root = Some(new.clone());
            }
        };

        self.last = self.next.clone();
        self.next = Some(new);
        self
    }

    /// next tells the builder to create a child at the current `next` pointer
    /// and move there.
    pub fn exit<'a>(&'a mut self) -> &'a mut Filter {
        self.next = self.last.clone();
        self
    }

    pub fn encode(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn encode_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn build(&self) -> Filter {
        self.to_owned()
    }
}

fn from_u256(u: U256) -> Vec<u8> {
    let mut hex = format!("{:x}", u);
    if hex.len() % 2 != 0 {
        hex = format!{"0{}", hex};
    }

    hex::decode(hex).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let val = U256::from(10000);
        let mut f = Filter::new();
        let new = f.value(val);
        // .and()
        // .to("0x7a250d5630B4cF539739dF2C5dAcb4c659F24ABC")
        // .to("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D")
        // .or()
        // .from("0x7a250d5630B4cF539739dF2C5dAcb4c659F24BCD")
        // .to("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D")
        // .exit()
        // .to("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
        println!("{}", new.encode_pretty().unwrap());
    }
}
