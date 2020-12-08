use gnuplot::*;

pub fn construct_and_save(xdata: &Vec<f64>, ydata: &Vec<f64>, title: &str, xlabel: &str, ylabel: &str, filename: &str) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label(xlabel, &[])
        .set_y_label(ylabel, &[])
        .lines(
            xdata.as_slice(),
            ydata.as_slice(),
            &[],
        );
    fg.save_to_png(filename,  800, 600);
}
