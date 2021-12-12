

use rand::Rng;
use crate::r_cube_mod::{RCube as RCube, TurnTypes};

        pub struct  Sim{
        
        pub cube : RCube,


    }

    impl Sim{
        pub fn new() -> Self{
            Sim {
                cube : RCube::new()
            }
        }

        pub fn show_cube_status(&self) -> String{
            return self.cube.to_string();
        }

        pub fn randomize_cube_mut(&mut self, i : i32){
            let mut r = rand::thread_rng();
            for count in 0..i{
                let am = count %2 +1;
                let dir ;
                match  r.gen::<u8>()%6{
                    0 => dir = TurnTypes::FrontLeft{ back_flip : true},
                    1 => dir = TurnTypes::FrontRight{ back_flip: true},
                    2 => dir = TurnTypes::LeftSideLeft{back_flip:true},
                    3 => dir = TurnTypes::LeftSideRight{back_flip:true},   
                    4 => dir = TurnTypes::TopPlatform{back_flip:true},
                    5 => dir = TurnTypes::BottomPlatform{back_flip:true},
                    _ => dir = TurnTypes::FrontLeft{back_flip:false}
                    
                }
                match &dir {
                    TurnTypes::FrontLeft { back_flip } => if *back_flip {()} else { println!("WRONG VALUE CALCULATED FOR TURNTYPE IN RANDOMIZE_CUBE\n"); return},
                    _ => (),
                }



                for _d in 0..am{
                    self.cube.turn_mut(&dir);
                }
                
            }
        }

        pub fn turn_mut_pip(&mut self, ty : &TurnTypes){
            self.cube.turn_mut(&ty);
        }

        pub fn is_solved(&self) -> bool{
            self.cube.is_solved()
        }

        pub fn random_solve_prnt(&mut self) -> u64{
            let mut ret = 0;
            let mut r = rand::thread_rng();
            while !self.is_solved() {
                match r.gen::<u8>() %12 {
                    0 => self.turn_mut_pip(&TurnTypes::FrontLeft{ back_flip : true}),
                    1 => self.turn_mut_pip(&TurnTypes::FrontLeft{ back_flip : false}),
                    2 => self.turn_mut_pip(&TurnTypes::LeftSideLeft{ back_flip : true}),
                    3 => self.turn_mut_pip(&TurnTypes::LeftSideLeft{ back_flip : false}),
                    4 => self.turn_mut_pip(&TurnTypes::FrontRight{ back_flip : true}),
                    5 => self.turn_mut_pip(&TurnTypes::FrontRight{ back_flip : false}),
                    6 => self.turn_mut_pip(&TurnTypes::LeftSideRight{ back_flip : true}),
                    7 => self.turn_mut_pip(&TurnTypes::LeftSideRight{ back_flip : false}),
                    8 => self.turn_mut_pip(&TurnTypes::TopPlatform{ back_flip : true}),
                    9 => self.turn_mut_pip(&TurnTypes::TopPlatform{ back_flip : false}),
                    10 => self.turn_mut_pip(&TurnTypes::BottomPlatform{ back_flip : true}),
                    11 => self.turn_mut_pip(&TurnTypes::BottomPlatform{ back_flip : false}),

                    _ => ()
                }

                ret+=1;
                if ret % 10000000000 == 0 {
                    println!("Did {} moves so far!", ret);

                }
            }
            return ret;
        }

        pub fn random_solve(&mut self) -> u128{
            let mut ret = 0;
            let mut r = rand::thread_rng();
            while !self.is_solved() {
                match r.gen::<u8>() %12 {
                    0 => self.turn_mut_pip(&TurnTypes::FrontLeft{ back_flip : true}),
                    1 => self.turn_mut_pip(&TurnTypes::FrontLeft{ back_flip : false}),
                    2 => self.turn_mut_pip(&TurnTypes::LeftSideLeft{ back_flip : true}),
                    3 => self.turn_mut_pip(&TurnTypes::LeftSideLeft{ back_flip : false}),
                    4 => self.turn_mut_pip(&TurnTypes::FrontRight{ back_flip : true}),
                    5 => self.turn_mut_pip(&TurnTypes::FrontRight{ back_flip : false}),
                    6 => self.turn_mut_pip(&TurnTypes::LeftSideRight{ back_flip : true}),
                    7 => self.turn_mut_pip(&TurnTypes::LeftSideRight{ back_flip : false}),
                    8 => self.turn_mut_pip(&TurnTypes::TopPlatform{ back_flip : true}),
                    9 => self.turn_mut_pip(&TurnTypes::TopPlatform{ back_flip : false}),
                    10 => self.turn_mut_pip(&TurnTypes::BottomPlatform{ back_flip : true}),
                    11 => self.turn_mut_pip(&TurnTypes::BottomPlatform{ back_flip : false}),

                    _ => ()
                }

                ret+=1;

            }
            return ret;
        }
    }


