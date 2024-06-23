pub fn main () {
    // ANSI color escape codes
    const WHITE: &str = "\x1b[0;37m";
    const RESET: &str = "\x1b[0m";

    // Define the ASCII art with placeholders for colors
    let art = r#"
    {white}             `  :y.`yy.`yy.y`y:   `
    {white}         -``MNsNMMNNNMNNNNMNNMMNsNM``-
    {white}      ` -MMNMMMMNNm``Nm````NmNNMMMMNMM- `
    {white}     `NNNMMMdo:` `+md`+md`+/    `:odMMMNNN`
    {white}   -ssNMMNo.                        .oNMMNss-
    {white}   `mMM{red}MMNmmmmmmmmmmmmmmmmmdydyy+{white}       `sMMMm`
    {white} `mMMM{red}MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMN/{white}   hMMMMm`
    {white} -oMN-:Ny:{red}mMMMMMMMMm     oNMMMMMMMMMm{white}     ::MMo-
    {white}.yMMMhhh+ {red}dMMMMMMMMd::::::+mMmMMMMMMN/{white}    yhMMMy.
    {white}-sNMMy    {red}dMMMMMMMMMMMMMMMMMMMMMMMsM{white}`     `yMMNs-
    {white}-sNMMy    {red}dMMMMMMMMNyyyyydMMMMMMMMMMy{white}    .odMMNs-
    {white}.yMMMm   {red}dMMMMMMMMh      +MMMMMMMMMM+{white}    sMMMMMy.
    {white} -oMMM{red}MMMMMMMMMMMMMMMM+    mMmMMMMMMMMMMMM {white}MMMo-
    {white} `mMMM{red}MMMMMMMMMMMMMMMM+    :N:NNMMMNMMMMMM{white}MMMm`
    {white}   `mMMMm                       `---:o+:/mMMMm`
    {white}   -ssNMMMyomo                    smohMMMNss-
    {white}     `NNNMs+mN/-`              `-/Nd/yMNNN`
    {white}      ` -MMNMMMMMNmmmNmmmNmmmNMMMMMNMM- `
    {white}         -``MNsNMMNMMMNMMMNMMMMNsNM``-
    {white}            `  :y.`yy.`yy.`yy.y:  `
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
    println!("|                                                             V: 0.0.4-A l                                                           |");
    println!("+------------------------------------------------------------------------------------------------------------------------------------+");
    println!("");
    println!("+----------------------------------------------------------------------------------------+");
    println!("|  ,---.   ,--.                           ,-----.                                   ,--. |");
    println!("| (   .-',-'  '-. ,--,--.,--.--. ,---.    |  |) /_  ,---. ,--. ,--.,---. ,--,--,  ,-|  | |");
    println!("|  `  `-.'-.  .-'| ,-.  ||  .--'(  .-'    |  .-.  || (===) |  '  /| .-. ||  ,,  |' .-. | |");
    println!("|  _)   |  |  |  | '-'  ||  | .-'  `)     |  '--' /|   --.  |   / ' '-' '|  ||  || `-' | |");
    println!("| (____/   `--'   `--`--'`--  `----'      `------'  `----'.-'  /   `---' `--''--' `---'  |");
    println!("|                                       V: 0.0.1          `---'                          |");
    println!("+----------------------------------------------------------------------------------------+");
    println!("");
}