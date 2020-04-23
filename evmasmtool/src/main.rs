use std::collections::HashMap;

struct EVMCodeGen {
    code : Vec<u8>,
    labels: HashMap<String,usize>,
    pending_labels : HashMap<String,Vec<usize>>,
}

impl EVMCodeGen {
    pub fn new() -> Self {
        Self {
            code : Vec::new(),
            labels: HashMap::new(),
            pending_labels: HashMap::new(),
        }
    }

    pub fn op_stop(&mut self) {
        self.code.push(0x00);
    }
    pub fn op_add(&mut self)  { self.code.push(0x01); }
    pub fn op_mul(&mut self)  { self.code.push(0x02); }
    pub fn op_sub(&mut self)  { self.code.push(0x03); }
    pub fn op_div(&mut self)  { self.code.push(0x04); }
    pub fn op_sdiv(&mut self) { self.code.push(0x05); }
    pub fn op_mod(&mut self)  { self.code.push(0x06); }
    pub fn op_smod(&mut self)  { self.code.push(0x07); }
    pub fn op_addmod(&mut self)  { self.code.push(0x08); }
    pub fn op_mulmod(&mut self)  { self.code.push(0x09); }
    pub fn op_exp(&mut self)  { self.code.push(0x0a); }
    pub fn op_signextend(&mut self)  { self.code.push(0x0b); }

    pub fn op_lt(&mut self)   { self.code.push(0x10); }
    pub fn op_gt(&mut self)   { self.code.push(0x11); }
    pub fn op_slt(&mut self)  { self.code.push(0x12); }
    pub fn op_sgt(&mut self)  { self.code.push(0x13); }
    pub fn op_eq(&mut self)   { self.code.push(0x14); }
    pub fn op_iszero(&mut self)  { self.code.push(0x15); }
    pub fn op_and(&mut self)  { self.code.push(0x16); }
    pub fn op_or(&mut self)  { self.code.push(0x17); }
    pub fn op_xor(&mut self)  { self.code.push(0x18); }
    pub fn op_not(&mut self)  { self.code.push(0x19); }
    pub fn op_byte(&mut self)  { self.code.push(0x1a); }

    pub fn op_keccak(&mut self) { self.code.push(0x20); }
    pub fn op_sha3(&mut self) { self.code.push(0x20); }   // alias

    pub fn op_address(&mut self)  { self.code.push(0x30); }
    pub fn op_balance(&mut self)  { self.code.push(0x31); }
    pub fn op_origin(&mut self)  { self.code.push(0x32); }
    pub fn op_caller(&mut self)  { self.code.push(0x33); }
    pub fn op_callvalue(&mut self)  { self.code.push(0x34); }
    pub fn op_calldataload(&mut self)  { self.code.push(0x35); }
    pub fn op_calldatasize(&mut self)  { self.code.push(0x36); }
    pub fn op_calldatacopy(&mut self)  { self.code.push(0x37); }
    pub fn op_codesize(&mut self)  { self.code.push(0x38); }
    pub fn op_codecopy(&mut self)  { self.code.push(0x39); }
    pub fn op_gasprice(&mut self)  { self.code.push(0x3a); }
    pub fn op_extcodesize(&mut self)  { self.code.push(0x3b); }
    pub fn op_extcodecopy(&mut self)  { self.code.push(0x3c); }
    pub fn op_returndatasize(&mut self)  { self.code.push(0x3d); }
    pub fn op_returndatacopy(&mut self)  { self.code.push(0x3e); }

    pub fn op_blockhash(&mut self)  { self.code.push(0x40); }
    pub fn op_coinbase(&mut self)  { self.code.push(0x41); }
    pub fn op_timestamp(&mut self)  { self.code.push(0x42); }
    pub fn op_number(&mut self)  { self.code.push(0x43); }
    pub fn op_difficulty(&mut self)  { self.code.push(0x44); }
    pub fn op_gaslimit(&mut self)  { self.code.push(0x45); }

