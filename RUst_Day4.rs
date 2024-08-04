---------------Modules

pub mod config {
    pub fn select() {
        println!("called config::select");
    }
}

fn main() {
    config::select();
} 

--------------------------------

mod c  
{  
	pub fn c()  
	{  
		println!("C is a structured programming language");  
	}    

	pub fn cplus()  
	{  
		println!("C++ is an object-oriented programming language");  
	}  
}  
fn main()  
{  
  c::c();  
  c::cplus();  
}  

--------------------------------

mod player {
    fn focus() {
        println!("called player::focus");
    }

    pub fn shift() {
        println!("called player::shift");
    }

    pub fn jump() {
        focus();
        shift();
        println!("called player::jump");
    }
}

use player::jump;

fn main() {
    jump();
}

------------------------------Nested

pub mod movies {
   pub fn play(name:String) {
      println!("Playing movie {}",name);
   }
}
fn main(){
   movies::play("PESU Stories".to_string());
}

---------------------

pub mod movies {
   pub fn play(name:String) {
      println!("Playing movie {}",name);
   }
}
use movies::play;
fn main(){
   play("PESU Stories ".to_string());
}

-------------

pub mod movies {
   pub mod english {
      pub mod comedy {
         pub fn play(name:String) {
            println!("Playing comedy movie {}",name);
         }
      }
   }
}
use movies::english::comedy::play; 

fn main() {
   // short path syntax
   play("PESU Stories".to_string());
   play("The Hangover".to_string());

   //full path syntax
   movies::english::comedy::play("Airplane!".to_string());
}