use enigo::{Enigo, MouseButton, MouseControllable, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn main() {
    let wait_time = Duration::from_secs(1);
    let mut enigo = Enigo::new();

    thread::sleep(wait_time);

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No instruction file given.");
        println!("Usage: shadowhand instruction_file");
        exit(1);
    }

    if !Path::new(&args[1]).exists() {
        println!("Instruction file given doesn't exist.");
        exit(1);
    }

    let mut instructions = Vec::with_capacity(30);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let input_line = line.trim().to_string();
            let input_parts = input_line.split_whitespace().collect::<Vec<_>>();

            if input_parts.is_empty() {
                continue;
            }

            match input_parts[0] {
                "mouse_move_to" | "mouse_move_relative" => {
                    if input_parts.len() != 3 {
                        panic!("{} must have 2 int arguments in {}", input_parts[0], input_line);
                    }
                    let x = input_parts[1].parse::<i32>().unwrap_or_else(|_| panic!("mouse_move_to must have 2 int arguments in {}", input_line));
                    let y = input_parts[2].parse::<i32>().unwrap_or_else(|_| panic!("mouse_move_to must have 2 int arguments in {}", input_line));
                    match input_parts[0] {
                        "mouse_move_to" => instructions.push(Instruction {action: InstructionType::MouseMoveTo, x:Some(x) , y:Some(y), key_str:None, key_type:None }),
                        "mouse_move_relative" => instructions.push(Instruction {action: InstructionType::MouseMoveRelative, x:Some(x) , y:Some(y), key_str:None, key_type:None }),
                        _ => panic!(),
                    }
                },
                "mouse_click" | "mouse_down" | "mouse_up" => {
                    if input_parts.len() != 1 {
                        panic!("{} doesn't take any arguments in {}", input_parts[0], input_line);
                    }
                    match input_parts[0] {
                        "mouse_click" => instructions.push(Instruction {action: InstructionType::MouseClick, x:None , y:None, key_str:None, key_type:None }),
                        "mouse_down" => instructions.push(Instruction {action: InstructionType::MouseDown, x:None , y:None, key_str:None, key_type:None }),
                        "mouse_up" => instructions.push(Instruction {action: InstructionType::MouseUp, x:None , y:None, key_str:None, key_type:None }),
                        _ => panic!(),
                    }
                },
                "key_click" | "key_down" | "key_up" => {
                    if input_parts.len() != 2 {
                        panic!("{} must have 1 string argument in {}", input_parts[0], input_line);
                    }
                    match input_parts[0] {
                        "key_click" => instructions.push(Instruction {action: InstructionType::KeyClick, x:None , y:None, key_str:Some(input_parts[1].to_string()), key_type:None}),
                        "key_down" => instructions.push(Instruction {action: InstructionType::KeyDown, x:None , y:None, key_str:Some(input_parts[1].to_string()), key_type:None}),
                        "key_up" => instructions.push(Instruction {action: InstructionType::KeyUp, x:None , y:None, key_str:Some(input_parts[1].to_string()), key_type:None}),
                        _ => panic!(),
                    }

                },
                "key_sequence" => {
                    let sequence = input_line.replace("key_sequence", "").trim().to_string();
                    instructions.push(Instruction {action: InstructionType::KeySequence, x:None , y:None, key_str:Some(sequence), key_type:None});

                },
                _ => panic!("unrecognized keyword in input file. {}", input_line),
            }
        }
    }

    for instruction in instructions.iter_mut() {
        match instruction.action {
            InstructionType::KeyClick | InstructionType::KeySequence | InstructionType::KeyDown | InstructionType::KeyUp => {
                match instruction.key_str.as_ref().unwrap().to_lowercase().as_str() {
                    "{alt}" => instruction.key_type = Some(Key::Alt),
                    "{backspace}" => instruction.key_type = Some(Key::Backspace),
                    "{capslock}" => instruction.key_type = Some(Key::CapsLock),
                    "{command}" => instruction.key_type = Some(Key::Meta),
                    "{control}" => instruction.key_type = Some(Key::Control),
                    "{delete}" => instruction.key_type = Some(Key::Delete),
                    "{downarrow}" => instruction.key_type = Some(Key::DownArrow),
                    "{end}" => instruction.key_type = Some(Key::End),
                    "{escape}" => instruction.key_type = Some(Key::Escape),
                    "{f1}" => instruction.key_type = Some(Key::F1),
                    "{f10}" => instruction.key_type = Some(Key::F10),
                    "{f11}" => instruction.key_type = Some(Key::F11),
                    "{f12}" => instruction.key_type = Some(Key::F12),
                    "{f2}" => instruction.key_type = Some(Key::F2),
                    "{f3}" => instruction.key_type = Some(Key::F3),
                    "{f4}" => instruction.key_type = Some(Key::F4),
                    "{f5}" => instruction.key_type = Some(Key::F5),
                    "{f6}" => instruction.key_type = Some(Key::F6),
                    "{f7}" => instruction.key_type = Some(Key::F7),
                    "{f8}" => instruction.key_type = Some(Key::F8),
                    "{f9}" => instruction.key_type = Some(Key::F9),
                    "{home}" => instruction.key_type = Some(Key::Home),
                    "{leftarrow}" => instruction.key_type = Some(Key::LeftArrow),
                    "{meta}" => instruction.key_type = Some(Key::Meta),
                    "{option}" => instruction.key_type = Some(Key::Option),
                    "{pagedown}" => instruction.key_type = Some(Key::PageDown),
                    "{pageup}" => instruction.key_type = Some(Key::PageUp),
                    "{return}" => instruction.key_type = Some(Key::Return),
                    "{rightarrow}" => instruction.key_type = Some(Key::RightArrow),
                    "{shift}" => instruction.key_type = Some(Key::Shift),
                    "{space}" => instruction.key_type = Some(Key::Space),
                    "{super}" => instruction.key_type = Some(Key::Meta),
                    "{tab}" => instruction.key_type = Some(Key::Tab),
                    "{uparrow}" => instruction.key_type = Some(Key::UpArrow),
                    "{windows}" => instruction.key_type = Some(Key::Meta),
                    _ => instruction.key_type = Some(Key::Layout(instruction.key_str.as_ref().unwrap().chars().collect::<Vec<_>>()[0])),
                }
            },
            _ => (),

        }
    }

    for instruction in instructions {
        match instruction.action {
            InstructionType::MouseMoveTo => enigo.mouse_move_to(instruction.x.unwrap(), instruction.y.unwrap()),
            InstructionType::MouseMoveRelative => enigo.mouse_move_relative(instruction.x.unwrap(), instruction.y.unwrap()),
            InstructionType::MouseClick => enigo.mouse_click(MouseButton::Left),
            InstructionType::MouseDown => enigo.mouse_down(MouseButton::Left),
            InstructionType::MouseUp => enigo.mouse_up(MouseButton::Left),
            InstructionType::KeyClick => enigo.key_click(instruction.key_type.unwrap()),
            InstructionType::KeySequence => enigo.key_sequence(&instruction.key_str.unwrap()),
            InstructionType::KeyDown => enigo.key_down(instruction.key_type.unwrap()),
            InstructionType::KeyUp => enigo.key_up(instruction.key_type.unwrap()),
        }

        thread::sleep(wait_time);
    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum InstructionType {
    MouseMoveTo,
    MouseMoveRelative,
    MouseClick,
    MouseDown,
    MouseUp,
    KeyClick,
    KeySequence,
    KeyDown,
    KeyUp,
}


#[derive(Debug)]
struct Instruction {
    action: InstructionType,
    x: Option<i32>,
    y: Option<i32>,
    key_str: Option<String>,
    key_type: Option<Key>,
}
