use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut guess: u32 = 0;
        let mut low=min;
        let mut high=max;
        
        for num in min..=max{
            let mut guess = low +(high-low)/2; //Binary search formula
            let mut guessing= player.ask_to_compare(guess);
            
            if guessing == 0{
            return guess;
            }
            else if guessing== -1{
            high =guess+ 1;
            }
        if guessing == 1{ 
            low= guess-1; 
        }

        }
        panic!("Why you lying?! :(");
    }
 }

