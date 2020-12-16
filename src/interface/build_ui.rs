use gtk::prelude::*;

use gio::*;
use glib::*;
use gtk::*;

use crate::queueing_system::{analytics, statistics, types, simulation};

use super::plotting::*;
use super::state_draw::*;
use std::{rc::Rc, cell::RefCell, fs, cmp};

pub fn get_confidence(str: &str) -> Result<types::ConfidenceLevel, &str> {
    match str {
        "Standard" => Ok(types::ConfidenceLevel::Standard),
        "High" => Ok(types::ConfidenceLevel::High),
        "Very High" => Ok(types::ConfidenceLevel::VeryHigh),
        _ => Err("Invalid confidence level"),
    }
}

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../glade/main.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("mainwindow").expect("Couldn't get mainwindow");
    window.set_application(Some(application));

    window.show_all();

    let clear_b: Button = builder.get_object("clear_b").expect("NO");
    clear_b.connect_clicked(clone!(@weak window => move |_| {
        if let Ok(dir) = fs::read_dir("target/plot") {
            for entry in dir {
                if let Ok(entry) = entry {
                    fs::remove_file(entry.path());
                }
            }
        }
    }));
    clear_b.emit_clicked();

    let quit_b: Button = builder.get_object("quit_b").expect("NO");
    quit_b.connect_clicked(clone!(@weak window => move |_| {
        window.get_application().unwrap().quit();
    }));

    let sim_button: Button = builder.get_object("sim_button").expect("Couldn't get go button");
    let deny_radio: RadioButton = builder.get_object("deny_radio").expect("NO");
    let reqtime_radio: RadioButton = builder.get_object("reqtime_radio").expect("NO");
    let usage_radio: RadioButton = builder.get_object("usage_radio").expect("NO");

    let simulations: Rc<RefCell<Vec<types::Simulation>>> = Rc::new(RefCell::new(Vec::new()));
    let final_n: Rc<RefCell<usize>> = Rc::new(RefCell::new(100));

    let btn_bld = builder.clone();
    let simulations_btn_gen = simulations.clone();
    let final_n_btn_gen = final_n.clone();

    sim_button.connect_clicked(clone!(@weak window => move |_| {
        let numsrc: Entry = btn_bld.get_object("numsrc").expect("Couldn't get numsrc");
        let numdvc: Entry = btn_bld.get_object("numdvc").expect("Couldn't get numdvc");
        let bufcap: Entry = btn_bld.get_object("bufcap").expect("Couldn't get bufcap");
        let avgsrc: Entry = btn_bld.get_object("avgsrc").expect("Couldn't get avgsrc");
        let avgdvc: Entry = btn_bld.get_object("avgdvc").expect("Couldn't get avgdvc");
        let confidence: ComboBox = btn_bld.get_object("confidence").expect("Couldn't get avgdvc");

        let inp = types::UserInput {
            n_src: numsrc.get_text().as_str().parse::<usize>().unwrap(),
            n_dvc: numdvc.get_text().as_str().parse::<usize>().unwrap(),
            n_buf: bufcap.get_text().as_str().parse::<usize>().unwrap(),
            avg_src: avgsrc.get_text().as_str().parse::<u64>().unwrap(),
            avg_dvc: avgdvc.get_text().as_str().parse::<u64>().unwrap(),
        };
        let confidence_level = get_confidence(confidence.get_active_id().unwrap().as_str()).unwrap();

        let (sims, fin_n) = analytics::get_res(confidence_level, 100, inp, None);
        *simulations_btn_gen.borrow_mut() = sims;
        *final_n_btn_gen.borrow_mut() = fin_n;
        if simulations_btn_gen.borrow().len() == 21 {
            simulations_btn_gen.borrow_mut().remove(0);
        }

        let mut x_marks: Vec<f64> = vec![0.0; 21];
        x_marks = x_marks.iter()
                         .enumerate()
                         .map(|(i, _)| (i * 5) as f64)
                         .collect();

        let mut image: Image = btn_bld.get_object("auto_denygraph").expect("No");
        let mut radio: RadioButton = btn_bld.get_object("deny_radio").expect("NO");
        if !radio.get_active() {
            image.hide();
        }
        let mut deny_probs: Vec<f64> = simulations_btn_gen.borrow().iter()
                                                  .map(|x| statistics::deny_probability(x))
                                                  .collect();
        deny_probs.insert(0, 0.);

        plot(&x_marks, &deny_probs, "Deny prob", "% of reqs processed", "Deny prob", "target/plot/auto_deny.png", 800, 600);
        image.set_from_file("target/plot/auto_deny.png");

        image = btn_bld.get_object("auto_reqtimegraph").expect("No");
        radio = btn_bld.get_object("reqtime_radio").expect("NO");
        if !radio.get_active() {
            image.hide();
        }
        let mut avgreq_times: Vec<f64> = simulations_btn_gen.borrow().iter()
                                                    .map(|x| statistics::average_request_time_in_system(x))
                                                    .collect();
        avgreq_times.insert(0, 0.);

        plot(&x_marks, &avgreq_times, "Avg req time in sys", "% of reqs processed", "Avg req time in sys", "target/plot/auto_reqtime.png", 800, 600);
        image.set_from_file("target/plot/auto_reqtime.png");

        image = btn_bld.get_object("auto_usagegraph").expect("No");
        radio = btn_bld.get_object("usage_radio").expect("NO");
        if !radio.get_active() {
            image.hide();
        }
        let mut usage_coeffs: Vec<f64> = simulations_btn_gen.borrow().iter()
                                                .map(|x| statistics::usage_coefficient(x))
                                                .collect();
        usage_coeffs.insert(0, 0.);

        plot(&x_marks, &usage_coeffs, "Usage coeff", "% of reqs processed", "Usage coeff", "target/plot/auto_usage.png", 800, 600);
        image.set_from_file("target/plot/auto_usage.png");
    }));

    let raddeny_bld = builder.clone();
    deny_radio.connect_toggled(clone!(@weak window => move |radio| {
        let image: Image = raddeny_bld.get_object("auto_denygraph").expect("NO");
        if radio.get_active() {
            image.show();
        } else {
            image.hide();
        }
    }));

    let radreq_bld = builder.clone();
    reqtime_radio.connect_toggled(clone!(@weak window => move |radio| {
        let image: Image = radreq_bld.get_object("auto_reqtimegraph").expect("NO");
        if radio.get_active() {
            image.show();
        } else {
            image.hide();
        }
    }));

    let radusg_bld = builder.clone();
    usage_radio.connect_toggled(clone!(@weak window => move |radio| {
        let image: Image = radusg_bld.get_object("auto_usagegraph").expect("NO");
        if radio.get_active() {
            image.show();
        } else {
            image.hide();
        }
    }));

    let test_button: Button = builder.get_object("test_button").expect("NO");
    let tstbtn_bld = builder.clone();
    test_button.connect_clicked(clone!(@weak window => move |_| {
        let numsrc: Entry = tstbtn_bld.get_object("numsrc").expect("Couldn't get numsrc");
        let numdvc: Entry = tstbtn_bld.get_object("numdvc").expect("Couldn't get numdvc");
        let bufcap: Entry = tstbtn_bld.get_object("bufcap").expect("Couldn't get bufcap");
        let avgsrc: Entry = tstbtn_bld.get_object("avgsrc").expect("Couldn't get avgsrc");
        let avgdvc: Entry = tstbtn_bld.get_object("avgdvc").expect("Couldn't get avgdvc");

        numsrc.set_text("5");
        numdvc.set_text("6");
        bufcap.set_text("3");
        avgsrc.set_text("320");
        avgdvc.set_text("560");
    }));

    let ind_src_button: Button = builder.get_object("src_button").expect("NO");
    let ind_src_window: Window = builder.get_object("indsrcwindow").expect("Couldn't get");

    let srcopen_window = ind_src_window.clone();
    ind_src_button.connect_clicked(clone!(@weak window => move |_| {
        srcopen_window.show_all();
    }));

    let exit_src: Button = builder.get_object("exit_src").expect("NO");
    let srcexit_window = ind_src_window.clone();
    exit_src.connect_clicked(clone!(@weak window => move |_| {
        srcexit_window.hide();
    }));

    // let closesrc_window = ind_src_window.clone();
    // ind_src_window.connect_delete_event(clone!(@weak window => move |_, _| {
    //     closesrc_window.hide();
    //     Inhibit{ 0: true }
    // }));

    let show_button: Button = builder.get_object("show_button").expect("NO");
    let shwbtn_bld = builder.clone();
    let simulations_indsrc = simulations.clone();
    let final_indsrc = final_n.clone();
    show_button.connect_clicked(clone!(@weak window => move |_| {
        let srcnum: Entry = shwbtn_bld.get_object("srcnum").expect("Couldn't get srcnum");
        let mut num = srcnum.get_text().as_str().parse::<usize>().unwrap();
        if num == 0 || num > simulations_indsrc.borrow()[0].max_sources {
           //error 
        } else {
            num = num - 1;

            let topl_image: Image = shwbtn_bld.get_object("indsrc_denygraph").expect("NO");
            let topr_image: Image = shwbtn_bld.get_object("indsrc_reqtimegraph").expect("NO");
            let btml_image: Image = shwbtn_bld.get_object("indsrc_buftimegraph").expect("NO");
            let btmr_image: Image = shwbtn_bld.get_object("indsrc_proctimegraph").expect("NO");

            let mut x_marks: Vec<f64> = vec![0.0; 21];
            x_marks = x_marks.iter()
                            .enumerate()
                            .map(|(i, _)| (i * 5) as f64)
                            .collect();

            let mut src_deny_probs: Vec<f64> = simulations_indsrc.borrow().iter()
                                                    .map(|x| statistics::src_deny_probability(x, num))
                                                    .collect();
            src_deny_probs.insert(0, 0.);
            plot(&x_marks, &src_deny_probs, "Deny prob",
                                            "% of reqs processed",
                                            "Deny prob",
                                            format!("target/plot/auto_src_deny{}.png", num).as_str(),
                                            400, 300);

            let mut src_req_times: Vec<f64> = simulations_indsrc.borrow().iter()
                                                    .map(|x| statistics::src_avg_request_time_in_system(x, num))
                                                    .collect();
            src_req_times.insert(0, 0.);
            plot(&x_marks, &src_req_times, "Avg req time in sys",
                                        "% of reqs processed",
                                        "Avg req time in sys",
                                        format!("target/plot/auto_src_reqt{}.png", num).as_str(),
                                        400, 300);

            let mut src_buf_times: Vec<f64> = simulations_indsrc.borrow().iter()
                                                    .map(|x| statistics::src_avg_request_time_in_buffer(x, num))
                                                    .collect();
            src_buf_times.insert(0, 0.);
            plot(&x_marks, &src_buf_times, "Avg req time in buf",
                                        "% of reqs processed",
                                        "Avg req time in buf",
                                        format!("target/plot/auto_src_buft{}.png", num).as_str(),
                                        400, 300);

            let mut src_proc_times: Vec<f64> = simulations_indsrc.borrow().iter()
                                                    .map(|x| statistics::src_avg_devices_busy(x, num))
                                                    .collect();
            src_proc_times.insert(0, 0.);
            plot(&x_marks, &src_proc_times, "Avg req time processing",
                                            "% of reqs processed",
                                            "Avg req time processing",
                                            format!("target/plot/auto_src_proc{}.png", num).as_str(),
                                            400, 300);

            topl_image.set_from_file(format!("target/plot/auto_src_deny{}.png", num).as_str());
            topr_image.set_from_file(format!("target/plot/auto_src_reqt{}.png", num).as_str());
            btml_image.set_from_file(format!("target/plot/auto_src_buft{}.png", num).as_str());
            btmr_image.set_from_file(format!("target/plot/auto_src_proc{}.png", num).as_str());

            let reqproc: Label = shwbtn_bld.get_object("reqstatl").expect("NO");
            reqproc.set_text(format!("Requests generated: {} out of {}",
                                     simulations_indsrc.borrow()[19].state.s_requests_count[num],
                                     final_indsrc.borrow()).as_str());
        };
    }));

    let simulation: Rc<RefCell<Option<types::Simulation>>> = Rc::new(RefCell::new(None));

    let stepmode_button: Button = builder.get_object("stepmode_button").expect("NO");
    let stepmode_window: Window = builder.get_object("stepmodewindow").expect("Couldn't get");

    let stepopen_window = stepmode_window.clone();
    let stepmode_init_sim = simulation.clone();
    let stepopen_bld = builder.clone();
    stepmode_button.connect_clicked(clone!(@weak window => move |_| {
        let numsrc: Entry = stepopen_bld.get_object("numsrc").expect("Couldn't get numsrc");
        let numdvc: Entry = stepopen_bld.get_object("numdvc").expect("Couldn't get numdvc");
        let bufcap: Entry = stepopen_bld.get_object("bufcap").expect("Couldn't get bufcap");
        let avgsrc: Entry = stepopen_bld.get_object("avgsrc").expect("Couldn't get avgsrc");
        let avgdvc: Entry = stepopen_bld.get_object("avgdvc").expect("Couldn't get avgdvc");

        let inp = types::UserInput {
            n_src: numsrc.get_text().as_str().parse::<usize>().unwrap(),
            n_dvc: numdvc.get_text().as_str().parse::<usize>().unwrap(),
            n_buf: bufcap.get_text().as_str().parse::<usize>().unwrap(),
            avg_src: avgsrc.get_text().as_str().parse::<u64>().unwrap(),
            avg_dvc: avgdvc.get_text().as_str().parse::<u64>().unwrap(),
        };

        *stepmode_init_sim.borrow_mut() = Some(analytics::base_simulation(*final_n.borrow(), inp));

        let width = 40 + cmp::max(cmp::max(inp.n_src, inp.n_dvc), inp.n_buf) * 50;
        let layout: Layout = stepopen_bld.get_object("stepmodelayout").expect("NO");
        layout.set_property_width(width as u32);
        // let exit_step: Button = stepopen_bld.get_object("exit_step").expect("NO");
        // layout.set_child_x(&exit_step, width as i32 - 50);
        
        stepopen_window.show_all();
    }));

    let step_button: Button = builder.get_object("step_button").expect("NO");
    let stpbtn_bld = builder.clone();

    let simulation_stepmode = simulation.clone();

    step_button.connect_clicked(clone!(@weak window => move |_| {
        let sim = simulation_stepmode.borrow().clone().unwrap();

        if sim.current_event != types::SimulationEvent::StopSimulation {
            *simulation_stepmode.borrow_mut() = Some(simulation::simulator(&sim));
        }

        draw_sources(&sim);
        let src_image: Image = stpbtn_bld.get_object("step_src").expect("NO");
        src_image.set_from_file("target/stepmodeui/states.png");

        draw_devices(&sim);
        let dvc_image: Image = stpbtn_bld.get_object("step_dvc").expect("NO");
        dvc_image.set_from_file("target/stepmodeui/devices.png");

        draw_buffer(&sim);
        let buf_image: Image = stpbtn_bld.get_object("step_buf").expect("NO");
        buf_image.set_from_file("target/stepmodeui/buffer.png");

        let reqleft: Label = stpbtn_bld.get_object("reqleftl").expect("NO");
        reqleft.set_text(format!("Requests left: {}, processed: {}, denied: {}",
                                 sim.state.requests_left,
                                 sim.state.requests_processed,
                                 sim.state.requests_denied).as_str());
        let curtime: Label = stpbtn_bld.get_object("curtimel").expect("NO");
        curtime.set_text(format!("Current time: {}", sim.current_time).as_str());
        let curevent: Label = stpbtn_bld.get_object("cureventl").expect("NO");
        curevent.set_text(format!("Current event: {:?}", sim.current_event).as_str());
    }));

    let exit_step: Button = builder.get_object("exit_step").expect("NO");
    let stepexit_window = stepmode_window.clone();
    exit_step.connect_clicked(clone!(@weak window => move |_| {
        stepexit_window.hide();
    }));
}
