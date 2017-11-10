struct World {
    width: usize,
    height: usize,
    buffer: [bool],
}

fn load (filename: String) {
    world = World(7, 4,
                  [false, true , false, false, false, false, false,
                   false, true , false, true , true , true , false,
                   false, true , false, false, false, false, false,
                   false, false, false, false, false, false, false]);
    return world;
}

fn blank (source: World) {
    world = World(source.width, source.height,
                  [false; source.width * source.height]);
    return world;
}

fn index (world: World, x: usize, y: usize) {
    return world.buffer[world.width*y + x];
}

fn assign (world: World, x: usize, y: usize, value: bool) {
    world.buffer[world.width*y + x] = value;
}

fn show (world: World) {
    println!("");
    println!("");
    
    // print top frame
    print!("+");
    for x in ..world.width {
        print!("-");
    }
    println!("+");
    
    // print body
    print!("|");
    for y in ..world.height {
        for x in ..world.width {
//            print!(if index(world, x, y) {"X"} else {" "});
//            print!(match index(world, x, y) {
//                b if b==true  => "X",
//                b if b==false => " ",
//            } as String);
            s: String = match index(world, x, y) {
                b if b==true  => "X",
                b if b==false => " ",
            };
            print!("{}", s);
        }
    }
    println!("|");
    
    // print bottom frame
    print!("+");
    for x in ..world.width {
        print!("-");
    }
    println!("+");
}

fn iterate (source: world, destination: world) {
    for y in ..source.height {
        for x in ..source.width {
            count: usize = index(source, x-1, y-1) + index(source, x, y-1) + index(source, x+1, y-1)
                         + index(source, x-1, y  ) + index(source, x, y  ) + index(source, x+1, y  )
                         + index(source, x-1, y+1) + index(source, x, y+1) + index(source, x+1, y+1);
            new: bool = match count {
                n if n==3 => true,
                n if n==4 => true,
                _         => false,
            };
            assign(destination, x, y, new);
        }
    }
}

fn main () {
    println!("John Conway's Game of Life");
    
    // load initial world
    let world1 = load("initial.map");
    let world2 = blank(world1);
    
    // iterate
    while true {
        show(world1);
        iterate(world1, world2);
        show(world2);
        iterate(world2, world1);
    }
}
