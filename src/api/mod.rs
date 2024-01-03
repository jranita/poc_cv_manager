pub mod clientcompany;
pub mod cv;
pub mod fileupload;
pub mod jobfunction;
pub mod keyword;
pub mod user;

use salvo::Depot;

pub fn sanitize_query_string(raw_string: String) -> String {
    raw_string
        .chars()
        .map(|x| match x.is_alphanumeric() {
            true => x,
            false => '_',
        })
        .collect()
}

// TODO improve filter to use other operators
pub fn string_to_filter(raw_string: String) -> String {
    let mut strvec: Vec<String> = raw_string
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| sanitize_query_string(x.to_string()))
        .collect();

    if strvec.len() % 2 != 0 {
        strvec.pop();
    }

    let mut filtervec: Vec<&str> = vec!["WHERE "];
    for (position, element) in strvec.iter().enumerate() {
        println!("+++++++{}+ +{}+", position, element);
        if position == 0 {
            filtervec.push(element);
            continue;
        }
        if position % 2 != 0 {
            filtervec.push(" LIKE '%");
            filtervec.push(element);
            filtervec.push("%'");
        }
        if position % 2 == 0 {
            filtervec.push(" AND ");
            filtervec.push(element);
        }

        //     match position%2 == 0 {
        //         true => filtervec.push(&element),
        //         false =>
        //     }
        //         filtervec.push(&element);
        //         filtervec.push(" LIKE '%");
        //         filtervec.push(&element);
        //     } else {
        //         filtervec.push("%' AND ");
        //     }
    }

    println!("++++++++++++++{:?}\n {}", filtervec, filtervec.join(""));

    filtervec.join("").to_string()
}
