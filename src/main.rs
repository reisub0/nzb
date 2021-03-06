//
// main.rs
// Copyright (C) 2019 g <g@ABCL>
// Distributed under terms of the MIT license.
//

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate prettytable;

mod args;
mod disp;
mod nzb;
mod tui;

pub fn main() -> Result<(), Box<std::error::Error>> {
    let a = args::parse_args();
    // println!("{:?}", a);
    if let Some(s) = a.flag_auth {
        unsafe {
            nzb::TOKEN = Box::leak(Box::new(s));
        }
    }
    if let Some(x) = a.arg_command {
        match x {
            args::Command::Add => {
                if a.arg_args.len() == 0 {
                    println!("Please enter a task name!");
                    std::process::exit(0);
                }
                nzb::add_task(a.arg_args.join(" "))?
            }
            args::Command::All => disp::print_all()?,
            args::Command::Cat => disp::print_categories(a.arg_args)?,
            args::Command::Conky => disp::print_conky()?,
            args::Command::Debug => disp::print_debug()?,
            args::Command::Done => {
                let tasks = nzb::get_tasks()?;
                nzb::mark_done(tui::task_picker(
                    tasks,
                    a.arg_args,
                    "Select task(s) to mark as done (Multi-select w/ TAB) >> ",
                )?)?;
            }
            args::Command::Help => args::print_help(),
            args::Command::Inbox => disp::print_inbox()?,
            args::Command::Link => {
                if a.arg_args.len() == 0 {
                    println!("Please enter a link!");
                    std::process::exit(0);
                }
                nzb::add_link(a.arg_args.join(" "))?
            }
            args::Command::List => {
                if a.arg_args.is_empty() {
                    disp::print_all()?
                } else {
                    disp::print_lists(a.arg_args)?
                }
            }
            args::Command::Login => nzb::make_auth_token()?,
            args::Command::Move => tui::mv()?,
            args::Command::Mv => tui::mv()?,
            args::Command::Now | args::Command::Priority | args::Command::Starred => {
                disp::print_now()?
            }
            args::Command::Open => {
                webbrowser::open("https://app.nozbe.com/")?;
            }
            args::Command::Overdue => disp::print_overdue()?,
            args::Command::Star => {
                let tasks = nzb::get_tasks()?
                    .iter()
                    .filter(|x| x.now == false)
                    .cloned()
                    .collect();
                nzb::star(tui::task_picker(
                    tasks,
                    a.arg_args,
                    "Select task(s) to star (Multi-select w/ TAB) >> ",
                )?)?;
            }
            args::Command::Today => disp::print_today()?,
            args::Command::Unstar => {
                let tasks = nzb::get_tasks()?
                    .iter()
                    .filter(|x| x.now == true)
                    .cloned()
                    .collect();
                nzb::unstar(tui::task_picker(
                    tasks,
                    a.arg_args,
                    "Select task(s) to unstar (Multi-select w/ TAB) >> ",
                )?)?;
            }
        }
    } else {
        // Default action = Print all
        disp::print_all()?;
    }
    Ok(())
}
