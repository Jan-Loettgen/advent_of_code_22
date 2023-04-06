
struct Dir<'a>{
    dirs: Vec<Dir>,
    files: Vec<u32>,
    dir_index: usize,
    dir_size: u32,
    parent: Option<& Dir>,
}

impl Dir {
    fn new () -> Dir{
        Dir{ dirs : Vec::new(), files : Vec::new(), dir_index: 0, dir_size: 0, parent: None}
    }
}


pub fn fun(){
    print!("test");
    let dir = Dir::new();
}