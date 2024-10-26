use std::cmp::min;
use std::io::{Error,Read};
use std::fs::File;
#[derive(Default, Clone, Copy)]
pub struct PresentsParams {
    pub size: i64,
    pub length: i64,
}
#[derive(Default)]
pub struct NoMath {
    pub data_content: String,
    pub presents_params: PresentsParams,
}
impl NoMath {
    pub fn solve(&mut self) -> Result<PresentsParams, Error>{
        Self::read_file(self)?;
        for line in self.data_content.lines() {
            let mut dims = line.split('x');
            let temp: PresentsParams = Self::formula(
                dims.next().unwrap().parse().unwrap(),
                dims.next().unwrap().parse().unwrap(),
                dims.next().unwrap().parse().unwrap());
            self.presents_params.size += temp.size;
            self.presents_params.length += temp.length;
        }
        Ok(self.presents_params)
    }    
    fn read_file(&mut self) -> Result<(), Error> {
        let mut data_file = File::open("files/list_presents.txt")?;
        data_file.read_to_string(&mut self.data_content)?;
        Ok(())
    }
    fn formula(l: i64, w: i64, h: i64) -> PresentsParams {
        let size = (2*l*w) + (2*w*h) + (2*h*l) + Self::extra_space(l, w, h);
        let length = Self::ribbon_length(l, w, h);
        PresentsParams{size, length}
    }
    fn extra_space(l: i64, w: i64, h: i64) -> i64 {
        min(l*w, min(w*h, h*l))
    }
    fn ribbon_length(l: i64, w: i64, h: i64) -> i64 {
        let additional = l * w * h;
        let length = (2 * min(l, min(w, h))) + (2 * Self::find_middle(l,w,h));
        additional + length
    }
    fn find_middle(l: i64, w: i64, h: i64) -> i64 {
        let res: i64;
        if (l >= w && w >= h) || (l <= w && w <= h) {
            res = w;
        }
        else if (w >= l && l >= h) || (w <= l && l <= h){
            res = l;
        }
        else {
            res = h;
        }
        res
    }

}