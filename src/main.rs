extern crate rand;

use std::io;
use rand::Rng;

//use std::io::prelude::*;
//use std::io::SeekFrom;
//use std::fs::File;

/*fn disp_array( move_array: &[bool;25] ) {
    let mut vec = create_empty_grid(5);
    for i in 0..25 {
        if move_array[i] {
            do_move( &mut vec, 5, [i%5, i/5] );
        }
    }
    
    display( &vec, 5);
}*/


fn number_of_moves_to_solve(puzzle_number: usize) -> usize {
    let best_puzzle_number = shortest_solution(puzzle_number);
    let mut power_of_two: usize = 1;
    let mut count: usize = 0;
    for _ in 0..25 {
        if best_puzzle_number & power_of_two != 0{
            count +=1; 
        }
        power_of_two *= 2;
    }
    return count;
    
}

// returns a particular solution to grid where true in spot i means click [i%5, i/5] on the grid.
fn get_particular_solution(puzzle_number: usize) -> [bool; 25] {
    let mut solution = [false;25]; 
    
    let mut power_of_two: usize = 1;
    for i in 0..25 {
        if puzzle_number & power_of_two != 0 {
            solution[i] = true;
        }
        power_of_two *= 2;
    }
    return solution;
}

// takes a puzzle number and returns the puzzle_number that refers to the same puzzle
// but uses the fewest moves
fn shortest_solution(puzzle_number: usize) -> usize {
   
    let n_1 = [true, false, true, false, true, true, false, true, false, true, false, false, false, false, false, true, false, true, false, true, true, false, true, false, true]; //move [5,5] in terms of others
    let n_2 = [false, true, true, true, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, false]; // move [4,5] in terms of others
    let n_3 = xor_array(&n_1,&n_2);
    // true in spot i tells you to do move i
    
    /*disp_array(&n_1);
    disp_array(&n_2);
    disp_array(&n_3);*/

    let particular_solution = get_particular_solution(puzzle_number);
    //disp_array(&particular_solution);
    let mut number_moves = num_moves(particular_solution);
    
    let mut best_array = particular_solution;
    
    // do what n_1 says
    let mut temp = xor_array( &particular_solution, &n_1 );
    if num_moves( temp ) < number_moves {
        number_moves = num_moves(temp);
        best_array = temp;
    }
   // disp_array(&xor_array( &particular_solution, &n_1 ));
    
    temp = xor_array( &particular_solution, &n_2);
    if num_moves( temp ) < number_moves {
        number_moves = num_moves(temp);
        best_array = temp;
    }
    //disp_array(&xor_array( &particular_solution, &n_2 ));
    
    temp = xor_array( &particular_solution, &n_3);
    if num_moves( temp ) < number_moves {
        //number_moves = num_moves(temp);
        best_array = temp;
    }
    //disp_array(&xor_array( &particular_solution, &n_3 ));
    
    
    let mut shortest_puzzle_number: usize = 0;
    let mut power_of_two: usize = 1;
    for i in 0..25 {
        if best_array[i] {
            shortest_puzzle_number += power_of_two;
        }
        power_of_two *= 2;
    }
    
    return shortest_puzzle_number;
}

fn num_moves( bool_array: [bool;25] ) -> usize {
    let mut count: usize = 0;
    for i in 0..25 {
        if bool_array[i] {
            count += 1;
        }
    }
    return count;
}

fn print_solution(puzzle_number:usize) {
    let shortest_number = shortest_solution(puzzle_number);

    
    println!("Puzzle {} (equivalent to Puzzle {}) is solved by the following moves:", puzzle_number, shortest_number);
	let mut power: usize = 1;
	for i in 0..25 {
		if shortest_number & power !=0 {
			//let index: [usize;2] = [i%grid_size, i/grid_size];
			//do_move(&mut grid, grid_size, index);
			print!("({},{})  ",i%5+1, i/5+1);
		}
		power *= 2;
	}
	print!("\n");
}


fn num_to_puzzle( puzzle_number: usize, grid_size: usize ) -> Vec<bool> {
	let mut grid: Vec<bool> = create_empty_grid(grid_size);
	let mut power: usize = 1;
	for i in 0..grid_size*grid_size {
		if puzzle_number & power !=0 {
			let index: [usize;2] = [i%grid_size, i/grid_size];
			do_move(&mut grid, grid_size, index);
		}
		power *= 2;
	}
	
	return grid;
	
	
}

fn get_random_number() -> usize {
	let mut rng = rand::thread_rng();
	let random_number = rng.gen::<usize>() % 33554432 +1; // 8388608 = 2^25
	return random_number;
}


