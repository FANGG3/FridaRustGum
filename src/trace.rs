use crate::error::Error;
use std::collections::HashMap;
pub struct Trace{
    addr: u64,
    regs: HashMap<String,u64>
    
}
impl Trace{
    pub fn new(index:u64,addr:u64) ->Trace {
        let regs = HashMap::new();
        Trace{
            addr,
            regs
        }
    }
    // pub fn from_record(line:&str) -> Result<Trace,Error>{
    //     let tokens = line.split(",");
    //     // let c = tokens.collect();

    //     let e = tokens.enumerate();
    //     for (i,item) in e{
    //         println!("begin");
    //         println!("{} {}",i,item);
    //         println!("over");

    //     }
    //     println!("do over");
    //     Err(Error::ParserError(line.to_owned()))
    // }

    


    pub fn to_str(&self)->String{
        
        return format!("addr:{:x} reg:{:?}", self.addr,self.regs);
    }

    


}