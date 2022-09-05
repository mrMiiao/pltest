#![feature(panic_info_message, box_syntax, decl_macro, print_internals)]

extern crate unmem;
use unmem::{get_mut, DeOwn};
use core::ops::Index;
use std::{io::Write, collections::HashMap};

#[cfg(debug_assertions)]
macro debug($l:tt) {
    println!("{:?}", $l.0);
}

#[cfg(not(debug_assertions))]
macro debug($l:tt) {}

macro print($($body:tt)*) {
    std::io::_print(format_args!($($body)*));
    std::io::stdout().flush().unwrap();
}

trait Dec {
    fn dec(&mut self);
}

impl Dec for u8 {
    fn dec(&mut self) {
        if *self != 0 {
            *self -= 1;
        }
    }
}

struct HMW<'a>(pub HashMap<&'a str, u8>);

impl<'a> HMW<'a> {
    #[inline(never)]
    pub fn insert(&mut self, a: &'a str, b: u8) {
        self.0.insert(a, b);
    }

    #[inline(never)]
    pub fn max(&self) -> &'a str {
        self.0.iter().map(|(t, u)| (*u, *t)).collect::<HashMap<u8, &str>>()[self.0.values().max().unwrap()]
    }
}

impl<'a> core::ops::Index<&'a str> for HMW<'_> {
    type Output = u8;

    #[inline(never)]
    fn index(&self, idx: &'a str) -> &Self::Output {
        &self.0[idx]
    }
}

impl<'a> core::ops::IndexMut<&'a str> for HMW<'_> {
    #[inline(never)]
    fn index_mut(&mut self, idx: &'a str) -> &mut Self::Output {
        get_mut(self.0.index(idx))
    }
}

#[inline(never)]
fn sin() -> &'static str {
    let mut i = String::new();
    let res = std::io::stdin().read_line(&mut i);
    println!();
    if let Err(_) = res {
        ""
    } else {
        i.leak().trim()
    }
}

macro_rules! read {
    () => {
        sin()
    };

    ($type:ty) => {
        sin().parse::<$type>().expect("Введённое число не входит в промежуток 0 - 10!")
    }
}

