use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::blueprints::blu_common::Blueprint;
use crate::components::com_move::Move;
use crate::components::com_draw2d::Draw2d;
use crate::components::com_transform2d::Transform2d;
use crate::components::com_controll_ball::ControlBall;
use crate::components::com_collide::Collide;
use crate::components::com_shake::Shake;
use crate::components::com_fade::Fade;
use crate::systems::sys_draw2d::sys_draw2d;
use crate::systems::sys_transform2d::sys_transform2d;
use crate::systems::sys_move::sys_move;
use crate::systems::sys_control_ball::sys_control_ball;
use crate::systems::sys_collide::sys_collide;
use crate::systems::sys_control_block::sys_control_block;
use crate::systems::sys_control_paddle::sys_control_paddle;
use crate::systems::sys_shake::sys_shake;
use crate::systems::sys_fade::sys_fade;
use crate::systems::sys_framerate::sys_framerate;

pub const MAX_ENTITIES: usize = 10000;
pub const MAX_CHILDREN: usize = 1000;

pub struct Game {
    pub world: Vec<u32>,
    pub camera: usize,

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
    pub shake: Vec<Option<Shake>>,
    pub fade: Vec<Option<Fade>>,
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

        let mut canvas = window.into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let event_pump = sdl_context.event_pump().unwrap();

        Game {
            world: vec![0; MAX_ENTITIES],
            camera: 0,

            window_width,
            window_height,

            // window,
            canvas,
            event_pump,

            clear_color: [0, 0, 0, 255],

            // Are there more than 300 keys?
            input_state: vec![None; 300],

            transform: vec![None; MAX_ENTITIES],
            draw2d: vec![None; MAX_ENTITIES],
            move_component: vec![None; MAX_ENTITIES],
            control_ball: vec![None; MAX_ENTITIES],
            collide: vec![None; MAX_ENTITIES],
            shake: std::iter::repeat_with(|| None).take(MAX_ENTITIES).collect(),
            fade: vec![None; MAX_ENTITIES],
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

   pub fn add(&mut self, mut blueprint: Blueprint) -> usize {
        let entity = self.create_entity();
        let transform_mixin = Transform2d::new(blueprint.translation, blueprint.rotation, blueprint.scale);
        transform_mixin(self, entity);

        for mixin in blueprint.using.iter_mut() {
            mixin(self, entity);
        }

        match blueprint.children {
            Some(children_list) => {
                let mut i = 0;
                for child in children_list {
                    if i >= MAX_CHILDREN {
                        panic!(format!("Reached max number of children per entity ({}). Modify settings in game.rs.", MAX_CHILDREN))
                    }
                    let child_id = self.add(child);

                    // XXX: It's safe to unwrap here, we know those values are there.
                    let mut child_transform = self.transform[child_id].unwrap();
                    child_transform.parent = Some(entity);
                    self.transform[child_id] = Some(child_transform);

                    let mut parent_transform = self.transform[entity].unwrap();
                    parent_transform.children[i] = Some(child_id);
                    self.transform[entity] = Some(parent_transform);

                    i += 1;
                }
            }
            None => {}
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
        sys_shake(self, delta);
        sys_fade(self, delta);
        sys_move(self, delta);
        sys_transform2d(self, delta);
        sys_collide(self, delta);
        sys_draw2d(self, delta);

        sys_framerate(self, delta);
    }

    pub fn start(&mut self) {
        let mut last_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    // TODO: FIXME!
                    Event::KeyDown { scancode: Some(key_code), repeat: false, .. } => {
                        self.input_state[key_code as usize] = Some(true);
                    },
                    Event::KeyUp { scancode: Some(key_code), repeat: false, .. } => {
                        self.input_state[key_code as usize] = None;
                    },
                    _ => {}
                }
            }

            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

            self.update(((now - last_time) as f32) / 1000.0);

            last_time = now;

            self.canvas.present();
        }
    }
}
