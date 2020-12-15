use num_integer::Integer;

fn main() {
    //const DATA: &str = include_str!("../inputs/day13.txt");
    /*
    const DATA: &str = "17,x,13,19";
    let mut lines = DATA.lines();
    //let my_time = lines.next().unwrap().parse::<isize>().unwrap();
    let (times, buses): (Vec<isize>, Vec<isize>) = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        // throw away indices with x, and snap into 2 vecs
        .filter_map(|(t, s)| s.parse::<isize>().map(|num| (t as isize, num)).ok())
        .unzip::<_, _, Vec<_>, Vec<_>>();

    dbg!(&times);
    dbg!(&buses);
    */
    /*
    let times = vec![0, 2, 3];
    let buses = vec![17, 13, 19];
    */
    //let times = vec![0,2,3];
    //let buses = vec![17,13,19];
    //let data = "7,13,x,x,59,x,31,19";
    //let data = "7,13";

    // ^^ who needs tests, right?
    const DATA: &str = include_str!("../inputs/day13.txt");
    let data = DATA.lines().nth(1).unwrap();

    let (times, mut buses): (Vec<isize>, Vec<isize>) = data.lines()
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        // throw away indices with x, and snap into 2 vecs
        .filter_map(|(t, s)| s.parse::<isize>().map(|num| (t as isize, num)).ok())
        .unzip::<_, _, Vec<_>, Vec<_>>();
    dbg!(&times);
    dbg!(&buses);

    let big_n: isize = buses.iter().product();
    let lcms = buses
        .iter()
        .enumerate()
        .map(|(i, _bus)| {
            // multiply everything except buses[i] together
            // so -,1,2,3 then 0,-,2,3 then 0,1,-,3 etc
            buses[0..i].iter().product::<isize>() * buses[i + 1..].iter().product::<isize>()
        })
        .collect::<Vec<_>>();

    let coeffs = buses.iter().zip(lcms.iter())
        // giving us [(3, 140), (4, 105), (5, 84), (7, 60)]
        .map(|(&bus, product)| {
            // things to google:
            // chinese remainder theorem
            // modular inverse
            let gcd = product.extended_gcd(&bus);
            (gcd.x % bus + bus) % bus
        })
        .collect::<Vec<isize>>();

    let r = times.iter()
        .zip(lcms.iter())
        .zip(coeffs.iter())
        .map(|((&t, &lcm), &coeff)| {
            println!("({}, {}, {})", t, lcm, coeff);
            t * lcm * coeff
        })
        .sum::<isize>();


    println!("times:  {:?}", times);
    println!("lcms:   {:?}", lcms);
    println!("coeffs: {:?}", coeffs);
    println!("big_n: {:?}", big_n);
    println!("r: {:?}", r);
    println!("r % big_n: {:?}", r % big_n);
    println!("big_n - r: {:?}", big_n - r);
    println!("big_n - r % big_n: {:?}", big_n - r % big_n);
}