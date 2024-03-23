use std::time::Duration;

const THRESHOLD_PERCENTAGE: f64 = 0.4;

fn get_average(data: &Vec<u128>) -> u128 {
    let mut average: u128 = 0;
    for data_point in data {
        average += data_point;
    }
    average / data.len() as u128
}

pub fn make_stats(title: &String, data: &Vec<Duration>, show_extras: bool) {
    let mut micros: Vec<u128> = data.iter().map(|x| x.as_micros()).collect();
    micros.sort_unstable();
    // println!("micros: {:?}", micros);

    // absolute min in the initial data
    let min_duration = micros[0];
    // absolute max in the initial data
    let max_duration = micros[micros.len() - 1];
    // median in the initial data
    let median = micros[micros.len() / 2];
    // average of the initial data
    let average = get_average(&micros);

    // Calculate the cutoff index
    let cutoff = ((micros.len() as f64) * THRESHOLD_PERCENTAGE).ceil() as usize;

    // the first x% of the original values
    // let micros: Vec<u128> = Vec::from(&micros[..cutoff]);
    let micros = &micros[..cutoff];

    // println!("micros, cutoff at {}: {:?}", cutoff, micros);

    let median_index = micros.len() / 2;
    let median_cutoff = micros[median_index];

    // absolute max in the cut-off data
    let max_duration_cutoff = micros[micros.len() - 1];

    let mut average_cutoff: u128 = 0;
    for duration in micros {
        average_cutoff += duration;
    }
    average_cutoff = average_cutoff / micros.len() as u128;

    let line = "=".repeat(title.len()+2);

    println!("{}\n {}\n{}",line, title,line);

    if show_extras{
    println!(
        "ORIGINAL DATA [{} points], microseconds\n\tmin: {}\n\tmax: {}\n\tmedian: {}\n\taverage: {}\n",
        data.len(),min_duration, max_duration, median, average
    );
    }
    println!(
        "CUT-OFF DATA  [{} of {} = {} points], microseconds\n\tmin: {}\n\tmax: {}\n\tmedian: {}\n\taverage: {}\n",
        THRESHOLD_PERCENTAGE,data.len(), micros.len(), min_duration, max_duration_cutoff, median_cutoff, average_cutoff
    );

}
