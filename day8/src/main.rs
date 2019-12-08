use std::fs;
extern crate image;

// 145 too low

#[derive(Debug)]
struct Layer {
    contents: Vec<u32>,
    counts: Vec<usize>,
    id: usize
}

impl Default for Layer {
    fn default () -> Layer {
        Layer{id: 0, contents: Vec::new(), counts: Vec::new()}
    }
}

fn main() {
    // let input = "123456789012".to_string();
    let input = fs::read_to_string("src/input.txt")
    .expect("Something went wrong reading the file").trim().to_string();
    let size: usize = 150;
    part1(input, size);
    let input = fs::read_to_string("src/input.txt")
    .expect("Something went wrong reading the file").trim().to_string();
    let size: usize = 150;
    part2(input, size);
}

fn part1(input: String, size: usize) {
    let mut layerlist: Vec<Layer> = Vec::new();
    let layers = input.len() / size;

    for i in 0..layers {
        // println!("{}", i*size);
        let contents: Vec<u32> = input[i*size..(i+1)*size].chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
        let mut counts: Vec<usize> = Vec::new();

        for j in 0..3 {
            counts.push(contents.iter().filter(|&n| *n == j).count());
        }

        let currlayer = Layer{ id: i, contents: contents, counts: counts };
        // println!("Layer {}: {:?}\n", i, &currlayer);
        layerlist.push(currlayer);
    }
    // println!("Layers: {}", layers);
    // println!("LayerList: {:?}", layerlist);

    // Count digits already in Layer struct
    let mut lowest = size;
    let mut lowest_layer: Layer = Layer::default();
    for layer in layerlist {
        if layer.counts[0] < lowest {
            lowest = layer.counts[0];
            lowest_layer = layer;
        }
    }

    println!("Lowest: {}", lowest);
    println!("Lowest: {:?}", lowest_layer);

    let answer = lowest_layer.counts[1] * lowest_layer.counts[2];

    println!("Answer: {}", answer);

}

fn part2(input: String, size: usize) {
    let mut layerlist: Vec<Layer> = Vec::new();
    let layers = input.len() / size;

    for i in 0..layers {
        // println!("{}", i*size);
        let contents: Vec<u32> = input[i*size..(i+1)*size].chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
        let mut counts: Vec<usize> = Vec::new();

        for j in 0..3 {
            counts.push(contents.iter().filter(|&n| *n == j).count());
        }

        let currlayer = Layer{ id: i, contents: contents, counts: counts };
        // println!("Layer {}: {:?}\n", i, &currlayer);
        layerlist.push(currlayer);
    }
    // println!("Layers: {}", layers);
    // println!("LayerList: {:?}", layerlist);

    let mut imgbuf = image::ImageBuffer::new(25, 6);

    layerlist.reverse();

    for layer in layerlist {
        // println!("{:?}", layer);
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let idx = x + y* 25;
            let nr = layer.contents[idx as usize];
            match nr {
                0 => *pixel = image::Rgba([0, 0, 0, 255]),
                1 => *pixel = image::Rgba([255, 255, 255, 255]),
                2 => (),
                _ => println!("You bad propgrammer boi")
            }
        }
    }

    imgbuf.save("image/answer.png").unwrap();

}
