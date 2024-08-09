use moji_odpadki_parser::{calendar::*, search::*};
use std::io::Read;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let mut term = String::new();
    let mut streets: Vec<Street>;
    loop {
        println!("Poišči ulico (min. 2 znaka):");
        let term = loop {
            std::io::stdin().read_line(&mut term).unwrap();
            if let Some(term) = term.lines().last() {
                if term.chars().count() >= 2 {
                    break term;
                }
            }
            println!("Prosim vnesi izraz daljši od 2 znakov!");
        };
        println!("\nIskanje \"{}\" ...\n\n", term);

        let (url, req) = get_street_search_url_and_request(Some(term));
        let mut res = client.post(url).body(req).send()?;
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        let body = body.as_str();
        //let body: serde_json::Value = serde_json::from_str(body)?;
        let response: Response<Street> = serde_json::from_str(body)?;

        streets = response.result;

        if !streets.is_empty() {
            break;
        }

        println!("Ni rezultatov iskanja.\n");
    }

    println!("Rezultati iskanja:\n");

    for (i, street) in streets.iter().clone().enumerate() {
        println!("{:3}| {}", i + 1, street.label);
    }

    println!("\nProsim vnesi zaporedno številko pred izbrano ulico:");

    let mut street = String::new();
    let street: usize = loop {
        std::io::stdin().read_line(&mut street).unwrap();
        if let Some(street) = street.lines().last() {
            if let Ok(street) = street.parse::<usize>() {
                if street > 0 && street < streets.len() + 1 {
                    break street;
                }
            }
        }
        println!("Prosim vnesi veljavno številko!");
    } - 1;

    let street = streets[street].clone();

    println!("\nIzbrana ulica: {}\n", street.label);

    let (url, req) = get_address_search_url_and_request(street.id);
    let mut res = client.post(url).body(req).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let body = body.as_str();
    let response: Response<Address> = serde_json::from_str(body)?;

    let addresses: Vec<Address> = response.result;

    println!("Naslovi v tej ulici:");

    for address in addresses.iter() {
        println!(
            "{} {}{}, {}",
            address.street,
            address.number,
            address.addition,
            address.municipality
        );
    }

    println!("\nProsim vnesi hišno številko:");

    let mut address = String::new();
    let address: Address = loop {
        std::io::stdin().read_line(&mut address).unwrap();
        if let Some(address) = address.lines().last() {
            if let Some(mid) = address.find(|c: char| !c.is_numeric()) {
                let (number, addition) = address.split_at(mid);
                let address: Vec<Address> = addresses
                    .iter()
                    .filter(|a| a.number == number)
                    .cloned()
                    .collect::<Vec<_>>();
                let mut address: Vec<Address> = address
                    .iter()
                    .filter(|a| a.addition.to_lowercase() == addition.to_lowercase())
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some(address) = address.pop() {
                    break address;
                }
            } else {
                let address: Vec<Address> = addresses
                    .iter()
                    .filter(|a| a.number == address)
                    .cloned()
                    .collect::<Vec<_>>();
                let mut address: Vec<Address> = address
                    .iter()
                    .filter(|a| a.addition.is_empty())
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some(address) = address.pop() {
                    break address;
                }
            }
        }
        println!("Prosim vnesi veljavno hišno številko!");
    };

    println!(
        "\n\n\n\n\nIzbrani naslov: {} {}{}, {}",
        address.street, address.number, address.addition, address.municipality
    );

    let uprn = address.id;
    let url = get_url(uprn);
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let body = body.as_str();

    let calendar = parse_html(body)?;

    println!("___________________________________________________________________________________________");
    for monthly in &calendar {
        print!(
            "{:30.30}",
            format!("|   {:5}     {:9}", monthly.year, monthly.month)
        );
    }
    print!("|");

    println!("\n|_____________________________|_____________________________|_____________________________|");

    for i in 0..31 {
        for monthly in &calendar {
            if let Some(Some(day)) = monthly.days.get(i) {
                let mut types: Vec<&str> = Vec::new();
                if day.mko {
                    types.push("MKO")
                };
                if day.emb {
                    types.push("EMB")
                };
                if day.bio {
                    types.push("BIO")
                };
                if day.pap {
                    types.push("PAP")
                };
                print!(
                    "{:30.30}",
                    format!(
                        "| {:2} {:.3}  {}",
                        i + 1,
                        format!("{}", day.diaw),
                        types.join(", ")
                    )
                );
            } else {
                print!("{:30.30}", "|")
            }
        }
        print!("|");
        if i != 30 {
            println!();
        }
    }
    println!("\n|_____________________________|_____________________________|_____________________________|");

    Ok(())
}
