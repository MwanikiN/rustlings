struct Point {  //custom data type to represent a point in 2D space
    x: u64,
    y: u64,
}

enum Message {  //custom data type to represent different types of messages
    Resize { width: u64, height: u64 }, //variant with named fields for resizing
    Move(Point), //variant with a Point struct for moving
    Echo(String), //variant with a String for echoing messages
    ChangeColor(u8, u8, u8), //variant with three u8 values for changing color (RGB) tuple
    Quit,
}

struct State { //custom data type to represent the state of the application
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {

        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("hello world"),
        color: (0, 0, 0),
        quit: false,
    };

    state.process(Message::Resize {
        width: 800,
        height: 600,
    });

    state.process(Message::Move(Point { x: 100, y: 200 }));

    state.process(Message::Echo(String::from("Hello Rust!")));

    state.process(Message::ChangeColor(255, 0, 128));

    state.process(Message::Quit);

    println!("Width: {}", state.width);
    println!("Height: {}", state.height);
    println!("Position: ({}, {})", state.position.x, state.position.y);
    println!("Message: {}", state.message);
    println!("Color: {:?}", state.color);
    println!("Quit: {}", state.quit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
