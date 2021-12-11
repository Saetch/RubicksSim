
use crate::r_cube_mod::RCube;


        pub struct  Sim{
        
        cube : RCube,


    }

    impl Sim{
        pub fn new() -> Self{
            Sim {
                cube : RCube::new()
            }
        }
    }


