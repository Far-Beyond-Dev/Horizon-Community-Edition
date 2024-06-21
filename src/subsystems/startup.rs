pub fn main () {
    // ANSI color escape codes
    const WHITE: &str = "\x1b[0;37m";
    const RESET: &str = "\x1b[0m";

    // Define the ASCII art with placeholders for colors
    let art = r#"
    {white}             `  :y.`yy`.y:  `
    {white}         -``MNsNMMNNNNMMNsNM``-
    {white}      ` -MMNMMMMNNm``NNNMMMMNMM- `
    {white}     `NNNMMMdo:` `+md/  `:odMMMNNN`
    {white}   -ssNMMNo.                .oNMMNss-
    {white}   `mMM{red}MMNmmmmmmmmmmmmmdy+{white}     `sMMMm`
    {white} `mMMM{red}MMMMMMMMMMMMMMMMMMMMMMN/{white}  hMMMMm`
    {white} -oMN-:Ny:{red}mMMMMMm    oNMMMMMm{white} oN::MMo-
    {white}.yMMMhhh+ {red}dMMMMMd:::::+mMMMMN/{white} odyhMMMy.
    {white}-sNMMy    {red}dMMMMMMMMMMMMMMMMs{white}`    `yMMNs-
    {white}-sNMMy    {red}dMMMMMNyyyydMMMMMMy{white}   .odMMNs-
    {white}.yMMMm   {red}dMMMMMh     +MMMMMM+{white}  sMMMMMy.
    {white} -oMMM{red}MMMMMMMMMMMMMM+  mMMMMMMMMMM{white}MMMo-
    {white} `mMMM{red}MMMMMMMMMMMMMM+  :NMMMMMMMMM{white}MMMm`
    {white}   `mMMMm               `-:o+:/mMMMm`
    {white}   -ssNMMMyomo            smohMMMNss-
    {white}     `NNNMs+mN/-`      `-/Nd/yMNNN`
    {white}      ` -MMNMMMMMNmmmmNMMMMMNMM- `
    {white}         -``MNsNMMNMMNMMNsNM``-
    {white}            `  :y.`yy`.y:  `
    "#;

    // Replace placeholders with actual ANSI escape codes
    let colored_art = art.replace("{white}", WHITE).replace("{red}", "\x1b[0;31m");

    // Print the colored ASCII art
    print!("{}{}", colored_art, RESET);

    println!("");
    println!("Starting Horizon Server...");
    println!("");
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("|  __    __                      __                                       ______                                                     |");
    println!("| |  |  |  |                    |  |                                     /      |                                                    |");
    println!("| | $$  | $$  ______    ______   |$$ ________   ______   _______        |  $$$$$$|  ______    ______  __     __   ______    ______   |");
    println!("| | $$__| $$ /      |  /      | |  ||        | /      | |       |       | $$___|$$ /      |  /      ||  |   /  | /      |  /      |  |");
    println!("| | $$    $$|  $$$$$$||  $$$$$$|| $$ |$$$$$$$$|  $$$$$$|| $$$$$$$|       |$$    | |  $$$$$$||  $$$$$$||$$| /  $$|  $$$$$$||  $$$$$$| |");
    println!("| | $$$$$$$$| $$  | $$| $$   |$$| $$  /    $$ | $$  | $$| $$  | $$       _|$$$$$$|| $$    $$| $$   |$$ |$$|  $$ | $$    $$| $$   |$$ |");
    println!("| | $$  | $$| $$__/ $$| $$      | $$ /  $$$$_ | $$__/ $$| $$  | $$      |  |__| $$| $$$$$$$$| $$        |$$ $$  | $$$$$$$$| $$       |");
    println!("| | $$  | $$ |$$    $$| $$      | $$|  $$    | |$$    $$| $$  | $$       |$$    $$ |$$     || $$         |$$$    |$$     || $$       |");
    println!("|  |$$   |$$  |$$$$$$  |$$       |$$ |$$$$$$$$  |$$$$$$  |$$   |$$        |$$$$$$   |$$$$$$$ |$$          |$      |$$$$$$$ |$$       |");
    println!("|                                                               V: 0.0.4-A                                                           |");
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("|  ,---.   ,--.                            ,-----.                                   ,--. |");
    println!("| (   .-',-'  '-. ,--,--.,--.--. ,---.     |  |) /_  ,---. ,--. ,--.,---. ,--,--,  ,-|  | |");
    println!("|  `  `-.'-.  .-'| ,-.  ||  .--'(  .-'     |  .-.  || (===) |  '  /| .-. ||  ,,  |' .-. | |");
    println!("|  _)   |  |  |  | '-'  ||  | .-'  `)      |  '--' /|   --.  |   / ' '-' '|  ||  || `-' | |");
    println!("| (____/   `--'   `--`--'`--  `----'       `------'  `----'.-'  /   `---' `--''--' `---'  |");
    println!("|                                        V: 0.0.1          `---'                          |");
    println!("+-----------------------------------------------------------------------------------------+");
    println!("");
}