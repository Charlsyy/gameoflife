extern crate gtk;
extern crate gdk;
extern crate gameoflife;

//use gameoflife::*;
mod components;
use components::App;
use gtk::prelude::*;


const HEIGHT: usize = 10;
const WIDTH: usize = 10;

#[derive(Copy, Clone)]
struct Field {
    active: bool,
    upper_left: bool,
    upper: bool,
    upper_right: bool,

    left: bool,
    right: bool,

    lower_left: bool,
    lower: bool,
    lower_right: bool,

    
}
fn main() {
    let mut array_struct = vec![
        vec![
            Field {
                active: false,

                upper_left: false,
                upper: false,
                upper_right: false,

                left: false,
                right: false,

                lower_left: false,
                lower: false,
                lower_right: false,

                
            };
            10
        ];
        10
    ];
    let app = App::new();
    app.window.show_all();
    gtk::main();

    update_array(&mut array_struct);
    array_struct[3][3].active= true;
    array_struct[3][4].active= true;
    array_struct[3][5].active= true;

    update_array(&mut array_struct);


    next_turn(&mut array_struct);
    update_array(&mut array_struct);
    


}














fn update_array(array_struct: &mut Vec<Vec<Field>>) {
    for i in 1..HEIGHT - 1 {
        for k in 1..WIDTH - 1 {
            array_struct[i][k].upper_left = array_struct[i - 1][k - 1].active;
            array_struct[i][k].upper = array_struct[i - 1][k].active;
            array_struct[i][k].upper_right = array_struct[i - 1][k + 1].active;

            array_struct[i][k].left = array_struct[i][k - 1].active;
            array_struct[i][k].right = array_struct[i][k + 1].active;

            array_struct[i][k].lower_left = array_struct[i + 1][k - 1].active;
            array_struct[i][k].lower = array_struct[i + 1][k].active;
            array_struct[i][k].lower_right = array_struct[i + 1][k + 1].active;
        }
    }

   for i in 1..HEIGHT - 1 {
            array_struct[i][0].upper_left = array_struct[i - 1][WIDTH- 1].active;
            array_struct[i][0].left = array_struct[i][WIDTH-1].active;
            array_struct[i][0].lower_left = array_struct[i + 1][WIDTH- 1].active;

            array_struct[i][0].upper = array_struct[i - 1][0].active;
            array_struct[i][0].upper_right = array_struct[i - 1][1].active;
            array_struct[i][0].right = array_struct[i][1].active;

            array_struct[i][0].lower = array_struct[i + 1][0].active;
            array_struct[i][0].lower_right = array_struct[i + 1][1].active;
    }

   for i in 1..HEIGHT-1 {
       array_struct[i][WIDTH-1].upper_right = array_struct[i-1][0].active;
       array_struct[i][WIDTH-1].right = array_struct[i][0].active;
       array_struct[i][WIDTH-1].lower_right = array_struct[i+1][0].active;
            array_struct[i][WIDTH-1].upper_left = array_struct[i - 1][WIDTH - 2].active;
            array_struct[i][WIDTH-1].upper = array_struct[i - 1][WIDTH-1].active;

            array_struct[i][WIDTH-1].left = array_struct[i][WIDTH- 2].active;

            array_struct[i][WIDTH-1].lower_left = array_struct[i + 1][WIDTH - 2].active;
            array_struct[i][WIDTH-1].lower = array_struct[i + 1][WIDTH-1].active;
   }


    for i in 1..WIDTH-1 {
        array_struct[0][i].upper_left = array_struct[HEIGHT - 1][i - 1].active;
        array_struct[0][i].upper = array_struct[HEIGHT - 1][i].active;
        array_struct[0][i].upper_right = array_struct[HEIGHT - 1][i + 1].active;

            array_struct[0][i].left = array_struct[0][i- 1].active;
            array_struct[0][i].right = array_struct[0][i + 1].active;

            array_struct[0][i].lower_left = array_struct[1][i - 1].active;
            array_struct[0][i].lower = array_struct[1][i].active;
            array_struct[0][i].lower_right = array_struct[1][i + 1].active;
    }

    for i in 1..WIDTH-1 {
        array_struct[HEIGHT - 1][i].lower_left = array_struct[0][i - 1].active;
        array_struct[HEIGHT - 1][i].lower = array_struct[0][i].active;
        array_struct[HEIGHT - 1][i].lower_right = array_struct[0][i + 1].active;

            array_struct[HEIGHT-1][i].upper_left = array_struct[HEIGHT - 2][i - 1].active;
            array_struct[HEIGHT-1][i].upper = array_struct[HEIGHT - 2][i].active;
            array_struct[HEIGHT-1][i].upper_right = array_struct[HEIGHT - 2][i + 1].active;

            array_struct[HEIGHT-1][i].left = array_struct[HEIGHT-1][i - 1].active;
            array_struct[HEIGHT-1][i].right = array_struct[HEIGHT-1][i + 1].active;
    }

      array_struct[0][0].upper_left = array_struct[HEIGHT-1][WIDTH-1].active;
      array_struct[0][0].upper = array_struct[HEIGHT-1][0].active;
      array_struct[0][0].upper_right = array_struct[HEIGHT-1][1].active;

      array_struct[0][0].left = array_struct[0][WIDTH-1].active;
      array_struct[0][0].right = array_struct[0][1].active;

      array_struct[0][0].lower_left = array_struct[1][WIDTH - 1].active;
      array_struct[0][0].lower = array_struct[1][0].active;
      array_struct[0][0].lower_right = array_struct[1][1].active;

      
      array_struct[0][WIDTH-1].upper_left = array_struct[HEIGHT-1][WIDTH-2].active;
      array_struct[0][WIDTH-1].upper = array_struct[HEIGHT-1][WIDTH-1].active;
      array_struct[0][WIDTH-1].upper_right = array_struct[HEIGHT-1][0].active;

      array_struct[0][WIDTH-1].left = array_struct[0][WIDTH-2].active;
      array_struct[0][WIDTH-1].right = array_struct[0][0].active;

      array_struct[0][WIDTH-1].lower_left = array_struct[1][WIDTH - 2].active;
      array_struct[0][WIDTH-1].lower = array_struct[1][WIDTH-1].active;
      array_struct[0][WIDTH-1].lower_right = array_struct[1][0].active;

      
      
      array_struct[HEIGHT-1][0].upper_left = array_struct[HEIGHT-2][WIDTH-1].active;
      array_struct[HEIGHT-1][0].upper = array_struct[HEIGHT-2][0].active;
      array_struct[HEIGHT-1][0].upper_right = array_struct[HEIGHT-2][1].active;

      array_struct[HEIGHT-1][0].left = array_struct[HEIGHT-1][WIDTH-1].active;
      array_struct[HEIGHT-1][0].right = array_struct[HEIGHT-1][1].active;

      array_struct[HEIGHT-1][0].lower_left = array_struct[0][WIDTH - 1].active;
      array_struct[HEIGHT-1][0].lower = array_struct[0][0].active;
      array_struct[HEIGHT-1][0].lower_right = array_struct[0][1].active;

      
      array_struct[HEIGHT-1][WIDTH-1].upper_left = array_struct[HEIGHT-2][WIDTH-2].active;
      array_struct[HEIGHT-1][WIDTH-1].upper = array_struct[HEIGHT-2][WIDTH-1].active;
      array_struct[HEIGHT-1][WIDTH-1].upper_right = array_struct[HEIGHT-2][0].active;

      array_struct[HEIGHT-1][WIDTH-1].left = array_struct[HEIGHT-1][WIDTH-2].active;
      array_struct[HEIGHT-1][WIDTH-1].right = array_struct[HEIGHT-1][0].active;

      array_struct[HEIGHT-1][WIDTH-1].lower_left = array_struct[0][WIDTH - 2].active;
      array_struct[HEIGHT-1][WIDTH-1].lower = array_struct[0][WIDTH-1].active;
      array_struct[HEIGHT-1][WIDTH-1].lower_right = array_struct[0][0].active;


}






 
fn next_turn(array_struct: &mut Vec<Vec<Field>>) {
    for i in 0..HEIGHT {
        for k in 0..WIDTH {
            let mut counter=0;

                if array_struct[i][k].upper_left == true{
                    counter+=1;
                }
                if array_struct[i][k].upper == true{
                    counter+=1;
                }
                if array_struct[i][k].upper_right == true{
                    counter+=1;
                }

                if array_struct[i][k].left == true{
                    counter+=1;
                }
                if array_struct[i][k].right == true{
                    counter+=1;
                }

                if array_struct[i][k].lower_left == true{
                    counter+=1;
                }
                if array_struct[i][k].lower == true{
                    counter+=1;
                }
                if array_struct[i][k].lower_right == true{
                    counter+=1;
                }


            if array_struct[i][k].active == false {
                if counter==3{
                    array_struct[i][k].active=true;
                }

            }

            if array_struct[i][k].active==true {
                if counter <=1 {
                    array_struct[i][k].active=false;
                }


                else if counter>=4{
                    array_struct[i][k].active=false;
                }

            }
        }

    }

}