/*fn bits_to_puzzle( file_name: &str, puzzle_number: usize, grid_size: usize ) -> Vec<bool> {
	
	let mut f: std::fs::File;
	
	match File::open(file_name) {
		Err(_) => {
			println!("Unable to open file: {}",file_name);
			std::process::exit(1);
		}
		_ => f = File::open(file_name).unwrap(),
	}


	let buffer_size: usize= grid_size*grid_size/8 + 1;

	let mut buffer: Vec<u8> = vec![0; buffer_size];
	
	
	let seek_point: u64 = (buffer_size*puzzle_number) as u64;
	match f.seek(SeekFrom::Start(seek_point)) {
		Err(_) => panic!("Failed to seek in file"),
		_ => {},
	}
	for i in 0..buffer_size {
		let mut temp_buffer: [u8; 1] = [0];
		match f.read(&mut temp_buffer) {
			Err(_) => panic!("Failed to read form file"),
			_ => {},
		}
		buffer[i] = temp_buffer[0];
	}
	
	let mut grid: Vec<bool> = create_empty_grid(grid_size);
	for i in 0..buffer_size {
		let powers_of_two = [1,2,4,8,16,32,64,128];
		for j in 0..8 {
			// vector_index is the index (>=0, <grid_size^2) of the grid in vector form. 
			// it is a single number. it is also which bit we are dealing with
			// we don't want bits greater than grid_size^2
			let vector_index = i*buffer_size+j;
			if vector_index < grid_size*grid_size {
				if buffer[i] & powers_of_two[j] != 0 {					
					// my computer stores least significant bit first
					let index: [usize;2] = [vector_index%grid_size,vector_index/grid_size];
					do_move( &mut grid, grid_size, index);
				}
			}
		}
	}

	return grid;
}*/

enum InnerMenuOption {
	Help,
	Quit,
	Solve,
}

enum InnerMenuResult {
	Continue,
	Quit,
	GoToMenu,
	ShowSolution,
}


fn inner_menu(option: InnerMenuOption) -> InnerMenuResult {
	match option {
		InnerMenuOption::Help => {
			println!("\nInput \t Function\nH\t Display this Help message\nQ\t Exit current puzzle and/or return to outer inner_menu\n");
			return InnerMenuResult::Continue;
		}
		InnerMenuOption::Quit=> {
			loop {
				println!("Do you wish to\n\t(Q) Quit the game (Press Q) or \n\t(M) Return to the Main Menu (Press M) or \n\t(C) Continue with your current game (Press C)");
				let mut input = String::new();
				match io::stdin().read_line(&mut input) {
					Err(_) => panic!("Failed to get input"),
					_ => {},
				}
				
				match input.trim() {
					"Q" | "q" => return InnerMenuResult::Quit,
					"M"|"m"|"R"|"r" => return InnerMenuResult::GoToMenu,
					"C"|"c" => return InnerMenuResult::Continue,
					_ => println!("Please enter Q, M, or C"),
				}
			
			}
		}
		InnerMenuOption::Solve => return InnerMenuResult::ShowSolution,
	}
}

fn xor_bool(a: bool, b:bool) -> bool {
	match (a,b) {
		(true, false) => return true,
		(false, true) => return true,
		_ => return false,
	}
}

fn xor_array(v: &[bool;25], u: &[bool;25]) -> [bool;25] {
 
    let mut answer: [bool;25] = [false;25];
    for i in 0..25 {
        answer[i] = xor_bool( u[i], v[i] );
    }
    return answer;
    
    
}


fn display( grid: &Vec<bool>, grid_size: usize ) {	
	

	print!("\t");
	for i in 0..grid_size {
		print!("{}\t",i+1);
	}
	print!("--x\n\n");
	for i in 0..grid_size {
		print!("{}",i+1);
		for j in 0..grid_size {
			match grid[grid_size*i+j] {
				true => print!("\tX"),
				false => print!("\t."),
			}
			//print!( "\t{}", grid[grid_size*i+j]);
		}
		print!("\n");
	}
	println!("\n|\ny");

}

// get gridsize as input from user

/*fn get_grid_size(grid_choices: &Vec<usize>) -> usize {


	print!("What size grid would you like to play on? (choose from:");
	for i in 0..grid_choices.len() {
		print!(" {}", grid_choices[i]);
	}
	println!(")");
	
	
	let grid_size: usize;
	loop {
	
		let mut input = String::new();
		// read input, handle error
		match io::stdin().read_line(&mut input) {
			Err(_) => panic!("Failed to get input"),
			_ => {},
		}
		
		match input.trim() {
			"5" => {
				grid_size = 5;
				break;
			},
			"10" => {
				grid_size = 10;
				break;
			}
			_ => println!("Bad input: {}. Please input a valid choice.", input.trim()),
		}
	}


	return grid_size;

}*/

fn get_puzzle_number() -> usize {
	let puzzle_number: usize;
	println!("Input a puzzle number (R for Random, Q to Quit):");
	loop {
		
		let mut input = String::new();
		// read input, handle error
		match io::stdin().read_line(&mut input) {
			Err(error) => panic!("Failed to get input, error: {}",error),
			Ok(_) => {},
		}
		
		
		
		match input.trim() {
			"q"|"Q" => std::process::exit(0),
			"r"|"R" => {
				puzzle_number = get_random_number();
				return puzzle_number;
			}
			_ => {
				match input.trim().parse::<usize>() {
					Err(_) => println!("Please input a valid puzzle number (R for Random, Q to Quit)"),
					_ =>  {
						puzzle_number = input.trim().parse::<usize>().unwrap();
						break;
					}
				}
			}
		}
		
		
	}

	
	return puzzle_number;
}

