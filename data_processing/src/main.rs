use std::ffi::OsString;
use std::env;
use std::error::Error;
use std::process;
use std::fs;


fn read_csv() -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut vals: Vec<(f64, f64)> = vec![];
    // record loop
    for result in rdr.deserialize() {
        let (x, mut y): (f64, Option<f64>) = result?;
        match y {
            Some(_) => (),
            None => y = Some(0.0),
        }
        vals.push((x, y.unwrap()))
    }
    Ok(vals)
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}


fn main() {
    let mut points: Vec<(f64, f64)>;
    let data = read_csv();
    match data {
        Err(err) => {
            println!("error: {}", err);
            process::exit(1)
        }
        Ok(vals) => points = vals,
    }

    // sort ascending by x values
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // innit variables for linear regression
    let size: f64 = points.len() as f64;
    let (mut sx, mut sy, mut sxy, mut ssx): (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

    // loop through data
    for point in points.iter() {
        sx += point.0;
        sy += point.1;
        sxy += point.0*point.1;
        ssx += point.0*point.0;
    }

    let m = {
        ( ( size * sxy ) - ( sx * sy ) ) / ( ( size * ssx ) - ( sx * sx ) )
    };

    let b = {
        ( sy * ssx -  sx * sxy ) / ( size * ssx - sx * sx )
    };

    let output = format!("{m},{b}");
    match fs::write(r".\bestfitLine.txt", output) {
        Err(err) => {
            println!("error: {}", err)
        },
        Ok(()) => (),
    }
}