    pub fn op_pop(&mut self)  { self.code.push(0x50); }
    pub fn op_mload(&mut self)  { self.code.push(0x51); }
    pub fn op_mstore(&mut self)  { self.code.push(0x52); }
    pub fn op_mstore8(&mut self)  { self.code.push(0x53); }
    pub fn op_sload(&mut self)  { self.code.push(0x54); }
    pub fn op_sstore(&mut self)  { self.code.push(0x55); }

    pub fn op_jump(&mut self, label: &str)  {
        let offset = self.ref_label(label,1);
        self.op_push(&offset);
        self.code.push(0x56);
    }

    pub fn op_jumpi(&mut self, label: &str)  {
        let offset = self.ref_label(label,1);
        self.op_push(&offset);
        self.code.push(0x57);
    }

    pub fn op_pc(&mut self)  { self.code.push(0x58); }
    pub fn op_msize(&mut self)  { self.code.push(0x59); }
    pub fn op_gas(&mut self)  { self.code.push(0x5a); }

    pub fn op_jumpdest(&mut self, name: &str)  {
        if self.labels.contains_key(name) {
            panic!("Label already defined");
        }
        self.labels.insert(name.to_string(),self.code.len());
        self.code.push(0x5b);

        self.fill_label(name);
    }

    pub fn op_beginsub(&mut self, name: &str)  {
        if self.labels.contains_key(name) {
            panic!("Label already defined");
        }
        self.labels.insert(name.to_string(),self.code.len());
        self.code.push(0xb2);

        self.fill_label(name);
    }

    pub fn op_jumpsub(&mut self, label: &str)  {
        let offset = self.ref_label(label,1);
        self.op_push(&offset);
        self.code.push(0xb3);
    }

    pub fn op_returnsub(&mut self)  {
        self.code.push(0xb7);
    }

    pub fn op_push(&mut self, data: &[u8]) { // push n-bytes item into the stack
        if data.len() == 0 || data.len() > 32 {
            panic!("bad push");
        }
        self.code.push(0x5f + data.len() as u8);
        self.code.extend_from_slice(data);
    }

    pub fn op_dup(&mut self, n: u8) {
        if n > 16 {
            panic!("Assertion failed");
        }
        self.code.push(0x80 + n - 1);
    }

    pub fn op_swap(&mut self, n: u8) {
        if n > 16 {
            panic!("Assertion failed");
        }
        self.code.push(0x8f + n);
    }

    pub fn op_log0(&mut self)  { self.code.push(0xa0); }
    pub fn op_log1(&mut self)  { self.code.push(0xa1); }
    pub fn op_log2(&mut self)  { self.code.push(0xa2); }
    pub fn op_log3(&mut self)  { self.code.push(0xa3); }
    pub fn op_log4(&mut self)  { self.code.push(0xa4); }

    pub fn op_create(&mut self)  { self.code.push(0xf0); }
    pub fn op_call(&mut self)  { self.code.push(0xf1); }
    pub fn op_callcode(&mut self)  { self.code.push(0xf2); }
    pub fn op_return(&mut self)  { self.code.push(0xf3); }
    pub fn op_delegatecall(&mut self)  { self.code.push(0xf4); }

    pub fn op_staticcall(&mut self)  { self.code.push(0xfa); }
    pub fn op_revert(&mut self)  { self.code.push(0xfd); }
    pub fn op_invalid(&mut self)  { self.code.push(0xfe); }
    pub fn op_selfdestruct(&mut self)  { self.code.push(0xff); }

    pub fn push_label(&mut self,label:&str) {
        self.code.push(0x5b);
        let offset = self.ref_label(label,0);
        self.code.extend_from_slice(&offset);
    }

    pub fn push_data_segment(&mut self,label:&str) {
        self.code.push(0x5f+3);
        let offset = self.ref_label(label,0);
        self.code.extend_from_slice(&offset);
    }

    pub fn data_segment(&mut self,label:&str,data:&[u8]) {
        self.labels.insert(label.to_string(),self.code.len());
        self.fill_label(label);
        self.code.extend_from_slice(data);
    }

