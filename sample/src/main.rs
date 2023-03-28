use rs_drivelist::drive_list;

fn main() {
    match drive_list()
    {
        Err(err)=>panic!("{}",err),
        Ok(drives)=>{
            for i in 0..drives.len() {
                println!("Drive {}. {}",i+1,json::stringify_pretty(&drives[i],b' ' as u16));
            }
        }
    }
}