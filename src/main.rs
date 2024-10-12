
//shadowing concept 

fn main() {
    let mut apples = 12 ; 
    
    apples = 13 ; //this is okay 

   // apples = true; //not okay

    let apples = true; //this is okay //this is shadowing concept

    //even if the variable is mut you cant change its type but u can change the type using let
    //so we can make a new variable with a same name of existing variable

}


