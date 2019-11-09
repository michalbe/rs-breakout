use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::blueprints::blu_common::Blueprint;
use crate::components::com_move::Move;
use crate::components::com_draw2d::Draw2d;
use crate::components::com_transform2d::Transform2d;
use crate::components::com_controll_ball::ControlBall;
use crate::components::com_collide::Collide;
use crate::systems::sys_draw2d::sys_draw2d;
use crate::systems::sys_transform2d::sys_transform2d;
use crate::systems::sys_move::sys_move;
use crate::systems::sys_control_ball::sys_control_ball;
use crate::systems::sys_collide::sys_collide;
use crate::systems::sys_control_block::sys_control_block;
use crate::systems::sys_control_paddle::sys_control_paddle;

pub const MAX_ENTITIES: usize = 10000;

pub struct Game {
    pub world: Vec<u32>,

    pub window_width: u32,
    pub window_height: u32,

    // pub window: sdl2::video::Window,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,

    pub clear_color: [u8; 4],

    pub input_state: Vec<Option<bool>>,

    pub transform: Vec<Option<Transform2d>>,
    pub draw2d: Vec<Option<Draw2d>>,
    pub move_component: Vec<Option<Move>>, // 'move' is a reserved keyword in rust
    pub control_ball: Vec<Option<ControlBall>>,
    pub collide: Vec<Option<Collide>>,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32) -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rs-breakout", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window.into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Game {
            world: vec![0; MAX_ENTITIES],

            window_width,
            window_height,

            // window,
            canvas,
            event_pump,

            clear_color: [0, 0, 0, 255],

            // Are there more than 200 keys?
            input_state: vec![None; 200],

            transform: vec![None; MAX_ENTITIES],
            draw2d: vec![None; MAX_ENTITIES],
            move_component: vec![None; MAX_ENTITIES],
            control_ball: vec![None; MAX_ENTITIES],
            collide: vec![None; MAX_ENTITIES],
        }
    }

    fn create_entity(&mut self) -> usize {
        for i in 0..MAX_ENTITIES {
            if self.world[i] == 0 {
                return i;
            }
        }

        panic!("No more entities available!");
    }

   pub fn add(&mut self, blueprint: &mut Blueprint) -> usize {
        let entity = self.create_entity();
        let transform_mixin = Transform2d::new(blueprint.translation, blueprint.rotation, blueprint.scale);
        transform_mixin(self, entity);

        for mixin in blueprint.using.iter_mut() {
            mixin(self, entity);
        }

        entity
    }

    pub fn destroy(&mut self, entity: usize) {
        self.world[entity] = 0;
    }

    pub fn update(&mut self, delta: f32) {
        sys_control_ball(self, delta);
        sys_control_block(self, delta);
        sys_control_paddle(self, delta);
        sys_move(self, delta);
        sys_transform2d(self, delta);
        sys_collide(self, delta);
        sys_draw2d(self, delta);
    }

    pub fn start(&mut self) {

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    // TODO: FIXME!
                    Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                        self.input_state[0] = Some(true);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                        self.input_state[1] = Some(true);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                        self.input_state[0] = None;
                    },
                    Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                        self.input_state[1] = None;
                    },
                    _ => {}
                }
            }

            self.update(0.16);

            self.canvas.present();
        }
    }
}
