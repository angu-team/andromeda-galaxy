
//


pub struct BytecodeUtils;

impl BytecodeUtils {

    pub fn erc20_essentials_id(&self) -> [&'static str; 5] {
        ["a9059cbb","70a08231","dd62ed3e","095ea7b3","23b872dd"]
    }

    pub fn bytecode_is_deploy_erc20(bytecode:String) -> bool{
        let essentials = BytecodeUtils.erc20_essentials_id();
        essentials.iter().all(|&id| bytecode.contains(id))
    }
}