fn outer_menu() -> [usize;2] {
	
	// UNCOMMENT TO GET USER INPUT GRIDSIZE
	//let grid_choices: Vec<usize> = vec![5,10];
	// get gridsize as input from user
	
	//let grid_size = get_grid_size( &grid_choices);
	//println!("You have chosen a grid size of: {}x{}", grid_size,grid_size);
	
	let grid_size = 5;
	
	let puzzle_number = get_puzzle_number();
	println!("You have chosen puzzle {}", puzzle_number);
	
	return [grid_size,puzzle_number];
}



// takes input from 1<= x,y <= grid_size,
// outputs to 0<= x,y < grid_size
// also deals with ingame inner_menu input
fn get_xy_input(grid_size: usize, puzzle_number: usize) -> [usize;2] {
	
	let mut xy: [usize;2] = [99,99];
	
	let letters = ['X','Y'];
	for i in 0..2 {
		loop {	
			println!("Input a valid {}-COORDINATE or option (h for help):",letters[i]);
			let mut input = String::new();
			// read input, handle error
			match io::stdin().read_line(&mut input) {
				Err(_) => panic!("Failed to get input"),
				_ => {},
			}
			
			let menu_result: InnerMenuResult;
			match input.trim() {
				"H" | "h" => menu_result = inner_menu(InnerMenuOption::Help),
				"Q" | "q" => menu_result = inner_menu(InnerMenuOption::Quit),
				"solve" => menu_result = inner_menu(InnerMenuOption::Solve),
				_ => menu_result = InnerMenuResult::Continue,
			}
			
			match menu_result {
				InnerMenuResult::Quit => {
					println!("Now exiting...");
					std::process::exit(0);
				}
				InnerMenuResult::GoToMenu => {
					println!("Returning to outer menu.");
					game();
				}
				InnerMenuResult::ShowSolution => {
					print_solution(puzzle_number);
				}
				_ => {
					match input.trim().parse::<usize>() {
					
							Err(_) => println!("Please input a number between 1 and {} (inclusive) or a valid inner_menu entry (H for help)",grid_size),
							_ => {
								let temp: usize = input.trim().parse::<usize>().unwrap();
								if temp <= grid_size && 0< temp {
									xy[i] = temp;
									break;
							}
						}
					}
				}
			}
			
			

				
			
				
		}
	}
	if xy[0]==99 || xy[1] == 99 {
		unreachable!();
	}
	
	xy[0]-=1;
	xy[1]-=1;
	
	return xy;

}



// uses indices starting at 0. 
fn index_to_move(index: [usize;2], grid_size: usize) -> Vec<bool> {
	let mut the_move = vec![false; grid_size*grid_size];
	let i = index[0];
	let j = index[1];
	if !(i<grid_size && j<grid_size) {
		panic!("Index exceded grid dimensions: ({},{}) on a {}x{} grid",i,j,grid_size,grid_size);
	}
	the_move[grid_size*j+i] = true;
	if i>0 {
		the_move[grid_size*j+i-1] = true;
	}
	if j>0 {
		the_move[grid_size*(j-1)+i] = true;
	}
	if i<grid_size-1 {
		the_move[grid_size*j+i+1] = true;
	}
	if j<grid_size-1 {
		the_move[grid_size*(j+1)+i] = true;
	}
	
	return the_move;
}

fn do_move(grid: &mut Vec<bool>, grid_size: usize, index: [usize;2] ) {
	let the_move = index_to_move(index,grid_size);
	for i in 0..grid.len() {
		grid[i] = xor_bool(grid[i],the_move[i]);
	}
	
}

fn is_solved(grid: &Vec<bool>) -> bool {
	for i in 1..grid.len() {
		if grid[i] {
			return false;
		}
	}
	return true;
}

fn create_empty_grid(grid_size: usize) -> Vec<bool> {
	let grid: Vec<bool> = vec![false; grid_size*grid_size];
	return grid;
}


fn play_loop(grid_size: usize, puzzle_number: usize) {
	
	let mut grid: Vec<bool> = num_to_puzzle(puzzle_number,grid_size);
	
	
	
	println!("Here is your puzzle: ");
	
	let mut number_of_moves: usize = 0;
	while !is_solved(&grid) {
		display(&grid, grid_size);
		
		let index = get_xy_input( grid_size, puzzle_number);
		
		do_move( &mut grid,grid_size, index);
		number_of_moves+=1;
	
	}
	
	display(&grid, grid_size);
	
	println!("You Won in {} moves!", number_of_moves);
    println!("Puzzle {} is solvable in as few as {} moves", puzzle_number, number_of_moves_to_solve(puzzle_number));
	game();

}


fn game() {
	
	let outer_menu_results = outer_menu();
	
	let grid_size = outer_menu_results[0];
	let puzzle_number = outer_menu_results[1];
	
	play_loop(grid_size, puzzle_number);
}

fn main() {

    //shortest_solution(28111004);
	game();
	
}
