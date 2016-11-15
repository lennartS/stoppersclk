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

// Third-Party uses
use glib;
use gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, ImageMenuItem, Label, ToggleButton, Window};
use std::cell::RefCell;
use time;

// Project uses

struct State {
    runtime_till_last_pause: Option<time::Duration>,
    last_started_on: Option<time::PreciseTime>,
    laptime_till_last_pause: Option<time::Duration>,
    lap_started_on: Option<time::PreciseTime>,
}

// declare a new thread local storage key
thread_local!(
    static GLOBAL: RefCell<Option<(Builder, State)>> = RefCell::new(None)
);

fn reset_time_label(label: &Label) {
    label.set_label(format_duration(&time::Duration::seconds(0)).as_str());
}

fn get_total_time_label(builder: &Builder) -> Label {
    builder.get_object("lb_total").expect("Label 'lb_total' not found.")
}

fn get_lap_time_label(builder: &Builder) -> Label {
    builder.get_object("lb_lap").expect("Label 'lb_lap' not found.")
}

// Do I really have to implement this myself? Surely it's already somewhere in std?
fn format_duration(d: &time::Duration) -> String {
    let hours = d.num_hours();
    let minutes = d.num_minutes() - d.num_hours() * 60;
    let seconds = d.num_seconds() - d.num_minutes() * 60;
    let mseconds = d.num_milliseconds() - d.num_seconds() * 1000;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, mseconds)
}

fn update_time() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref builder, ref mut state)) = *global.borrow_mut() {
            let tb_start_stop: ToggleButton = builder.get_object("tb_start_stop")
                .expect("ToggleButton
            'tb_start_stop' not found.");
            if tb_start_stop.get_active() {
                let current = time::PreciseTime::now();
                let passed = match state.last_started_on {
                    Some(x) => {
                        x.to(current) +
                        state.runtime_till_last_pause.unwrap_or_else(|| time::Duration::seconds(0))
                    }
                    None => time::Duration::seconds(0),
                };
                get_total_time_label(builder).set_markup(format_duration(&passed).as_str());

                let lap_passed = match state.lap_started_on {
                    Some(x) => {
                        x.to(current) +
                        state.laptime_till_last_pause.unwrap_or_else(|| time::Duration::seconds(0))
                    }
                    None => time::Duration::seconds(0),
                };
                get_lap_time_label(builder).set_markup(format_duration(&lap_passed).as_str());
            }
        }
    });

    glib::Continue(true)
}

pub fn init_and_show() {
    let builder = Builder::new_from_string(include_str!("main_window.glade"));

    let window: Window = builder.get_object("main_window")
        .expect("Window 'main_window' not found.");
    let menu_quit: ImageMenuItem = builder.get_object("menu_file_quit")
        .expect("Menu entry 'File | Quit' not found.");
    let tb_start_stop: ToggleButton = builder.get_object("tb_start_stop").expect("ToggleButton
    'tb_start_stop' not found.");
    let bt_reset: Button = builder.get_object("bt_reset").expect("Button 'bt_reset' not found.");
    reset_time_label(&get_total_time_label(&builder));
    let bt_lap: Button = builder.get_object("bt_lap").expect("Button 'bt_lap' not found.");
    reset_time_label(&get_lap_time_label(&builder));

    // TODO We should only add this function when the clock is actually running
    glib::timeout_add(1, update_time);

    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((builder.clone(),
                                     State {
            runtime_till_last_pause: None,
            last_started_on: None,
            laptime_till_last_pause: None,
            lap_started_on: None,
        }))
    });

    window.connect_delete_event(|_, _| {
        debug!("GTK deinitialized.");
        gtk::main_quit();
        Inhibit(false)
    });

    menu_quit.connect_activate(|_| {
        debug!("GTK deinitialized.");
        gtk::main_quit();
    });

    tb_start_stop.connect_toggled(move |_| {
        GLOBAL.with(move |global| {
            if let Some((ref builder, ref mut state)) = *global.borrow_mut() {
                let tb_start_stop: ToggleButton = builder.get_object("tb_start_stop")
                    .expect("ToggleButton 'tb_start_stop' not
                found'");
                if tb_start_stop.get_active() {
                    state.last_started_on = Some(time::PreciseTime::now());
                    state.lap_started_on = Some(time::PreciseTime::now());
                } else {
                    state.runtime_till_last_pause = match state.last_started_on {
                        Some(x) => {
                            Some(state.runtime_till_last_pause
                                .unwrap_or_else(|| time::Duration::seconds(0)) +
                                 x.to(time::PreciseTime::now()))
                        }
                        None => state.runtime_till_last_pause,
                    };
                    state.last_started_on = None;
                    state.laptime_till_last_pause = match state.lap_started_on {
                        Some(x) => {
                            Some(state.laptime_till_last_pause
                                .unwrap_or_else(|| time::Duration::seconds(0)) +
                                 x.to(time::PreciseTime::now()))
                        }
                        None => state.laptime_till_last_pause,
                    };
                    state.lap_started_on = None;
                }

            }
        });
    });

    bt_reset.connect_clicked(move |_| {
        GLOBAL.with(move |global| {
            if let Some((ref builder, ref mut state)) = *global.borrow_mut() {
                let tb_start_stop: ToggleButton = builder.get_object("tb_start_stop")
                    .expect("ToggleButton 'tb_start_stop' not
                found'");
                if !tb_start_stop.get_active() {
                    reset_time_label(&get_total_time_label(builder));
                    state.last_started_on = None;
                    state.runtime_till_last_pause = None;

                    reset_time_label(&get_lap_time_label(builder));
                    state.lap_started_on = None;
                    state.laptime_till_last_pause = None;
                } else {
                    state.last_started_on = Some(time::PreciseTime::now());
                    state.runtime_till_last_pause = None;

                    state.lap_started_on = Some(time::PreciseTime::now());
                    state.laptime_till_last_pause = None;
                }
            }
        });
    });

    bt_lap.connect_clicked(move |_| {
        GLOBAL.with(move |global| {
            if let Some((_, ref mut state)) = *global.borrow_mut() {
                state.laptime_till_last_pause = None;
                state.lap_started_on = Some(time::PreciseTime::now());
            }
        });
    });

    window.show_all();
}

#[cfg(test)]
mod tests {
    // Globbing only uses public stuff
    use super::format_duration;

    use time::Duration;

    #[test]
    fn test_format_duration() {
        assert_eq!("00:01:00.000", format_duration(&Duration::minutes(1)));
        assert_eq!("01:01:00.000", format_duration(&Duration::minutes(61)));
        assert_eq!("01:01:01.000",
                   format_duration(&Duration::seconds(1 + 60 + 60 * 60)));
        assert_eq!("00:00:04.540",
                   format_duration(&Duration::milliseconds(4540)));
        assert_eq!("100:00:00.000", format_duration(&Duration::hours(100)));
    }
}
