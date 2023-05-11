use remu::exes::elf::ELF;
use remu::exes::exe::Exe;
use remu::fatal;
use remu::ioe::keyboard::KBEvent;
use remu::isas::isa::ISA;
use remu::isas::riscv::cpu::RiscvCPU;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    // init cpu and load binary
    let mut cpu = RiscvCPU::default();
    let mut exe = {
        if args.len() >= 2 {
            ELF::parse_path(&args[1]).unwrap()
        } else {
            fatal!("Usage: {} <elf> [args]", args[0]);
            std::process::exit(1);
        }
    };
    exe.load_binary(&mut cpu).unwrap();

    // init devices, i.e. vga, keyboard
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("REMU", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .unwrap();
    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut pixels = [0_u8; (WIDTH * HEIGHT * 3) as usize];
    let mut last = std::time::Instant::now();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }      => break 'running,
                Event::KeyUp { .. } | 
                Event::KeyDown { .. }   => {
                    for device in cpu.mems.devices.iter_mut() {
                        if device.name() == "keyboard" {
                            let code = KBEvent::from(event);
                            device.write(0, u32::from(code) as u64);
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        match cpu.step() {
            Ok(_) => {}
            Err(remu::error::RError::Ebreak(code)) => {
                println!("Ebreak: {}", code);
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
        let now = std::time::Instant::now();
        if now - last >= std::time::Duration::from_millis(1000/15) {
            last = now;
            // let start = std::time::Instant::now();
            for device in cpu.mems.devices.iter_mut() {
                if device.name() == "vga" {
                    // only consume 0.5ms per frame
                    for i in 0..WIDTH * HEIGHT {
                        let value = device.read(i as u64).unwrap();
                        pixels[i as usize * 3] = (value & 0xff) as u8; // r
                        pixels[i as usize * 3 + 1] = ((value >> 8) & 0xff) as u8; // g
                        pixels[i as usize * 3 + 2] = ((value >> 16) & 0xff) as u8; // b
                    }
                    texture.update(None, &pixels, WIDTH as usize * 3).unwrap();
                    canvas.copy(&texture, None, None).unwrap();
                    canvas.present();
                    break;
                }
            }
            // let end = std::time::Instant::now();
            // println!("{}ms consumed", (end - start).as_millis());
        }
    }
}
