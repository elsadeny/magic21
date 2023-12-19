pub mod test;

use std::{io,ops::Index};
use rand::Rng;

#[derive(Debug)]
struct Magic21 {
    numbers_21 : Vec<Vec<i16>>
}

impl Magic21 {
    fn new() -> Magic21 {
        // Create a vector to store the random numbers
        let mut random_numbers: Vec<Vec<i16>> = Vec::new();

         // Generate 21 random numbers and push them into the vector
        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let inner_vec: Vec<i16> = (0..7).map(|_| rng.gen_range(0..100)).collect();
            random_numbers.push(inner_vec);
        }
        Magic21{
            numbers_21 : random_numbers,
        }
    }
    fn display(&self) {
        for(i,item) in self.numbers_21.iter().enumerate(){
            println!("GROUP {} = {:?}",(i+1),item);
        }
    }
    
    fn process_choice(&mut self,choice:u8) {
        let vx = self.numbers_21.clone();
    
        let vxn = match choice {
            1 => [vx.index(1).clone(),vx.index(0).clone(),vx.index(2).clone()].to_vec(),
            3 =>  [vx.index(0).clone(),vx.index(2).clone(),vx.index(1).clone()].to_vec(),
            _ => vx.to_vec()
        };
        self.numbers_21 = vxn;
    }

    fn magic_shuffle(&mut self) {
        let mut mx1:Vec<i16> = vec![];
        let mut mx2:Vec<i16> = vec![];
        let mut mx3:Vec<i16> = vec![];
        let i  : Vec<i16>= self.numbers_21.clone().iter().flat_map(|inner|{
            inner.iter().cloned()
        }).collect();

        for(index,item) in i.iter().enumerate(){
            match (index+1)%3 {
                1 => [mx1.clone(),[item.to_owned()].to_vec()].concat().clone_into(&mut mx1),
                2 => [mx2.clone(),[item.to_owned()].to_vec()].concat().clone_into(&mut mx2),
                _ =>[mx3.clone(),[item.to_owned()].to_vec()].concat().clone_into(&mut mx3)
            }
        }
        self.numbers_21 = [mx1.to_owned(),mx2.to_owned(),mx3.to_owned()].to_vec();
    }
}

fn main() {
    println!("Think of a number between 1 and 21 and  answer 3 few questions");
    println!("");
    let mut magic21 = Magic21::new();
    (0..=2).for_each(|_|{
        magic21.display();
        println!("Which Group is the number located? 1,2 or 3");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("could not read your choice");
        if let Ok(choice) = input.trim().parse::<u8>(){
            magic21.process_choice(choice)
        }else{
            println!("\nplease select numbers only");
        }
        magic21.magic_shuffle();
    });

    println!("WoWoo the magic number you are thinking is {}",magic21.numbers_21.index(1).index(3));

}