fn main() {
    #[cfg(not(debug_assertions))]
    #[cfg(panic = "unwind")]
    std::panic::set_hook(box |info| {
        eprint!("{msg}", msg = match info.message() {
            None => "Program panicked!".to_owned(),
            Some(x) => x.to_string()
        });
        }
    );

    let mut langs: HMW = HMW(std::collections::HashMap::new());
    langs.insert("Rust", 0);
    langs.insert("C++", 0);
    langs.insert("C", 0);
    langs.insert("D", 0);
    langs.insert("Go", 0);
    langs.insert("JavaScript", 0);
    langs.insert("Python", 0);
    langs.insert("Nim", 0);
    langs.insert("Kotlin", 0);
    langs.insert("Dart", 0);
    langs.insert("C#", 0);
    langs.insert("Pascal", 0);
    langs.insert("Ruby", 0);
    langs.insert("Haskell", 0);
    langs.insert("PHP", 0);
    langs.insert("Lua", 0);
    langs.insert("Ada", 0);
    langs.insert("Erlang", 0);
    langs.insert("Fortran", 0);

    debug!(langs);

    println!("PLTest - утилита, которая поможет вам подобрать язык программирования.\nВам будет задано несколько вопросов, отвечайте на них числами по десятибальной шкале.\nНачнём!\n");
    print!("Насколько для вас важна производительность?: ");
    let performance = read!(u8);

    if performance > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Введите соотношение простоты обучения к простоте написания кода (меньше баллов - легче учить, больше - легче писать): ");
    let ease = read!(u8);

    if ease > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важен низкоуровневый контроль?: ");
    let control = read!(u8);

    if control > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна безопасность?: ");
    let security = read!(u8);

    if security > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна популярность языка, количество библиотек?: ");
    let popularity = read!(u8);

    if popularity > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Интересно ли вам устройство ЭВМ и то, как именно исполняется код?: ");
    let interest = read!(u8);

    if interest > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна возможность написания фронтенда сайтов?: ");
    let frontend = read!(u8);

    if frontend > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна возможность написания бэкенда сайтов?: ");
    let backend = read!(u8);

    if backend > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна возможность написания критического ПО (десктоп, ОС, драйвера, встроенные системы)?: ");
    let critical = read!(u8);

    if critical > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна возможность разработки игр?: ");
    let gamedev = read!(u8);

    if gamedev > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Насколько для вас важна кроссплатформенность?: ");
    let crossp = read!(u8);

    if crossp > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    print!("Вы - новичок в программировании? (0 если нет): ");
    let newbie = read!(u8);

    if newbie > 10 {
        panic!("Введённое число не входит в промежуток 0 - 10!");
    }

    if (0..=10).contains(&performance) {
        langs["Rust"] += 1;
        langs["C"] += 1;
    }

    if (0..=9).contains(&performance) {
        langs["C++"] += 1;
        langs["Pascal"] += 1;
        langs["Fortran"] += 1;
    }

    if (0..=8).contains(&performance) {
        langs["D"] += 1;
        langs["Nim"] += 1;
        langs["Ada"] += 1;
    }

    if (0..=7).contains(&performance) {
        langs["Go"] += 1;
        langs["C#"] += 1; 
    }

    if (0..=6).contains(&performance) {
        langs["Kotlin"] += 1;
    }

    if (0..=4).contains(&performance) {
        langs["Dart"] += 1;
        langs["Haskell"] += 1;
        langs["PHP"] += 1;
    }

    if (0..=2).contains(&performance) {
        langs["Erlang"] += 1;
    }

    if (0..=1).contains(&performance) {
        langs["JavaScript"] += 1;
        langs["Python"] += 1;
        langs["Ruby"] += 1;
        langs["Lua"] += 1;
    }

    debug!(langs);

    if (6..=10).contains(&ease) {
        langs["Rust"] += 1;
    }

    if (4..=8).contains(&ease) {
        langs["D"] += 1;
        langs["Haskell"] += 1;
        langs["Fortran"] += 1;
    }

    if (2..=5).contains(&ease) {
        langs["C++"] += 1;
        langs["Ada"] += 1;
    }

    if (1..=5).contains(&ease) {
        langs["C"] += 1;
    }

    if (1..=4).contains(&ease) {
        langs["C#"] += 1;
    }

    if (1..=7).contains(&ease) {
        langs["Nim"] += 1;
        langs["Kotlin"] += 1;
        langs["PHP"] += 1;
    }

    if (0..=9).contains(&ease) {
        langs["Go"] += 1;
        langs["Python"] += 1;
        langs["Ruby"] += 1;
    }

    if (0..=5).contains(&ease) {
        langs["JavaScript"] += 1;
        langs["Erlang"] += 1;
    }

    if (0..=6).contains(&ease) {
        langs["Dart"] += 1;
        langs["Lua"] += 1;
    }

    if (0..=10).contains(&ease) {
        langs["Pascal"] += 1;
    }

    debug!(langs);

    if (0..=10).contains(&critical) {
        langs["Rust"] += 1;
    }

    if (0..=9).contains(&critical) {
        langs["C"] += 1;
        langs["C++"] += 1;
    }

    if (0..=8).contains(&critical) {
        langs["D"] += 1;
        langs["Ada"] += 1;
        langs["Fortran"] += 1;
    }

    if (0..=7).contains(&critical) {
        langs["C#"] += 1;
        langs["Go"] += 1;
        langs["Pascal"] += 1;
    }

    debug!(langs);

    if (0..=10).contains(&control) {
        langs["Rust"] += 1;
    }

    if (0..=9).contains(&control) {
        langs["C"] += 1;
        langs["Pascal"] += 1;
        langs["Fortran"] += 1;
    }

    if (0..=8).contains(&control) {
        langs["C++"] += 1;
        langs["Ada"] += 1;
    }

    if (0..=7).contains(&control) {
        langs["D"] += 1;
        langs["Go"] += 1;
        langs["Nim"] += 1;
    }

    if (0..=6).contains(&control) {
        langs["C#"] += 1;
        langs["Haskell"] += 1;
    }

    if (0..=3).contains(&control) {
        langs["Kotlin"] += 1;
        langs["Erlang"] += 1;
    }

    if (0..=1).contains(&control) {
        langs["Python"] += 1;
        langs["JavaScript"] += 1;
        langs["PHP"] += 1;
        langs["Ruby"] += 1;
        langs["Lua"] += 1;
    }

    debug!(langs);

    if (0..=10).contains(&security) {
        langs["Rust"] += 1;
    }

    if (0..=9).contains(&security) {
        langs["Python"] += 1;
        langs["JavaScript"] += 1;
        langs["PHP"] += 1;
        langs["Ruby"] += 1;
        langs["Lua"] += 1;
    }

    if (0..=8).contains(&security) {
        langs["Dart"] += 1;
        langs["Kotlin"] += 1;
        langs["Haskell"] += 1;
        langs["Erlang"] += 1;
    }

    if (0..=5).contains(&security) {
        langs["D"] += 1;
        langs["Go"] += 1;
        langs["C#"] += 1;
        langs["Ada"] += 1;
        langs["Pascal"] += 1;
    }

    if (0..=3).contains(&security) {
        langs["Fortran"] += 1;
    }

    if (0..=2).contains(&security) {
        langs["C++"] += 1;
    }
    
    if (0..=1).contains(&security) {
        langs["C"] += 1;
    }

    debug!(langs);

    if (6..=10).contains(&popularity) {
        langs["Python"] += 1;
        langs["JavaScript"] += 1;
        langs["PHP"] += 1;
    }

    if (5..=8).contains(&popularity) {
        langs["Lua"] += 1;
        langs["Haskell"] += 1;
        langs["Kotlin"] += 1;
        langs["Go"] += 1;
        langs["C#"] += 1;
    }

    if (4..=9).contains(&popularity) {
        langs["C++"] += 1;
    }

    if (4..=8).contains(&popularity) {
        langs["C"] += 1;
        langs["Ruby"] += 1;
    }

    if (0..=4).contains(&popularity) {
        langs["Erlang"] += 1;
        langs["Nim"] += 1;
        langs["Pascal"] += 1;
    }

    if (0..=3).contains(&popularity) {
        langs["Ada"] += 1;
        langs["Fortran"] += 1;
    }

    if (0..=5).contains(&popularity) {
        langs["Rust"] += 1;
    }

    if popularity > 7 {
        langs["Rust"].dec();
        langs["Ada"].dec();
        langs["Fortran"].dec();
        langs["Nim"].dec();
        langs["Erlang"].dec();
        langs["D"].dec();
    }

    if performance > 5 {
        langs["Python"].dec();
        langs["JavaScript"].dec();
        langs["PHP"].dec();
        langs["Ruby"].dec();
        langs["Lua"].dec();
        langs["Kotlin"].dec();
        langs["Haskell"].dec();
        langs["Erlang"].dec();
        langs["C#"].dec();
    }

    if critical == 10 {
        langs["Python"].dec();
        langs["JavaScript"].dec();
        langs["PHP"].dec();
        langs["Ruby"].dec();
        langs["Lua"].dec();
        langs["Kotlin"].dec();
        langs["Haskell"].dec();
        langs["Erlang"].dec();
    }

    if security > 8 {
        langs["C"].dec();
        langs["C++"].dec();
    }

    if ease < 3 {
        langs["Rust"].dec();
        langs["C++"].dec();
    }

    if control == 10 {
        langs["Python"].dec();
        langs["JavaScript"].dec();
        langs["PHP"].dec();
        langs["Ruby"].dec();
        langs["Lua"].dec();
        langs["Kotlin"].dec();
        langs["Haskell"].dec();
        langs["Erlang"].dec();
    }

    debug!(langs);

    if interest == 0 {
        langs["Python"] += 1;
        langs["JavaScript"] += 1;
        langs["PHP"] += 1;
        langs["Haskell"] += 1;
        langs["Kotlin"] += 1;
        langs["Erlang"] += 1;
        langs["Lua"] += 1;
    }

    if (0..=6).contains(&interest) {
        langs["D"] += 1;
        langs["Go"] += 1; 
        langs["C#"] += 1;
        langs["Ada"] += 1;
    }

    if (0..=8).contains(&interest) {
        langs["C++"] += 1;
        langs["Pascal"] += 1;
        langs["Fortran"] += 1;
    }

    if (0..=10).contains(&interest) {
        langs["C"] += 1;
        langs["Rust"] += 1;
    }

    if interest < 4 {
        langs["C"].dec();
        langs["C++"].dec();
        langs["Rust"].dec();
        langs["D"].dec();
        langs["Ada"].dec();
    }

    debug!(langs);

    if (0..=10).contains(&backend) {
        langs["Rust"] += 1;
    }

    if (0..=9).contains(&backend) {
        langs["Python"] += 1;
        langs["PHP"] += 1;
        langs["Go"] += 1;
    }

    if (0..=7).contains(&backend) {
        langs["JavaScript"] += 1;
        langs["Lua"] += 1;
        langs["C#"] += 1;
        langs["Ruby"] += 1;
    }

    if (0..=5).contains(&backend) {
        langs["C++"] += 1;
        langs["D"] += 1;
        langs["Kotlin"] += 1;
        langs["Haskell"] += 1;
        langs["Erlang"] += 1;
        langs["Dart"] += 1;
        langs["Nim"] += 1;
    }

    if (0..=3).contains(&backend) {
        langs["Ada"] += 1;
        langs["Fortran"] += 1;
        langs["Pascal"] += 1;
    }

    debug!(langs);

    if (0..=10).contains(&gamedev) {
        langs["C++"] += 1;
    }

    if (0..=9).contains(&gamedev) {
        langs["Rust"] += 1;
    }

    if (0..=4).contains(&gamedev) {
        langs["Python"] += 1;
        langs["PHP"] += 1;
        langs["Go"] += 1;
        langs["JavaScript"] += 1;
        langs["Lua"] += 1;
        langs["C#"] += 1;
        langs["Ruby"] += 1;
        langs["D"] += 1;
        langs["Kotlin"] += 1;
        langs["Haskell"] += 1; 
        langs["Erlang"] += 1;
        langs["Dart"] += 1;
        langs["Nim"] += 1;
        langs["Ada"] += 1;
        langs["Fortran"] += 1;
        langs["Pascal"] += 1;
    }

    debug!(langs);

    if (0..=10).contains(&crossp) {
        langs["Dart"] += 1;
        langs["Kotlin"] += 1;
    }

    if (0..=9).contains(&crossp) {
        langs["Rust"] += 1;
    }

    if (0..=8).contains(&crossp) {
        langs["Python"] += 1;
        langs["PHP"] += 1;
        langs["Go"] += 1;
        langs["JavaScript"] += 1;
        langs["Lua"] += 1;
        langs["C#"] += 1;
        langs["Ruby"] += 1;
        langs["Haskell"] += 1;
        langs["Erlang"] += 1;
        langs["Nim"] += 1;
    }

    if (0..=7).contains(&crossp) {
        langs["D"] += 1;
        langs["C"] += 1;
        langs["C++"] += 1;
        langs["Fortran"] += 1;
        langs["Ada"] += 1;
        langs["Pascal"] += 1;
    }

    debug!(langs);

    if frontend == 10 {
        langs["Python"] = 0;
        langs["PHP"] += 1;
        langs["Go"] = 0;
        langs["Lua"] = 0;
        langs["C#"] = 0;
        langs["Ruby"] = 0;
        langs["Haskell"] = 0;
        langs["Erlang"] = 0;
        langs["D"] = 0;
        langs["Rust"] += 1;
        langs["Nim"] += 1;
        langs["Dart"] += 1;
        langs["Kotlin"] += 1;
        langs["Ada"] = 0;
        langs["Pascal"] = 0;
        langs["C"] = 0;
        langs["C++"] = 0;
        langs["D"] = 0;
        langs["Fortran"] = 0;
    }

    if (0..=9).contains(&frontend) {
        langs["Dart"] += 1;
        langs["Kotlin"] += 1;
        langs["JavaScript"] += 1;
        langs["Rust"] += 1;
        langs["PHP"] += 1;
    }

    debug!(langs);

    if newbie == 10 {
        langs["Python"] = 0;
        langs["PHP"] = 0;
        langs["Go"] = 0;
        langs["Lua"] = 0;
        langs["C#"] = 0;
        langs["Ruby"] = 0;
        langs["Haskell"] = 0;
        langs["Erlang"] = 0;
        langs["D"] = 0;
        langs["Rust"] = 0;
        langs["Nim"] = 0;
        langs["Dart"] = 0;
        langs["Kotlin"] = 0;
        langs["Ada"] = 0;
        langs["Pascal"] += 1;
        langs["C"] = 0;
        langs["C++"] = 0;
        langs["D"] = 0;
        langs["JavaScript"] = 0;
        langs["Fortran"] = 0;
    }

    if (1..=9).contains(&newbie) {
        langs["Pascal"] += 1;
    }

    debug!(langs);

    println!("Лучшим языком для вас будет: {}", langs.max());
    read!();
}