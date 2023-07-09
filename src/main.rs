use maplit::hashmap;
use std::collections::HashMap;

fn display_map(map: &HashMap<&str, &str>, keys: Vec<&str>) {
    println!("{}:\n", map["title"]);
    for key in keys {
        println!("{}:    {}", key, map[key]);
    }
}

fn easyselect(prompt: &str, choices: Vec<String>) -> String {
    inquire::Select::new(prompt, choices).prompt().unwrap()
}

fn check_key_exists(matches: &clap::ArgMatches, key: &str) -> bool {
    let key_exists: bool = match matches.get_one::<u8>(key).expect("Option get error") {
        0 => false,
        _ => true,
    };
    key_exists
}

fn main() {
    let reference = hashmap! {
        "basics" => hashmap!{
            "title" => "Basics",
            "Open new terminal" => "mod + Enter",
            "Focus left" => "mod + j OR mod + Left Arrow",
            "Focus down" => "mod + k OR mod + Down Arrow",
            "Focus up" => "mod + k OR mod + Up Arrow",
            "Focus right" => "mod + ; OR mod + Right Arrow",
            "Focus parent" => "mod + a",
            "Toggle focus mode" => "mod + Space",
        },
        "moving" => hashmap!{
            "title" => "Moving windows",
            "Move window left" => "mod + Shift + j OR mod + shift + Left Arrow",
            "Move window down" => "mod + Shift + k OR mod + shift + Down Arrow",
            "Move window up" => "mod + Shift + l OR mod + shift + Up Arrow",
            "Move window right" => "mod + Shift + l OR mod + shift + Right Arrow",
        },
        "modifying" => hashmap!{
            "title" => "Modifying windows",
            "Toggle fullscreen" => "mod + f",
            "Split window vertically" => "mod + v",
            "Split window horizontally" => "mod + h",
            "Enter/exit Resize mode" => "mod + r",
        },
        "layout" => hashmap!{
            "title" => "Changing the container layout",
            "Default layout" => "mod + e",
            "Stacking layout" => "mod + s",
            "Tabbed layout" => "mod + w",
        },
        "floating" => hashmap!{
            "title" => "Floating",
            "Toggle floating on window" => "mod + Shift + Space",
            "Drag floating window" => "Drag window bar (the one at the top of the window with the title) with Left Click",
        },
        "workspaces" => hashmap!{
            "title" => "Using workspaces",
            "Switch to workspace" => "mod + 0-9",
            "Move window to workspace" => "mod + Shift + 0-9",
        },
        "opening/closing" => hashmap!{
            "title" => "Opening applications / Closing windows",
            "Open application launcher (by default, dmenu)" => "mod + d",
            "Kill a window" => "mod + Shift + q",
        },
        "restart/exit" => hashmap!{
            "title" => "Restart / Exit",
            "Reload from configuration" => "mod + Shift + c",
            "Restart i3 in place" => "mod + Shift + r",
            "Exit i3" => "mod + Shift + e",
        },
    };

    let ordered = hashmap! {
        "basics" => vec![
            "Open new terminal",
            "Focus left",
            "Focus down",
            "Focus up",
            "Focus right",
            "Focus parent",
            "Toggle focus mode",
        ],
        "moving" => vec![
            "Move window left",
            "Move window down",
            "Move window up",
            "Move window right",
        ],
        "modifying" => vec![
            "Toggle fullscreen",
            "Split window vertically",
            "Split window horizontally",
            "Enter/exit Resize mode",
        ],
        "layout" => vec![
            "Default layout",
            "Stacking layout",
            "Tabbed layout",
        ],
        "floating" => vec![
            "Toggle floating on window",
            "Drag floating window",
        ],
        "workspaces" => vec![
            "Switch to workspace",
            "Move window to workspace",
        ],
        "opening/closing" => vec![
            "Open application launcher (by default, dmenu)",
            "Kill a window",
        ],
        "restart/exit" => vec![
            "Reload from configuration",
            "Restart i3 in place",
            "Exit i3",
        ],
        "reference" => vec![
            "basics", "moving", "modifying", "layout", "floating", "workspaces", "opening/closing", "restart/exit"
        ]
    };

    let help_prefix: &str = "Command line reference for i3wm.

Note: The key referred to as \"mod\" in the reference should be Left Alt by default. Some people have 
it set as Win/Cmd (depending on their keyboard). It should be set near the top of your i3 config file.";

    let matches: clap::ArgMatches = clap::Command::new("i3ref")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Astro Orbis <astroorbis@gmail.com>")
        .about(&help_prefix.to_owned())
        .long_about(&help_prefix.to_owned())
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("section")
                .short('s')
                .long("section")
                .action(clap::ArgAction::Set)
                .value_name("SECTION")
                .help("Only returns a section of the reference. Available sections can be found using the -l flag.")
                .required(false)
        )
        .arg(
            clap::arg!(-p --picker ... "Opens a section picker, then views the picked section.")
                .exclusive(true)
        )
        .arg(
            clap::arg!(-l --list ... "Lists the sections for the reference")
                .exclusive(true)
        )
        .arg(
            clap::arg!(-o --open ... "Opens the official reference card website")
                .exclusive(true)
        )
        .arg(
            clap::arg!(-f --full ... "Prints the full reference")
                .exclusive(true)
        )
        .get_matches();

    if check_key_exists(&matches, "open") {
        println!("Opening in browser...");
        webbrowser::open("https://i3wm.org/docs/refcard.html").expect("Eror opening in browser");
        std::process::exit(0);
    }

    if check_key_exists(&matches, "list") {
        println!("Sections:\n");
        for key in ordered["reference"].clone() {
            println!("{key}");
        }
        std::process::exit(0);
    }

    if check_key_exists(&matches, "full") {
        println!("i3 Reference:\n");
        for key in ordered["reference"].clone() {
            display_map(&reference[key], ordered[key].clone());
            println!("");
        }
        std::process::exit(0);
    }

    if check_key_exists(&matches, "picker") {
        let stringed = ordered["reference"].iter().map(|s| s.to_string()).collect();
        let choice = easyselect("Pick a section to view:", stringed);
        display_map(
            &reference[choice.as_str()],
            ordered[choice.as_str()].clone(),
        );
        std::process::exit(0);
    }

    let sectionkey = match matches.get_one::<String>("section") {
        Some(val) => val.to_lowercase(),
        None => String::from(""),
    };

    if !sectionkey.is_empty() {
        let sec: &str = sectionkey.as_str();
        if reference.contains_key(&sec) {
            display_map(&reference[sec], ordered[sec].clone())
        } else {
            println!(
                "The provided section does not exist. Try the section picker or the list flag!"
            )
        }
        std::process::exit(0);
    }
}
