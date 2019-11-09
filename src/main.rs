#[macro_use]
extern crate gfx;

use gfx::Device;
use gfx_gui::context::Vertex;
use std::time::Instant;

mod gfx_gui;

const CLEAR_COLOR: [f32;4] = [0.0,0.0,0.0,1.0];

pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Vec<Vertex> {
    let vertices= vec![
        Vertex{pos:[x, y, 0.0,1.0]},
        Vertex{pos:[x + width, y, 0.0f32,1.0f32]},
        Vertex{pos:[x + width, y - height, 0.0f32,1.0f32]},
        Vertex{pos:[x + width, y - height, 0.0,1.0]},
        Vertex{pos:[x, y - height, 0.0f32,1.0f32]},
        Vertex{pos:[x, y, 0.0f32,1.0f32]},
    ];
    vertices
}

pub fn centered_rect(x: f32, y: f32, width: f32, height: f32) -> Vec<Vertex> {
    let vertices= vec![
        Vertex{pos:[x - width / 2.0, y - height / 2.0, 0.0,1.0]},
        Vertex{pos:[x - width / 2.0, y + height / 2.0, 0.0f32,1.0f32]},
        Vertex{pos:[x + width / 2.0, y + height / 2.0, 0.0f32,1.0f32]},
        Vertex{pos:[x + width / 2.0, y - height / 2.0, 0.0,1.0]},
        Vertex{pos:[x - width / 2.0, y - height / 2.0, 0.0f32,1.0f32]},
        Vertex{pos:[x + width / 2.0, y + height / 2.0, 0.0f32,1.0f32]},
    ];
    vertices
}
pub fn draw_some(w: &mut gfx_gui::window::Window) {
    let vertices = rect(0.7, 0.1, 0.02, 1.0);
    w.context.add_triangles(&vertices, [1.0,0.0,0.0,0.6]);

    let vertices = rect(-0.7, 0.1, 0.02, 1.0);
    w.context.add_triangles(&vertices, [1.0,0.0,0.0,0.6]);

    let vertices = rect(0.0, 0.1, 0.02, 1.0);
    w.context.add_triangles(&vertices, [1.0,0.0,0.0,0.6]);


}

pub enum Rod {
    Left,
    Middle,
    Right
}

pub fn render_disks(n_rod: u8, disks: &Vec<f32>, w: &mut gfx_gui::window::Window) {
    let mut index = 1;
    for i in disks {
        let vertices = centered_rect(-0.7 + n_rod as f32 * 0.7 + 0.01 , -0.88 + index as f32 * 0.03, *i, 0.02);
        w.context.add_triangles(&vertices, [1.0, 1.0, 1.0, 0.8]);
        index+=1;
    }
}

pub struct Puzzle {
    rods: [Vec<f32>;3],
    smallest_pos: usize,
    smallest_move: bool,
    solved: bool
}

impl Puzzle {
    pub fn new(n: usize) -> Puzzle {
        let mut rods = [Vec::new(), Vec::new(), Vec::new()];
        let disk_width = 0.46;
        for i in 0..n {
            rods[0].push(disk_width - i as f32 *0.03);
        }
        Puzzle {
            rods,
            smallest_move: true,
            smallest_pos: 0,
            solved: false
        }
    }

    fn move_disk(&mut self, from: usize, to: usize) -> bool {
        if self.rods[from].len() > 0 {
            if self.rods[to].len() == 0 {
                let disk = self.rods[from].pop().unwrap();
                self.rods[to].push(disk);
                return true;
            } else {
                if self.rods[to].last().unwrap() > self.rods[from].last().unwrap() {
                    let disk = self.rods[from].pop().unwrap();
                    self.rods[to].push(disk);
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    pub fn do_move(&mut self) {
        if self.smallest_move {
            let from = self.smallest_pos;
            let to = (self.smallest_pos + 1) % 3;
            self.move_disk(from, to);
            self.smallest_pos = to;
        } else {
            let first = (self.smallest_pos + 1) % 3;
            let second =  (self.smallest_pos + 2) % 3;
            if !self.move_disk(first, second) {
                self.move_disk(second, first);
            }
        }
        self.smallest_move = !self.smallest_move;
        if self.rods[0].is_empty() {
            self.solved = true;
        }
    }
}
pub fn gfx_main() {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move||{
        let mut puzzle = Puzzle::new(9);
        let mut tm = Instant::now();
        let mut window = gfx_gui::window::Window::new("Hanoi Towers");
        loop {
            let elapsed = tm.elapsed();
            if elapsed.subsec_millis() > 100 {
                puzzle.do_move();
                if puzzle.solved {
                    break;
                }
                tm = Instant::now();
            }
            window.context.device.cleanup();
            window.context.encoder.clear(&window.context.out_color, CLEAR_COLOR);
            window.context.encoder.clear_depth(&window.context.out_depth, 1.0);
            window.text.draw(&mut window.context.encoder, &window.context.out_color).unwrap_or(());
            draw_some(&mut window);
            render_disks(0, &mut puzzle.rods[0], &mut window);
            render_disks(1, &mut puzzle.rods[1], &mut window);
            render_disks(2, &mut puzzle.rods[2], &mut window);
            window.context.encoder.flush(&mut window.context.device);
            let wnd = &window.window;
            window.event_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent {event, ..} => {
                        match event {
                            glutin::WindowEvent::CloseRequested => {
                                sender.send(0).unwrap();
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            });
            wnd.swap_buffers().unwrap();
        }
    });
    receiver.recv().unwrap_or(0);
}

fn main() {
    gfx_main();
}