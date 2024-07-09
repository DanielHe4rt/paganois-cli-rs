use clap::Parser;
use cli_table::{Cell, Style, Table};
use cli_table::format::Justify;
use colored::Colorize;
use serde::{Deserialize, Serialize};

const CURRENCY_API_URL: &str = "https://economia.awesomeapi.com.br/json/last/USD";

const ASCII_LOGO: &str = r"
       +         *
     +++++    *****
    ++++++++********
    ++++++*###*******    :@@@@@                   @@%  @*       ==          =
   +++++*%%%%%##******   :@   @+:@=@@ #@#@@  @+@@ @@@@ @* @@*@@ @@ @@+@=    @  @@%@=
  ++++*%%%%%%%%%##****   :@@@%  *@@@@ @#  @ :@@@@ @@ @@@* @  .@ @@ .@@@:    @ %@  *@
  ++*%%%%%%%%%%%%%##***  :@     @@@=@%-@@@@ %@@%@%@@  @@* @@@@* @@ @@@@* @: @  @@@@.
 +*%%%%%%%%%%%%%%%%%##*               *@*@@
  %%%%%%%%%%%%%%%%%%%##
    %%%%%%%%%%%%%%%%%
      %%%%%%%%%%%%            ----------------  PagaNoisCLI  ---------------
         %%%%%%               --------------- by: danielhe4rt ---------------
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

  let table = vec![
    vec![
      format!("R$ {:.2}", final_salary).cell().justify(Justify::Center),
      format!("R$ {:.2}", args.last_bid).cell().justify(Justify::Center),
      format!("R$ {:.2}", exchange_rates.usd_brl.bid.parse::<f64>().unwrap()).cell().justify(Justify::Center),
      format!("R$ {:.2}", previous_salary).cell().justify(Justify::Center),
      format!("R$ {:.2}", final_less_half_percent).cell().justify(Justify::Center),
      format!("R$ {:.2}", salary_diff).cell().justify(Justify::Center),
    ],
  ]
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

  let husky_table = vec![
    vec![
      format!("R$ {:.2}", final_salary).cell().justify(Justify::Center),
      format!("R$ {:.2}", final_less_one_percent).cell().justify(Justify::Center),
      format!("R$ {:.2}", final_less_half_percent).cell().justify(Justify::Center),
      format!("R$ {:.2}", final_less_one_percent - final_less_half_percent).cell().justify(Justify::Center),
    ],
  ]
    .table()
    .title(vec![
      "Current Salary".cell().bold(true),
      "With 1% Discount".cell().bold(true),
      "With 0.5% Discount".cell().bold(true),
      "Discount Diff".cell().bold(true),
    ]);

  println!("{}", ASCII_LOGO);
  println!("{}", "Salary information:".bold());
  println!("{}", table.display().unwrap());
  println!("{}", "Husky information:".bold());
  println!("{}", husky_table.display().unwrap());
  if salary_diff > 0.0 {
    println!("{}", "Status:FAZ O L CARAIO".green());
  } else {
    println!("{}", "Status: DESFAÇA O L IMEDIATAMENTE".red());
  }

  println!("\n{}", "Dont forget to drink water!".cyan());
}


