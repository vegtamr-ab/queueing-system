use gnuplot::*;

pub fn construct_and_save(data: &Vec<f64>) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Final sim denyprob", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("% of requests processed", &[])
        .set_y_label("Deny probability", &[])
        .lines(
            &[0., 5., 10., 15., 20., 25., 30., 35., 40., 45., 50., 55., 60., 65., 70., 75., 80., 85., 90., 95., 100.],
            data.as_slice(),
            &[],
        );
    fg.save_to_png("target/plot/1.png",  800, 600);
}
