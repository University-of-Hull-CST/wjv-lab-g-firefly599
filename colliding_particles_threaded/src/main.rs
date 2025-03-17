#![allow(unused_imports)]
#![allow(dead_code)]
use rand::Rng;
use scoped_threadpool::Pool;
use scoped_threadpool::Scope; 
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

mod particle;
use particle::Particle;

struct ParticleSystem{
    particles: Vec<Particle>,
    collisions_atomic: Arc<AtomicUsize>, 
    collisions_i32: i32,
}

impl ParticleSystem {
    fn new() -> ParticleSystem {
        ParticleSystem {
            particles: Vec::new(),
            collisions_atomic: Arc::new(AtomicUsize::new(0)), //Arc::new(AtomicUsize::new(0)),
            collisions_i32: 0,
        }
    }

    fn create_particles(&mut self){
        let mut rng = rand::rng();

        for _i in 0..PARTICLE_MAX {
            let x = rng.random_range(0.0..=10.0);
            let y = rng.random_range(0.0..=10.0);
        
            self.particles.push(Particle::new(x, y));
        }   
    }

    fn run_simulation(&mut self){
        let mut _cycle = 0;
        let mut pool = scoped_threadpool::Pool::new(NUM_OF_THREADS as u32);
        for _ in 0..CYCLE_MAX {    
            self.move_particles_thread(&mut pool);
            
            self.check_collision();
            //self.check_collision_thread(&mut pool);
            _cycle += 1;
        }
    }

    fn move_particles_thread(&mut self, pool: &mut scoped_threadpool::Pool){
        pool.scoped(|scope| {
            for slice in self.particles.chunks_mut(PARTICLES_PER_THREAD){
                scope.execute(move || move_thread_main(slice));
            }            
        });
    }

    fn check_collision(&mut self){
        for i in 0..PARTICLE_MAX{
            for j in (i+1)..PARTICLE_MAX{
                let particle_1 = self.particles[i];
                let particle_2 = self.particles[j];

                if particle_1.collide(particle_2.x, particle_2.y){
                    self.collisions_i32 += 1;
                    //println!("Particle collision: {:.4} {:.4}\t{:.4} {:.4} {:.4} {:.4}", i, j, particle_1.x, particle_1.y, particle_2.x, particle_2.y); 
                }
            }
        }
    }

    fn check_collision_thread(&mut self, pool: &mut scoped_threadpool::Pool){
        pool.scoped(|scope| {
            for start in (0..PARTICLE_MAX).step_by(PARTICLES_PER_THREAD){
                let collisions_clone = Arc::clone(&self.collisions_atomic);
                scope.execute(move || collide_thread_main(start, &self.particles.clone(), collisions_clone));
            }
        });
    }

    fn print_particle_positions(&self){
        let mut counter: i32 = 0;
        let display_particle_count = 100;

        for part in &self.particles{
            if counter < display_particle_count{
                println!("Particle {}\tX:{:.4}\tY:{:.4}", counter, part.x, part.y);
            }
            else{
                return;
            }
            counter += 1;   
        }
    }
    
    fn print_collision_counter(&self){
        println!("Atomic Collisions: {}", self.collisions_atomic.load(Ordering::SeqCst));
        println!("i32 Collisions: {}", self.collisions_i32);
    }
}

pub fn  move_thread_main (particles: &mut [Particle]){
    let mut rng = rand::rng();
    for part in particles{
        let d_x = rng.random_range(0.0..1.0);
        let d_y = rng.random_range(0.0..1.0);

        part.update(d_x, d_y);
    }
}

pub fn collide_thread_main(start: usize, particles: &Vec<Particle>, collision: Arc<AtomicUsize>){
    for i in start..(start+PARTICLES_PER_THREAD){
        for j in (i+1)..PARTICLE_MAX{
            let particle_1 = particles[i];
            let particle_2 = particles[j];
            
            if particle_1.collide(particle_2.x, particle_2.y){
                collision.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}

const NUM_OF_THREADS: usize = 10;
const PARTICLE_MAX: usize = 10000;
const CYCLE_MAX: usize = 1000;
const PARTICLES_PER_THREAD: usize = PARTICLE_MAX / NUM_OF_THREADS;
const COLLISION_DISTANCE: f32 = 0.01;

fn main() {
    let mut system = ParticleSystem::new();
    let start = Instant::now();

    system.create_particles();
    system.run_simulation();

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    //system.print_particle_positions();
    system.print_collision_counter();
}