struct World {
    width: usize,
    height: usize,
    data: [bool],
}

fn load (filename) {
    data = World(7, 4,
                 [false, true , false, false, false, false, false,
                  false, true , false, true , true , true , false,
                  false, true , false, false, false, false, false,
                  false, false, false, false, false, false, false]);
    return data;
}

fn blank (source) {
    data = World(source.width, source.height,
                 [false; source.width * source.height])
}

fn index (buffer, x, y) {
    return buffer.data[buffer.width*y + x]
}

fn assign (buffer, x, y, value) {
    buffer.data[buffer.width*y + x] = value;
}

fn show (buffer) {
    println!("");
    println!("");
    
    // print top frame
    print!("+");
    for x in ..buffer.width {
        print!("-");
    }
    println!("+");
    
    // print body
    print!("|");
    for y in ..buffer.height {
        for x in ..buffer.width {
            print!(if index(buffer, x, y) {"X"} else {" "});
        }
    }
    println!("|");
    
    // print bottom frame
    print!("+");
    for x in ..buffer.width {
        print!("-");
    }
    println!("+");
}

fn iterate (source, destination) {
    for y in ..buffer.height {
        for x in ..buffer.width {
            count: usize = index(buffer, x-1, y-1) + index(buffer, x, y-1) + index(buffer, x+1, y-1)
                         + index(buffer, x-1, y  ) + index(buffer, x, y  ) + index(buffer, x+1, y  )
                         + index(buffer, x-1, y+1) + index(buffer, x, y+1) + index(buffer, x+1, y+1);
            new: bool = match count {
                n if n==3 => true,
                n if n==4 => true,
                _         => false,
            }
            assign(buffer, x, y, new);
        }
    }
}

fn main () {
    println!("John Conway's Game of Life");
    
    // load initial world
    let buffer1 = load("initial.map");
    let buffer2 = blank(buffer1);
    
    // iterate
    while true {
        show(buffer1);
        iterate(buffer1, buffer2);
        show(buffer2);
        iterate(buffer2, buffer1):
    }
}
