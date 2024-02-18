use crate::{Address, Hashable};

#[derive(Debug)]
pub struct Transaction {
    pub outputs: Vec<Output>,
    pub inputs: Vec<Output>,
}

impl Transaction {}

#[derive(Clone, Debug)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        // TODO: why not &self? / how does it work vec extend / how does it work as_bytes / what different between as_bytes and bytes.
        bytes.extend(self.to_addr.as_bytes());
        // bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        // Vec<Output> to Vec<v8>
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}
