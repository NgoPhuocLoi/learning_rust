
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self){
        println!("Draw a Button with label: {}", self.label);
    }
}

struct Image {
    width: u16,
    height: u16
}

impl Draw for Image {
    fn draw(&self){
        println!("Draw a Image({}, {})", self.width, self.height);
    }
}


fn main() {
    let mut components: Vec<Box<dyn Draw>> = Vec::new();

    components.push(Box::new(Button {
        label: String::from("Button 1")
    }));

    components.push(Box::new(Image {
        width: 20,
        height: 40
    }));

    for item in components.iter() {
        item.draw();
    }
}
