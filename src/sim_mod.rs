


use std::vec;

use rand::Rng;
use crate::{r_cube_mod::{RCube as RCube, TurnTypes, }, u,up,d,dp,f,fp,b,bp,l,lp,r,rp};

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
            let mut max = 0;
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
                if self.cube.get_solved_amount_direct() > max{
                    max = self.cube.get_solved_amount_direct();
                    println!("new maximum found: {}", max);
                }
                

                ret+=1;
                if ret % 1000000000 == 0 {
                    println!("Did {} moves so far! Most solved parts were: {}", ret, max);

                }
            }
            return ret;
        }

        pub fn solve_with_steps_skip(&mut self, steps : u32, skip : u32) -> u32{
            let start_c = self.cube.clone();
            let mut tried : u32= 0;
            let dummy_m = vec![u!(), up!(), d!(), dp!(), f!(), fp!(),b!(),bp!(),l!(),lp!(),r!(), rp!()];
            let mut max =self.cube.get_solved_amount_top_first();
            let mut vec_moves : Vec<TurnTypes> = Vec::new();
            //this is so loops can be eliminated
            let mut vec_states : Vec<RCube> = vec![self.cube.clone()];
            //let mut wanted_moves;
            let mut found ;
            while !self.is_solved(){
                    found =false;
                    //wanted_moves = self.get_moves_favorable(&dummy_m);
                    for mv in self.get_moves_favorable(&dummy_m){
                        let hypo_cube = self.cube.hypo_move(mv);
                        if !vec_states.contains(&hypo_cube)  {
                            vec_states.push(hypo_cube);
                            tried +=1;
                            self.cube=hypo_cube;
                            found = true;
                            vec_moves.push(mv);
                            break;
                        }
                    }
                    if !found || &vec_moves.len()> &(steps as usize){
                        //println!("Moving 1 step up! Chained {} moves so far!", vec_moves.len());
                        let du = &vec_moves.pop();
                        if du.is_none(){
                            self.cube = start_c;
                            println!("Going one step deeper. Tried all options with {} steps",steps);
                            return self.solve_with_steps(steps+skip);
                        }else{
                            self.cube.turn_mut(&du.unwrap().switch_direction());
                        }
                    }

                    if self.cube.get_solved_amount_top_first()> max{
                        max = self.cube.get_solved_amount_top_first();
                        println!("max solved parts: {}", max);
                    }    
                    tried +=1;                
            }
            println!("Tried {} moves in this function call!",tried);
            return vec_moves.len() as u32;
        }
        pub fn solve_with_steps(&mut self, steps : u32) -> u32{
            return self.solve_with_steps_skip(steps, 1);
        }

        pub fn random_solve(&mut self) -> u128{
            let mut ret = 0;
            let mut max = 0;
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
                if self.cube.get_solved_amount_direct() > max{
                    max = self.cube.get_solved_amount_direct();
                    println!("new maximum found: {}", max);
                }
                ret+=1;

            }
            return ret;
        }

        pub fn default_cube(&mut self)->bool{
            if !self.is_solved(){
                return false;
            }
            let mut vec : Vec::<TurnTypes> = Vec::new();
            //this is a default scramble according to a rubik's cube rule book
            vec.push(TurnTypes::BottomPlatform{ back_flip : true});
            vec.push(TurnTypes::FrontRight{ back_flip : true});
            vec.push(TurnTypes::FrontLeft{ back_flip : false});
            vec.push(TurnTypes::LeftSideRight{ back_flip : true});
            vec.push(TurnTypes::FrontRight{ back_flip : false});
            vec.push(TurnTypes::FrontLeft{ back_flip : false});
            vec.push(TurnTypes::TopPlatform{ back_flip : true});
            vec.push(TurnTypes::TopPlatform{ back_flip : true});
            vec.push(TurnTypes::LeftSideRight{ back_flip : true});//
            vec.push(TurnTypes::BottomPlatform{ back_flip : true});
            vec.push(TurnTypes::BottomPlatform{ back_flip : true});
            vec.push(TurnTypes::FrontRight{ back_flip : false});
            vec.push(TurnTypes::FrontLeft{ back_flip : true});
            vec.push(TurnTypes::FrontLeft{ back_flip : true});
            vec.push(TurnTypes::LeftSideRight{ back_flip : true});
            vec.push(TurnTypes::LeftSideRight{ back_flip : true});
            vec.push(TurnTypes::BottomPlatform{ back_flip : false});
            vec.push(TurnTypes::FrontRight{ back_flip : true});
            vec.push(TurnTypes::FrontRight{ back_flip : true});
            vec.push(TurnTypes::LeftSideLeft{ back_flip : true});
            vec.push(TurnTypes::LeftSideLeft{ back_flip : true});
            vec.push(TurnTypes::BottomPlatform{ back_flip : true});
            vec.push(TurnTypes::BottomPlatform{ back_flip : true});
            vec.push(TurnTypes::FrontLeft{ back_flip : true});
            vec.push(TurnTypes::FrontLeft{ back_flip : true});
            vec.push(TurnTypes::TopPlatform{ back_flip : false});
            vec.push(TurnTypes::LeftSideLeft{ back_flip : true});
            vec.push(TurnTypes::LeftSideLeft{ back_flip : true});
            vec.push(TurnTypes::TopPlatform{ back_flip : false});




            for t_type in vec  {
                self.cube.turn_mut(&t_type);
            }

            return true;
        }

        pub fn _get_solved(&self) -> u8{
            return self.cube.get_solved_amount_direct();
        }

        pub fn _other_simple_heuristic_solve(&mut self, modifier : f32)-> u32{
            if modifier < 12.0 {
                return 2 as u32;
            }
            return 0 as u32;
        }

        //THIS is supposed to be called with 12 different Turntypes, always. Otherwise the .unwrap()s can panick
        pub fn get_moves_favorable(&self, possible_moves : &Vec<TurnTypes>) -> Vec<TurnTypes>{
            let mut retete = Vec::new();
            let mut m : Vec<(TurnTypes, u16)> = Vec::new();
            for mov in possible_moves{
                m.push((mov.clone(), self.cube.clone().turn_mut_ret(&mov).get_solved_amount_top_first() ));
            }
            let mut f;
            while m.len()>0{
                f = m.iter().reduce(|x, y| if x.1 > y.1 { x} else {y}).unwrap();
                retete.push(f.0);
                let index =m.iter().position(|y|y.0 == f.0).unwrap();
                m.swap_remove(index);
            }
            return retete;
        }

        pub fn simple_heuristic_solve(&mut self) -> u32{
            let dummy_m = vec![u!(), up!(), d!(), dp!(), f!(), fp!(),b!(),bp!(),l!(),lp!(),r!(), rp!()];
            let mut max =self.cube.get_solved_amount_top_first();
            let mut vec_moves : Vec<TurnTypes> = Vec::new();
            //this is so loops can be eliminated
            let mut vec_states : Vec<RCube> = vec![self.cube.clone()];
            let mut wanted_moves;
            let mut found ;
            let mut search_heur_val = max;
            let mut compare_val = 0;
            let mut compare = true;
            while !self.is_solved(){
                    search_heur_val -=1;
                    found =false;
                    wanted_moves = self.get_moves_favorable(&dummy_m);
                    for mv in wanted_moves{
                        let hypo_cube = self.cube.hypo_move(mv);
                        if !vec_states.contains(&hypo_cube) && hypo_cube.get_solved_amount_top_first() >= search_heur_val{
                            vec_states.push(hypo_cube);
                            compare=true;
                            self.cube=hypo_cube;
                            found = true;
                            vec_moves.push(mv);
                            search_heur_val = hypo_cube.get_solved_amount_top_first();
                            break;
                        }
                    }
                    if compare && max > 700{
                        compare_val = 500;
                    }
                    if compare && max > 900{
                        compare_val = 0;
                    }
                    if !found && search_heur_val<= compare_val{
                        //println!("Moving 1 step up! Chained {} moves so far!", vec_moves.len());
                        let du = &vec_moves.pop();
                        if du.is_none(){
                            compare = false;
                        }else{
                            self.cube.turn_mut(&du.unwrap().switch_direction());
                        }
                    }
                    if self.cube.get_solved_amount_top_first()> max{
                        max = self.cube.get_solved_amount_top_first();
                        println!("max solved parts: {}", max);
                    }                    
            }

 


            return vec_moves.len() as u32;
        }
    }


