use plotters::prelude::*;
use plotters::style::text_anchor::{Pos, HPos, VPos};

use crate::queueing_system::{analytics, statistics, types};

const BOX_WIDTH: i32 = 50;
const BOX_HEIGHT: i32 = 30;

pub fn draw_sources(s: &types::Simulation) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("target/stepmodeui/states.png",
                                                            ((s.max_sources as u32 * BOX_WIDTH as u32), BOX_HEIGHT as u32))
                                                            .into_drawing_area();
    root.fill(&WHITE)?;

    for i in 0..s.max_sources {
        let pos = Pos::new(HPos::Center, VPos::Center);
        let style = TextStyle::from(("sans-serif", 20).into_font()).pos(pos).color(&BLACK);

        root.draw(&Rectangle::new([(i as i32 * BOX_WIDTH, 0), ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)], 
                                    &BLACK))?;

        root.draw(&Text::new(format!("{}", s.state.sources[i]), (i as i32 * BOX_WIDTH + BOX_WIDTH / 2, BOX_HEIGHT / 2), style))?;
    }

    Ok(())
}

pub fn draw_devices(s: &types::Simulation) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("target/stepmodeui/devices.png",
                                                            ((s.max_devices as u32 * BOX_WIDTH as u32), BOX_HEIGHT as u32))
                                                            .into_drawing_area();
    root.fill(&WHITE)?;

    for i in 0..s.max_devices {
        let mut border = Rectangle::new([(i as i32 * BOX_WIDTH, 0), 
                                                                   ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)], 
        &BLACK);
        
        if s.state.device_pointer == i {
            border = Rectangle::new([(i as i32 * BOX_WIDTH, 0), ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)], 
                                        BLUE.stroke_width(3));
        };

        if s.current_time >= s.state.devices[i] {
            root.draw(&border)?;
        } else {
            root.draw(&Rectangle::new([(i as i32 * BOX_WIDTH, 0), ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)],
                                        RED.filled()))?;
            root.draw(&border)?;
        }

        let pos = Pos::new(HPos::Center, VPos::Center);
        let style = TextStyle::from(("sans-serif", 20).into_font()).pos(pos).color(&BLACK);
        root.draw(&Text::new(format!("{}", s.state.devices[i]), (i as i32 * BOX_WIDTH + BOX_WIDTH / 2, BOX_HEIGHT / 2), style))?;
    }

    Ok(())
}

pub fn draw_buffer(s: &types::Simulation) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("target/stepmodeui/buffer.png",
                                                            ((s.max_buf_length as u32 * BOX_WIDTH as u32), BOX_HEIGHT as u32))
                                                            .into_drawing_area();
    root.fill(&WHITE)?;

    for i in 0..s.max_buf_length {
        let mut border = Rectangle::new([(i as i32 * BOX_WIDTH, 0), 
                                                                   ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)], 
        &BLACK);
        
        if s.state.buf_pointer == i {
            border = Rectangle::new([(i as i32 * BOX_WIDTH, 0), ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)], 
                                        BLUE.stroke_width(3));
        };

        if s.state.buf[i].is_some() {
            let pos = Pos::new(HPos::Center, VPos::Center);
            let style = TextStyle::from(("sans-serif", 20).into_font()).pos(pos).color(&BLACK);
            root.draw(&Rectangle::new([(i as i32 * BOX_WIDTH, 0), ((i + 1) as i32 * BOX_WIDTH - 1, BOX_HEIGHT - 1)], 
                                        RED.filled()))?;
            root.draw(&border)?;

            root.draw(&Text::new(format!("{}", s.state.buf[i].unwrap().time_arrived), (i as i32 * BOX_WIDTH + BOX_WIDTH / 2, BOX_HEIGHT / 2), style))?;
        } else {
            root.draw(&border)?;
        }
    }

    Ok(())
}