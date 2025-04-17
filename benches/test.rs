use criterion::{Criterion, criterion_group, criterion_main};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{BlendMode, Canvas, CanvasBuilder, RenderTarget},
};
use std::hint::black_box;

use sdl2::video::{Window, WindowContext};
const SCREEN_WIDTH: u32 = 1000;
const _1000: u32 = 1000;

fn texture_c(w: u32, h: u32, c: &Canvas<Window>) {
    let t_creator = c.texture_creator();
    let mut draw_texture = t_creator
        .create_texture_target(t_creator.default_pixel_format(), w, h)
        .unwrap();
    draw_texture.set_blend_mode(BlendMode::Blend);
}

fn create_dif_canvases(c: &mut Criterion) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("foo", 1, 1)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;

    c.bench_function("texture create 1x1", |b| {
        b.iter(|| texture_c(black_box(1), black_box(1), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100", |b| {
        b.iter(|| texture_c(black_box(100), black_box(100), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100x2", |b| {
        b.iter(|| texture_c(black_box(100 * 2), black_box(100), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100x3", |b| {
        b.iter(|| texture_c(black_box(100 * 3), black_box(100), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100x4", |b| {
        b.iter(|| texture_c(black_box(100 * 4), black_box(100), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100x5x10", |b| {
        b.iter(|| texture_c(black_box(100 * 5), black_box(100 * 10), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100x50", |b| {
        b.iter(|| texture_c(black_box(100 * 50), black_box(100), black_box(&canvas)))
    });
    c.bench_function("texture create 100x100x100", |b| {
        b.iter(|| texture_c(black_box(100 * 10), black_box(100 * 10), black_box(&canvas)))
    });

    Ok(())
}

fn texture_draw_to_dif_canvases(c: &mut Criterion) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("foo", 1000, 1000)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;
    let t_creator = canvas.texture_creator();
    let mut draw_texture = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 100, 100)
        .unwrap();
    draw_texture.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_big = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 100 * 10, 100 * 10)
        .unwrap();
    draw_texture_big.set_blend_mode(BlendMode::Blend);
    c.bench_function("texture draw to 1000x1000", |b| {
        b.iter(|| {
            let _ = canvas.with_texture_canvas(&mut draw_texture, |c| {
                c.set_draw_color(Color::RGB(200, 200, 200));
                let _ = c.fill_rect(Rect::new(0, 0, black_box(20), black_box(20)));
            });
        })
    });

    c.bench_function("texture draw to 1000x1000x10", |b| {
        b.iter(|| {
            let _ = canvas.with_texture_canvas(&mut draw_texture_big, |c| {
                c.set_draw_color(Color::RGB(200, 200, 200));
                let _ = c.fill_rect(Rect::new(0, 0, black_box(20), black_box(20)));
            });
        })
    });

    Ok(())
}

fn texture_draw_cache(c: &mut Criterion) -> Result<(), String> {
    let mut g = c.benchmark_group("texture");
    g.measurement_time(std::time::Duration::from_secs(10));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("foo", 1000, 1000)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;
    let t_creator = canvas.texture_creator();
    let mut draw_texture_1 = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_1.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_2 = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_2.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_2_buffer = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_2_buffer.set_blend_mode(BlendMode::Blend);

    g.bench_function("texture 10x10 draw to 1000x1000", |b| {
        b.iter(|| {
            for i in 0..10 {
                canvas.with_texture_canvas(&mut draw_texture_1, |c| {
                    c.set_draw_color(Color::RGB(200, 200, 200));
                    c.fill_rect(Rect::new(0, 0, black_box(200), black_box(200)));
                    c.fill_rect(Rect::new(0, 0, black_box(230), black_box(200)));
                    c.fill_rect(Rect::new(0, 0, black_box(230), black_box(200)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(210)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(210)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(216)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(216)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(211)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(211)));
                    c.fill_rect(Rect::new(0, 0, black_box(235), black_box(211)));
                });
            }
        })
    });
    g.bench_function("texture 10 + 10 with cache draw to 1000x1000", |b| {
        b.iter(|| {
            canvas.with_texture_canvas(&mut draw_texture_2_buffer, |c| {
                c.set_draw_color(Color::RGB(200, 200, 200));
                c.fill_rect(Rect::new(0, 0, black_box(200), black_box(200)));
                c.fill_rect(Rect::new(0, 0, black_box(230), black_box(200)));
                c.fill_rect(Rect::new(0, 0, black_box(230), black_box(200)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(210)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(210)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(216)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(216)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(211)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(211)));
                c.fill_rect(Rect::new(0, 0, black_box(235), black_box(211)));
            });
            for i in 0..10 {
                canvas.with_texture_canvas(&mut draw_texture_2, |c| {
                    c.copy(&draw_texture_2_buffer, None, None);
                });
            }
        })
    });

    Ok(())
}

fn texture_draw_cache_2(c: &mut Criterion) -> Result<(), String> {
    let mut g = c.benchmark_group("texture");
    g.measurement_time(std::time::Duration::from_secs(20));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("foo", 1000, 1000)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;
    let t_creator = canvas.texture_creator();
    let mut draw_texture_1 = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_1.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_2 = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_2.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_2_buffer = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_2_buffer.set_blend_mode(BlendMode::Blend);

    g.bench_function("texture 10x100 draw to 1000x1000", |b| {
        b.iter(|| {
            for i in 0..100 {
                canvas.with_texture_canvas(&mut draw_texture_1, |c| {
                    c.set_draw_color(Color::RGB(200, 200, 200));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                });
            }
        })
    });
    g.bench_function("texture 10 + 100 with cache draw to 1000x1000", |b| {
        b.iter(|| {
            canvas.with_texture_canvas(&mut draw_texture_2_buffer, |c| {
                c.set_draw_color(Color::RGB(200, 200, 200));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(100)));
            });
            for i in 0..100 {
                canvas.with_texture_canvas(&mut draw_texture_2, |c| {
                    c.copy(&draw_texture_2_buffer, None, None);
                });
            }
        })
    });

    Ok(())
}

fn texture_draw_cache_3(c: &mut Criterion) -> Result<(), String> {
    let mut g = c.benchmark_group("texture");
    g.measurement_time(std::time::Duration::from_secs(10));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("foo", 1000, 1000)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = CanvasBuilder::new(window)
        .build()
        .map_err(|e| e.to_string())?;
    let t_creator = canvas.texture_creator();
    let mut draw_texture_1 = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_1.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_2 = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_2.set_blend_mode(BlendMode::Blend);

    let mut draw_texture_2_buffer = t_creator
        .create_texture_target(t_creator.default_pixel_format(), 1000, 1000)
        .unwrap();
    draw_texture_2_buffer.set_blend_mode(BlendMode::Blend);

    g.bench_function("texture 10x100 draw to 1000x1000", |b| {
        b.iter(|| {
            for i in 0..100 {
                canvas.with_texture_canvas(&mut draw_texture_1, |c| {
                    c.set_draw_color(Color::RGB(200, 200, 200));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                    c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                });
            }
        })
    });
    g.bench_function("texture 10 + 100 with cache draw to 1000x1000", |b| {
        b.iter(|| {
            canvas.with_texture_canvas(&mut draw_texture_2_buffer, |c| {
                c.set_draw_color(Color::RGB(200, 200, 200));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
                c.fill_rect(Rect::new(0, 0, black_box(1000), black_box(1000)));
            });
            for i in 0..100 {
                canvas.with_texture_canvas(&mut draw_texture_2, |c| {
                    c.copy(&draw_texture_2_buffer, None, None);
                });
            }
        })
    });

    Ok(())
}
fn get_res(c: &mut Criterion) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    c.bench_function("get res", |b| {
        b.iter(|| {
            video_subsystem.display_mode(black_box(0), black_box(0));
        })
    });

    Ok(())
}

const MAP_SIZE: usize = 10;
struct Map {
    map: [i32; MAP_SIZE],
}
impl Map {
    fn new() -> Map {
        let mut mp = [0; MAP_SIZE];
        mp[0] = 32;
        mp[1] = 63;
        mp[2] = 32;
        mp[3] = 63;
        mp[4] = 32;
        mp[5] = 63;
        mp[6] = 32;
        mp[7] = 63;
        mp[8] = 32;
        mp[9] = 63;
        Map { map: mp }
    }
    fn get(&self, index: usize) -> i32 {
        self.map[index]
    }
    fn set(&mut self, index: usize, val: i32) {
        self.map[index] = val;
    }
    fn new_fast() -> Map {
        Map {
            map: [32, 63, 32, 63, 32, 63, 32, 63, 32, 63],
        }
    }
    fn send(&mut self) {
        //
    }
}
fn get_map(c: &mut Criterion) -> Result<(), String> {
    c.bench_function("get map", |b| {
        b.iter(|| {
            let mut map = Map::new();
            map.get(black_box(5));
            map.set(black_box(5), black_box(35));
            black_box(map.send());
        })
    });
    c.bench_function("get map fast", |b| {
        b.iter(|| {
            let mut map = Map::new_fast();
            map.get(black_box(5));
            map.set(black_box(5), black_box(35));
            black_box(map.send());
        })
    });

    Ok(())
}
// bigger canvases takes longer to create O(n * 0.02 + C), n - number of pixels (w*h), C = .45 ms
// criterion_group!(benches, create_dif_canvases);

// bigger canvases takes the same amount of time to draw to, drawing the same, small object
// criterion_group!(benches, texture_draw_to_dif_canvases);

// copying 100x100 buffer takes the same amount of time as drawing 10x10 object 100 times
// conclusion:
// draw to buffer is faster, but only if there is overdraw
// copying has to be optimizes with (source - destanation) boundary
// criterion_group!(benches, texture_draw_cache);
// criterion_group!(benches, texture_draw_cache_2);
// criterion_group!(benches, texture_draw_cache_3);

// criterion_group!(benches, get_res);

criterion_group!(benches, get_map);
criterion_main!(benches);
