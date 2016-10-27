// Copyright (C) 2016 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
//
// This file is part of Stoppersclk.
//
// Stoppersclk is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Stoppersclk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stoppersclk.  If not, see <http://www.gnu.org/licenses/>.

// Extern crates
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;

// Third-Party uses

// Project uses
mod gui;

fn main() {
    if env_logger::init().is_err() {
        println!("Could not initialize logging subsystem.");
    }

    if gtk::init().is_err() {
        error!("Failed to initialize GTK.");
        return;
    }

    gui::init_and_show();

    gtk::main();
}