    fn ref_label(&mut self, label: &str, offset: usize) -> Vec<u8>{
        if let Some(loc) = self.labels.get(label) {
            let loc_bytes = if *loc == 0 {
                vec![0u8]
            } else {
                loc.to_be_bytes().to_vec().iter().skip_while(|n|*n==&0u8).cloned().collect::<Vec<_>>()
            };
            loc_bytes
        } else {
            self.pending_labels.entry(label.to_string()).or_insert(Vec::new()).push(self.code.len()+offset);
            vec![0,0,0]
        }
    }

    fn fill_label(&mut self, label : &str) {
        if let Some(locations) = self.pending_labels.remove(label) {
            let dst = self.labels[label];
            let (dst0,dst1,dst2) = ((dst>>16) as u8,((dst>>8)&0xff) as u8,(dst&0xff) as u8);
            locations.iter().for_each(|loc| {
                println!("LOC={}",loc);
                self.code[loc+0]=dst0;
                self.code[loc+1]=dst1;
                self.code[loc+2]=dst2;
            });
        }
    }

    fn gen_code(&self) -> &Vec<u8> {
       if self.pending_labels.len() > 0 {
            let list = self.pending_labels.keys().collect::<Vec<_>>();
            panic!(format!("pending_labels {:?}",list));
        }
        &self.code
    }

