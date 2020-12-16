use gnuplot::*;

pub fn plot(xdata: &Vec<f64>, ydata: &Vec<f64>, title: &str, xlabel: &str, ylabel: &str, filename: &str, width: u32, height: u32) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .set_x_label(xlabel, &[])
        .set_y_label(ylabel, &[])
        .lines_points(
            xdata.as_slice(),
            ydata.as_slice(),
            &[],
        );
    fg.save_to_png(filename,  width, height);
}
