use ansi_term::Colour;
use clap::Parser;
use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use colored::Colorize;
use serde::{Deserialize, Serialize};

const CURRENCY_API_URL: &str = "https://economia.awesomeapi.com.br/json/last/USD";

const ASCII_LOGO: &str = r"
       +         *
     +++++    *****
    ++++++++********
    ++++++*###*******     /$$$$$$$                                /$$   /$$           /$$               /$$
   +++++*%%%%%##******   | $$__  $$                              | $$$ | $$          |__/              |__/
  ++++*%%%%%%%%%##****   | $$  \ $$  /$$$$$$   /$$$$$$   /$$$$$$ | $$$$| $$  /$$$$$$  /$$  /$$$$$$$     /$$  /$$$$$$
  ++*%%%%%%%%%%%%%##***  | $$$$$$$/ |____  $$ /$$__  $$ |____  $$| $$ $$ $$ /$$__  $$| $$ /$$_____/    | $$ /$$__  $$
 +*%%%%%%%%%%%%%%%%%##*  | $$____/   /$$$$$$$| $$  \ $$  /$$$$$$$| $$  $$$$| $$  \ $$| $$|  $$$$$$     | $$| $$  \ $$
  %%%%%%%%%%%%%%%%%%%##  | $$       /$$__  $$| $$  | $$ /$$__  $$| $$\  $$$| $$  | $$| $$ \____  $$    | $$| $$  | $$
    %%%%%%%%%%%%%%%%%    | $$      |  $$$$$$$|  $$$$$$$|  $$$$$$$| $$ \  $$|  $$$$$$/| $$ /$$$$$$$/ /$$| $$|  $$$$$$/
      %%%%%%%%%%%%       |__/       \_______/ \____  $$ \_______/|__/  \__/ \______/ |__/|_______/ |__/|__/ \______/
         %%%%%%                               /$$  \ $$
                                             |  $$$$$$/     ----------------  PagaNoisCLI  ----------------
                                              \______/      --------------- by: danielhe4rt ---------------
";

#[derive(Parser, Debug)]
enum Mode {
    Terminal,
    Push,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    mode: Mode,

    /// Your current salary
    #[arg(short, long, default_value_t = 5000)]
    salary: u16,

    #[arg(short, long, default_value_t = 5.505)]
    last_bid: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeRate {
    code: String,
    #[serde(rename = "codein")]
    code_in: String,
    name: String,
    high: String,
    low: String,
    #[serde(rename = "varBid")]
    var_bid: String,
    #[serde(rename = "pctChange")]
    pct_change: String,
    bid: String,
    ask: String,
    timestamp: String,
    create_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeRates {
    #[serde(rename = "USDBRL")]
    usd_brl: ExchangeRate,
}

fn display_logo() {
    let lines: Vec<String> = ASCII_LOGO.lines().map(|s| s.to_string()).collect();
    let num_lines = lines.len();

    // Define the gradient colors based on the logo
    let start_color = (203, 43, 125); // Dark purple
    let mid_color = (243, 63, 113); // Pink
    let end_color = (254, 85, 53); // Red-orange

    for (i, line) in lines.iter().enumerate() {
        let ratio = i as f32 / (num_lines - 1) as f32;

        let (r, g, b) = if ratio < 0.5 {
            let ratio = ratio * 2.0;
            (
                (start_color.0 as f32 * (1.0 - ratio) + mid_color.0 as f32 * ratio).round() as u8,
                (start_color.1 as f32 * (1.0 - ratio) + mid_color.1 as f32 * ratio).round() as u8,
                (start_color.2 as f32 * (1.0 - ratio) + mid_color.2 as f32 * ratio).round() as u8,
            )
        } else {
            let ratio = (ratio - 0.5) * 2.0;
            (
                (mid_color.0 as f32 * (1.0 - ratio) + end_color.0 as f32 * ratio).round() as u8,
                (mid_color.1 as f32 * (1.0 - ratio) + end_color.1 as f32 * ratio).round() as u8,
                (mid_color.2 as f32 * (1.0 - ratio) + end_color.2 as f32 * ratio).round() as u8,
            )
        };

        let color = Colour::RGB(r, g, b);
        println!("{}", color.paint(line));
    }
}

fn main() {
    let args = Args::parse();

    // TODO: Implement mode selection
    // match args.mode {
    //   Mode::Terminal => {
    //     println!("Terminal mode");
    //   }
    //   Mode::Push => {
    //     println!("Push mode");
    //   }
    // }

    let response = reqwest::blocking::get(CURRENCY_API_URL).unwrap();
    let exchange_rates: ExchangeRates = response.json().unwrap();

    let final_salary = args.salary as f64 * exchange_rates.usd_brl.bid.parse::<f64>().unwrap();
    let previous_salary = args.salary as f64 * args.last_bid;

    let final_less_one_percent = final_salary * 0.99;
    let final_less_half_percent = final_salary * 0.995;
    let salary_diff = final_salary - previous_salary;

    let table = vec![vec![
        format!("R$ {:.2}", final_salary)
            .cell()
            .justify(Justify::Center),
        format!("R$ {:.2}", args.last_bid)
            .cell()
            .justify(Justify::Center),
        format!(
            "R$ {:.2}",
            exchange_rates.usd_brl.bid.parse::<f64>().unwrap()
        )
        .cell()
        .justify(Justify::Center),
        format!("R$ {:.2}", previous_salary)
            .cell()
            .justify(Justify::Center),
        format!("R$ {:.2}", final_less_half_percent)
            .cell()
            .justify(Justify::Center),
        format!("R$ {:.2}", salary_diff)
            .cell()
            .justify(Justify::Center),
    ]]
    .table()
    .title(vec![
        "Current Salary".cell().bold(true),
        "Last Payment BID".cell().bold(true),
        "Current BID".cell().bold(true),
        "Previous Salary".cell().bold(true),
        "Upcoming Salary".cell().bold(true),
        "BID Salary Diff".cell().bold(true),
    ])
    .bold(true);

    let husky_table = vec![vec![
        format!("R$ {:.2}", final_salary)
            .cell()
            .justify(Justify::Center),
        format!("R$ {:.2}", final_less_one_percent)
            .cell()
            .justify(Justify::Center),
        format!("R$ {:.2}", final_less_half_percent)
            .cell()
            .justify(Justify::Center),
        format!("R$ {:.2}", final_less_one_percent - final_less_half_percent)
            .cell()
            .justify(Justify::Center),
    ]]
    .table()
    .title(vec![
        "Current Salary".cell().bold(true),
        "With 1% Discount".cell().bold(true),
        "With 0.5% Discount".cell().bold(true),
        "Discount Diff".cell().bold(true),
    ]);

    display_logo();
    println!("{}", "Salary information:".bold());
    println!("{}", table.display().unwrap());
    println!("{}", "Husky information:".bold());
    println!("{}", husky_table.display().unwrap());

    if salary_diff > 0.0 {
        println!("{}", "Status: FAZ O L CARAIO".green());
    } else {
        println!("{}", "Status: DESFAÇA O L IMEDIATAMENTE".red());
    }

    println!("\n{}", "Dont forget to drink water!".cyan());
}
