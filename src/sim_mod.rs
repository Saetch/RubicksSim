

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

                    match &dir{
                        TurnTypes::FrontLeft { back_flip: _ } => println!("FRONTLEFT"),
                        TurnTypes::FrontRight { back_flip: _ } =>  println!("FRONTRIGHT"),
                        TurnTypes::LeftSideLeft { back_flip: _ } =>  println!("LEFTLEFT"),
                        TurnTypes::LeftSideRight { back_flip: _ } =>  println!("LEFTRIGHT"),
                        TurnTypes::TopPlatform { back_flip: _ } =>  println!("TOPLEFT"),
                        TurnTypes::BottomPlatform { back_flip : _ } => println!("TOPRIGHT"),
                    }
                    self.cube.turn_mut(&dir);
                }
                
            }
        }

        pub fn turn_mut_pip(&mut self, ty : &TurnTypes){
            self.cube.turn_mut(&ty);
        }
    }


