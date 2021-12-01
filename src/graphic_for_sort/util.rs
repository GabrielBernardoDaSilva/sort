use std::marker;

use rand::Rng;
use super::bars::Bar;


#[derive(Default)]
pub struct ControlIndex{
    pub i: usize,
    pub j: usize
}

pub trait Shuffle {
    fn shuffle(&mut self);
}



impl Shuffle for Vec<Bar> {
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in (0..self.len()).rev() {
            let num = rng.gen_range(0..self.len());
            let aux_i = self[i];
            let aux_num = self[num];

            self[i] = self[num];
            self[i].pos_x = aux_i.pos_x;
            self[num] = aux_i;
            self[num].pos_x = aux_num.pos_x;
        }
    }
}





