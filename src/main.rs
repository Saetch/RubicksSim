mod r_cube_mod;
mod sim_mod;
mod cube_side_mod;

use std::{thread, io};

use crate::{sim_mod::Sim as Sim, r_cube_mod::TurnTypes};
use rand::Rng;
fn main() {
    //this is for getting the console output colored, when ANSI-support is disabled by default
    let enabled = ansi_term::enable_ansi_support();
    println!("RES: {}\n",!enabled.err().is_some());

    
    let execution_style = 3;


    match execution_style {
        0 => just_debug_scramble_unscramble(),
        1 => randomize_test(),
        2 => solve_cube_with_heuristic(),
        3 => step_test(),
        _ => ()
    }
 


}

fn just_debug_scramble_unscramble(){
   //println!("new Cube: \n{}",sim_sim.show_cube_status());

   let mut vec : Vec<TurnTypes> = Vec::new();
   let mut rand = rand::thread_rng();
   for i in 0..1000{
          let mut sim_sim = Sim::new();
   assert!(sim_sim.is_solved());

   for _i in 0..100{
       let ne ;
       match rand.gen::<u8>() % 6{
           0 => ne =TurnTypes::FrontLeft{ back_flip : rand.gen::<u8>()%2 == 0},
           1 => ne =TurnTypes::FrontRight{ back_flip : rand.gen::<u8>()%2 == 0},        
           2 => ne =TurnTypes::LeftSideLeft{ back_flip : rand.gen::<u8>()%2 == 0},         
           3 => ne =TurnTypes::LeftSideRight{ back_flip : rand.gen::<u8>()%2 == 0},
           4 => ne =TurnTypes::TopPlatform{ back_flip : rand.gen::<u8>()%2 == 0},
           5 => ne =TurnTypes::BottomPlatform{ back_flip : rand.gen::<u8>()%2 == 0},

           _ => {println!("Something went wrong!"); return;}
       }
       sim_sim.turn_mut_pip(&ne);  
    /*   match &ne{
           TurnTypes::FrontLeft { back_flip } => println!("FRONTLEFT+{}",back_flip),
           TurnTypes::FrontRight { back_flip} =>  println!("FRONTRIGHT+{}",back_flip),
           TurnTypes::LeftSideLeft { back_flip } =>  println!("LEFTLEFT+{}",back_flip),
           TurnTypes::LeftSideRight { back_flip } =>  println!("LEFTRIGHT+{}",back_flip),
           TurnTypes::TopPlatform { back_flip } =>  println!("TOPLEFT+{}",back_flip),
           TurnTypes::BottomPlatform { back_flip  } => println!("TOPRIGHT+{}",back_flip),
       }*/
       //println!("new Cube: \n{}",sim_sim.show_cube_status());

       vec.push(ne);

   }
   let mut turn_type_o = vec.pop();
   let mut turn_type;
   while turn_type_o.is_some(){
       turn_type = turn_type_o.unwrap();
       /*
       match &(turn_type.switch_direction()){
           TurnTypes::FrontLeft { back_flip } => println!("FRONTLEFT+{}",back_flip),
           TurnTypes::FrontRight { back_flip} =>  println!("FRONTRIGHT+{}",back_flip),
           TurnTypes::LeftSideLeft { back_flip } =>  println!("LEFTLEFT+{}",back_flip),
           TurnTypes::LeftSideRight { back_flip } =>  println!("LEFTRIGHT+{}",back_flip),
           TurnTypes::TopPlatform { back_flip } =>  println!("TOPLEFT+{}",back_flip),
           TurnTypes::BottomPlatform { back_flip  } => println!("TOPRIGHT+{}",back_flip),
       }*/
       sim_sim.turn_mut_pip(&turn_type.switch_direction());
       if i == 500{
           println!("new Cube: \n{}",sim_sim.show_cube_status());

       }

       turn_type_o = vec.pop();

   } 
   assert!(sim_sim.is_solved());
   }


   //sim_sim.randomize_cube_mut(100);


   //println!("new Cube: \n{}",sim_sim.show_cube_status());
}

fn randomize_test(){

    for _i in 0..=15{
        thread::spawn(thread_rando_test);
    }


    let mut sim_sim = Sim::new();
    sim_sim.randomize_cube_mut(145);

    println!("This took {} moves!", sim_sim.random_solve_prnt());

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
}

fn step_test(){
    for i in 0..15{
        thread::spawn(move || thread_solve_cube_with_steps(21+i as u32));
    }


    let mut sim_sim = Sim::new();
    sim_sim.default_cube();

    println!("This took{} moves!", sim_sim.solve_with_steps(1));
    println!("This thread started with {} steps",1);
}

fn thread_rando_test(){
    println!("Started thread! ");
    let mut sim_sim = Sim::new();
    sim_sim.randomize_cube_mut(145);

    println!("This took {} moves!", sim_sim.random_solve());
}

fn thread_solve_cube_with_steps(steps: u32){
    let mut sim_sim = Sim::new();
    sim_sim.default_cube();

    println!("This took{} moves!", sim_sim.solve_with_steps_skip(steps,15));
    println!("This thread started with {} steps",steps);
}

fn solve_cube_with_heuristic(){

    let mut sim_sim = Sim::new();



    if sim_sim.default_cube(){
        println!("new Cube: \n{}",sim_sim.show_cube_status());


       let amount =  sim_sim.simple_heuristic_solve();

       
    }
    else{
        println!("Cube not solved when trying to initialize!");
    }


}