    fn gen_tx_code(&self) -> Vec<u8> {
        let code = self.gen_code();

        let mut tx = EVMCodeGen::new();

        // reserve data
        tx.op_push(&[0x60]);
        tx.op_push(&[0x40]);
        tx.op_mstore();

        // copy contract code to memory [] -> []
        tx.op_push(&[0]);                   // mem destOffset
        tx.push_data_segment("code");  // code offset
        tx.op_push(&[code.len() as u8]);    // code length
        tx.op_codecopy();

        // return memory offset + len
        tx.op_push(&[0]);                   // mem destOffset
        tx.op_push(&[code.len() as u8]);    // code length
        tx.op_return();

        tx.op_stop();

        tx.data_segment("code", code);

        tx.gen_code().clone()
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    fn unhex(data:&str) -> Vec<u8> {
        hex::decode(data).unwrap()
    }

    fn assert_code_eq(evmb: &EVMCodeGen, input: &str) {
        let mut trimmed = String::with_capacity(input.len());
        for l in input.split('\n') {
            l.chars()
                .take_while(|ch| *ch!='#')
                .filter(|ch| *ch!=' ')
                .for_each(|ch| trimmed.push(ch));
        }
        assert_eq!(hex::encode(evmb.gen_code()),trimmed);
    } 

    #[test]
    fn test_simple() {
        let mut b = EVMCodeGen::new();
        b.op_push(&[1]);
        b.op_push(&[3]);
        b.op_dup(2);
        b.op_stop();

        assert_code_eq(&b,"
            60 01  # PUSH1 01
            60 03  # PUSH1 01
            81     # DUP2
            00     # STOP
        ");
    }

    #[test]
    fn test_call_a() {
        let mut b = EVMCodeGen::new();
        b.op_jumpdest("loop");
        b.op_push(&[0x00]);       // retLen
        b.op_push(&[0x00]);       // retOffset
        b.op_push(&[0x00]);       // argsLen
        b.op_push(&[0x00]);       // argsOffset
        b.op_push(&[0x00]);       // value
        b.op_push(&[0xff,0x0b]);  // addr
        b.op_gas();                     // gas
        b.op_call();
        b.op_jump("loop");

        assert_code_eq(&b,"5b 6000 6000 6000 6000 6000 61ff0b 5a f1 60 00 56");
    }

    #[test]
    fn test_call_recursive() {
        let mut b = EVMCodeGen::new();
        b.op_pc();       // retLen
        b.op_dup(1);     // retOffset
        b.op_dup(1);     // argsLen
        b.op_dup(1);     // argsOffset
        b.op_dup(1);     // value
        b.op_address();  // addr
        b.op_gas();      // gas
        b.op_call();

        assert_code_eq(&b,"5880808080305af1");
    }

    #[test]
    fn test_call_recursive_tx() {
        let mut b = EVMCodeGen::new();
        b.op_pc();       // retLen
        b.op_dup(1);     // retOffset
        b.op_dup(1);     // argsLen
        b.op_dup(1);     // argsOffset
        b.op_dup(1);     // value
        b.op_address();  // addr
        b.op_gas();      // gas
        b.op_call();

        assert_code_eq(&b,"5880808080305af1");
    }
    

    #[test]
    fn test_call_precompile() {
        let mut b = EVMCodeGen::new();
        b.op_pc();       // retLen
        b.op_dup(1);     // retOffset
        b.op_dup(1);     // argsLen
        b.op_dup(1);     // argsOffset
        b.op_dup(1);     // value
        b.op_push(&[0xff,0x0a]);  // addr
        b.op_gas();      // gas
        b.op_call();
        println!("{}",hex::encode(b.gen_code()));
    }

    #[test]
    fn test_call_data_segment() {
        let mut b = EVMCodeGen::new();
        b.push_data_segment("data1");
        b.op_stop();
        b.data_segment("data1",&vec![1,2,4]);
        assert_code_eq(&b,"62 0000050 00 10204");
    }
    #[test]
    fn test_test1() {
        let contract = unhex("005880808080305af1");
    
        let mut b = EVMCodeGen::new();
        
        // copy contract code to memory [] -> []
        b.op_push(&[contract.len() as u8]); // code length
        b.push_data_segment("contract"); // code offset
        b.op_push(&[0]); // mem destOffset
        b.op_codecopy();
        
        // create contract [] -> [addr]
        b.op_push(&[contract.len() as u8]); // length
        b.op_push(&[0]); // mem offset
        b.op_push(&[0]); // value
        b.op_create();
        
        // store contract address to memory[0..31] [addr] -> []
        b.op_push(&[0]); // [offset=0,value]
        b.op_mstore();

        // loop call contrats
        b.op_jumpdest("loop");
        b.op_push(&[0x00]);       // retLen
        b.op_push(&[0x00]);       // retOffset
        b.op_push(&[0x00]);       // argsLen
        b.op_push(&[0x00]);       // argsOffset
        b.op_push(&[0x00]);       // value
        b.op_push(&[0x00]);       // mload contract addr
          b.op_mload();
        b.op_gas();                     // gas
        b.op_call();
        b.op_jump("loop");

        // define recursive contract        
        b.data_segment("contract",&contract);
        assert_eq!(hex::encode(b.gen_code()),"600962000026600039600960006000f06000525b600060006000600060006000515af1601356005880808080305af1");
    }
    #[test]
    fn test_jump() {
        let mut b = EVMCodeGen::new();
        b.op_jump("j");
        b.op_jumpdest("j");
        assert_code_eq(&b,"62 00 00 05 56 5b");
    }

    #[test]
    fn test_subs() {
        let mut b = EVMCodeGen::new();
        b.op_jumpsub("sub");
        b.op_stop();
        b.op_beginsub("sub");
        b.op_returnsub();
        assert_code_eq(&b,"62000006 b3 00 b2 b7");
    }

    #[test]
    fn test_recursive() {
        let mut b = EVMCodeGen::new();
        b.op_push(&[0x03,0xFF]); // counter
        b.op_beginsub("sub");
        b.op_dup(1);
        b.op_jumpi("continue");
        b.op_stop();
        b.op_jumpdest("continue");
        b.op_push(&[1]);
        b.op_swap(1);
        b.op_sub();
        b.op_jumpsub("sub");
        assert_code_eq(&b,"6103ffb2806200000b57005b600190036003b3");
    }

    #[test]
    fn test_retrunto_end() {
        let mut b = EVMCodeGen::new();
        b.op_jump("end");
        b.op_beginsub("sub");
        b.op_returnsub();
        b.op_jumpdest("end");
        b.op_jumpsub("sub");
        assert_code_eq(&b,"6103ffb280600b57005b600190036003b31");
    }

}
