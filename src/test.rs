//src/test.rs
#[cfg(test)]
mod tests{
use std::ops::Index;
use crate::Magic21;

    #[test]
    fn test_new_magic21(){
        let magic21 = Magic21::new();
        assert_eq!(magic21.numbers_21, [[1, 4, 7, 10, 13, 16, 19],[2, 5, 8, 11, 14, 17, 20], [3, 6, 9, 12, 15, 18, 21]].to_vec())
    }

    #[test]
    fn test_process_choice(){
        let mut magic21 = Magic21::new();
        magic21.process_choice(3);
        assert_eq!(magic21.numbers_21,[[1, 4, 7, 10, 13, 16, 19],[3, 6, 9, 12, 15, 18, 21],[2, 5, 8, 11, 14, 17, 20]].to_vec())
    }

    #[test]
    fn test_magic_mix(){
        let mut magic21 = Magic21::new();
        magic21.process_choice(3);
        assert_eq!(magic21.numbers_21,[[1, 4, 7, 10, 13, 16, 19],[3, 6, 9, 12, 15, 18, 21],[2, 5, 8, 11, 14, 17, 20]].to_vec());
        magic21.magic_shuffle();
        assert_eq!(magic21.numbers_21.index(0).to_owned(),[1, 10, 19, 9, 18, 5, 14].to_vec());
    
       }
}