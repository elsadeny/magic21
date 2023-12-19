use std:: ops::Index;


#[derive(Debug)]
struct Nmagic {
    numbezi : Vec<Vec<i16>>
}

impl Nmagic {
    fn new() -> Nmagic {
        let ix : Vec<i16> = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21].to_vec();
        Nmagic{
            numbezi :  match_mix(ix),
        }
    }
    fn disp(&self) {
        for(i,item) in self.numbezi.iter().enumerate(){
            println!("GROUP {} = {:?}",i,item);
        }
    }
    
    fn process_choice(&mut self,choice:u8) {
        let vx = self.numbezi.clone();
    
        let vxn = match choice {
            1 => [vx.index(1).clone(),vx.index(0).clone(),vx.index(2).clone()].to_vec(),
            3 =>  [vx.index(1).clone(),vx.index(2).clone(),vx.index(0).clone()].to_vec(),
            _ => vx.to_vec()
        };
        self.numbezi = vxn;
    }

    fn magic_mix(&mut self) {
        let mut mx1:Vec<i16> = vec![];
        let mut mx2:Vec<i16> = vec![];
        let mut mx3:Vec<i16> = vec![];
        let i  : Vec<i16>= self.numbezi.clone().iter().flat_map(|inner|{
            inner.iter().cloned()
        }).collect();

        for(index,item) in i.iter().enumerate(){
            match (index+1)%3 {
                1 => [mx1.clone(),[item.to_owned()].to_vec()].concat().clone_into(&mut mx1),
                2 => [mx2.clone(),[item.to_owned()].to_vec()].concat().clone_into(&mut mx2),
                _ =>[mx3.clone(),[item.to_owned()].to_vec()].concat().clone_into(&mut mx3)
            }
        }
        self.numbezi = [mx1.to_owned(),mx2.to_owned(),mx3.to_owned()].to_vec();
    }
}


fn match_mix(sequence : Vec<i16>) -> Vec<Vec<i16>>{

    let mut mx1:Vec<i16> = vec![];
    let mut mx2:Vec<i16> = vec![];
    let mut mx3:Vec<i16> = vec![];

    let xp = sequence.clone();
        
    if sequence.clone().len() == 21 {
        let _ = xp.iter().for_each(|f|{
            match (f.to_owned())%3 {
                1 => [mx1.clone(),[f.to_owned()].to_vec()].concat().clone_into(&mut mx1),
                2 => [mx2.clone(),[f.to_owned()].to_vec()].concat().clone_into(&mut mx2),
                _ =>[mx3.clone(),[f.to_owned()].to_vec()].concat().clone_into(&mut mx3)
            }
        });
    } else{
        println!("vector size is not 21 instread is {}",sequence.clone().len())
    }

   return [mx1.to_owned(),mx2.to_owned(),mx3.to_owned()].to_vec();

    
}

fn main() {
    //let ix : Vec<i16> = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21].to_vec();
    println!("Think a number between 1 and 21 and  answer 3 few questions");
    println!("which side is the number located? A,B or C");
    let mut pi = Nmagic::new();
    
    pi.disp();

}
