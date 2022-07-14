// Enums are types which have a few definite values

enum Movement {
    // Variants
    East,
    West,
    North,
    South,
  }
  
  fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
      Movement::East => println!("Avatar moving east"),
      Movement::West => println!("Avatar moving west"),
      Movement::North => println!("Avatar moving north"),
      Movement::South => println!("Avatar moving south"),
    }
  }
  
  pub fn run() {
    let avatar1 = Movement::East;
    let avatar2 = Movement::West;
    let avatar3 = Movement::North;
  let avatar4 = Movement::South;

   move_avatar(avatar1);
   move_avatar(avatar2);
   move_avatar(avatar3);
    move_avatar(avatar4);
  }