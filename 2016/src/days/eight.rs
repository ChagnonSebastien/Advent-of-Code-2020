use crate::utils::Part;

fn draw(screen: [[bool; 50]; 6]) {
  println!("+--------------------------------------------------+");
  for row in &screen {
    print!("|");
    for pixel in row.iter() {
      print!("{}", match pixel { true => "#", false => " " });
    }
    println!("|");
  }
  println!("+--------------------------------------------------+");
}

pub fn execute(input: String, part: &Part) {
  let mut screen = [[false; 50]; 6];

  for instruction in input.split("\n") {
    let mut parts = instruction.split(" ");
    let verb = parts.next().unwrap();
    match verb {
      "rect" => {
        let sizes = parts.next().unwrap().split("x").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        println!("Draw rect {}x{}.", sizes[0], sizes[1]);

        for i in 0..sizes[1] {
          for j in 0..sizes[0] {
            screen[i][j] = true;
          }
        }
      },
      "rotate" => {
        let direction = parts.next().unwrap();
        let index = parts.next().unwrap().split("=").last().unwrap().parse::<usize>().unwrap();
        let offset = parts.last().unwrap().split("=").last().unwrap().parse::<usize>().unwrap();
        println!("Move {} {} by {}.", direction, index, offset);

        let mut pixel_array: Vec<bool> = Vec::new();
        match direction {
          "row" => {
            for col in 0..50 { pixel_array.push(screen[index][col]) }
            for col in 0..50 { screen[index][(col + offset) % 50] = pixel_array[col] }
          },
          "column" => {
            for row in 0..6 { pixel_array.push(screen[row][index]) }
            for row in 0..6 { screen[(row + offset) % 6][index] = pixel_array[row] }
          },
          _ => panic!("Unknown direction: {}", direction),
        }
      },
      _ => panic!("Unknown verb: {}", verb),
    }

    if *part == Part::PartTwo {
      draw(screen);
    }
  }

  if *part == Part::PartOne {
      let mut lit_pixels = 0;
      for row in &screen {
        for pixel in row.iter() {
          if *pixel { lit_pixels += 1 }
        }
      }
      println!("Amount of lit pixels: {}", lit_pixels);
  }

}
