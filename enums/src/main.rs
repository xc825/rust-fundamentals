#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Abava,
}

fn wine_popularity(w: &WineRegions) {
    match w {
        WineRegions::Bordeaux => println!("Bordeaux is popular!"),
        WineRegions::Burgundy => println!("Burgundy is popular!"),
        WineRegions::Champagne => println!("Champagne is popular!"),
        WineRegions::Tuscany => println!("Tuscany is popular!"),
        WineRegions::Rioja => println!("Rioja is popular!"),
        WineRegions::NapaValley => println!("Napa Valley is popular!"),
        WineRegions::Abava => println!("Abava is popular!"),
    }
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: &WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        WineRegions::Abava => println!("Abava is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Abavas Vīns"),
        region: WineRegions::Abava,
    };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(&wine1.region);
    supported_regions(&WineRegions::Rioja);
    supported_regions(&wine3.region);

    wine_popularity(&wine3.region);
    wine_popularity(&wine2.region);
}
