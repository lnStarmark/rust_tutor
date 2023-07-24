use std::fmt; // Импортируем `fmt`
//================================================================================================

//--- форматированный вывод list с помощью display
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display/testcase_list.html

// Определяем структуру с именем `List`, которая хранит в себе `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Получаем значение с помощью индекса кортежа
        // и создаём ссылку на `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Пройдёмся по каждому `v` в `vec`.
        // Номер итерации хранится в `count`.
        for (count, v) in vec.iter().enumerate() {
            // Для каждого элемента, кроме первого, добавим запятую
            // до вызова `write!`. Используем оператор `?` или `try!`,
            // чтобы вернуться при наличии ошибок.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Закроем открытую скобку и вернём значение `fmt::Result`
        write!(f, "]")
    }
}

fn test_fmt_list() {

	println!("\n<<< форматированный вывод list с помощью display >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display/testcase_list.html\n");

	println!("Работа по тексту туториала:\n");
	
	//--- так
    let v = List(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    println!("{}", v);

	//--- или так
    println!("{}", List(vec![4, 6, 7, 2, 5, 8]));    
}
//================================================================================================

//--- форматированный вывод с помощью display
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display.html

// Структура, которая хранит в себе два числа.
// Вывод типажа `Debug` добавлен для сравнения с `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Реализуем `Display` для `MinMax`.
impl fmt::Display for MinMax {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	// Используем `self.номер`, чтобы получить доступ к каждому полю структуры.
    	write!(f, "({}, {})", self.0, self.1)
    }	
}

// Объявим структуру с именованными полями, для сравнения
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// По аналогии, реализуем `Display` для Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Point2D будет по имени
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Объявим структуру Complex
#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

// По аналогии, реализуем `Display` для Complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Complex будет по имени
        write!(f, "{} + {}i", self.re, self.im)
    }
}

fn test_fmt_display() {

	println!("\n<<< форматированный вывод с помощью display >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display.html\n");

	println!("Работа по тексту туториала:\n");

    let minmax = MinMax(0, 14);

    println!("Сравниваем структуры:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("Большой диапазон - {big} и маленький диапазон {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Сравниваем точки:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    let cmpl = Complex { re: 1.3, im: 2.7 };
    
    println!("Сравниваем вывод комплексных чисел:");
    println!("Display: {}", cmpl);
    println!("Debug: {:#?}", cmpl);    

    // Ошибка. Типажи `Debug` и `Display` были реализованы, но `{:b}`
    // необходима реализация `fmt::Binary`. Следующий код не сработает.
    // println!("Как выглядит Point2D в виде двоичного кода: {:b}?", point);
}
//================================================================================================

//--- форматированный вывод с помощью debug
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_debug.html
fn test_fmt_debug() {
	println!("\n<<< форматированный вывод с помощью debug >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_debug.html\n");


    println!("Атрибут `derive` автоматически реализует");
    println!("необходимые методы, чтобы была возможность");
    println!("напечатать структуру `struct` с помощью `fmt::Debug`.");
    println!("#[derive(Debug)]");
    println!("struct DebugPrintable(i32);.\n");

	println!("Работа по тексту туториала:\n");
	
    // `{}` автоматически будет заменено на аргументы. Они будут преобразованы в строку.
    println!("{} дней", 31);

	// Вывод и реализация `fmt::Debug` для `Structure`.
	// `Structure` - это структура, которая содержит в себе один `i32`.
	#[derive(Debug)]
	struct Structure(i32);

	// Добавим структуру `Structure` в структуру `Deep`.
	// Реализуем для `Deep` возможность вывода с помощью fmt::Debug`.
	#[derive(Debug)]
	struct Deep(Structure);

    // Вывод с помощью `{:?}` аналогичен `{}`.
    println!("{:?} месяцев в году.", 12);
    println!("{1:?} {0:?} - это имя {actor:?}.",
             "Слэйтер",
             "Кристиан",
             actor="актёра");

    // `Structure` теперь можно напечатать!
    println!("Теперь {:?} будет выведена на экран!", Structure(3));

    println!("Проблема с `выводом (derive)`, в том, что у нас не будет контроля");
    println!("над тем, как будет выглядеть результат.");
    println!("Что, если мы хотим напечатать просто `7`?");
    println!("А теперь напечатаем {:?}", Deep(Structure(7)));
    println!("Не получилось");
    
    // Добавим структуру `Person`.
    #[derive(Debug)]
	struct Person<'a> {
    name: &'a str,
    fam: &'a str,
    job: &'a str,
    age: u8
    }
    
    let name = "Peter";
    let fam = "Petrov";
    let job = "programmer";
    let age = 27;
    let peter = Person { name, fam, job, age };

    // Красивый  print структуры
    println!("{:#?}", peter);

}
//================================================================================================

//--- простой форматированный вывод
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print.html
fn test_fmt_out() {
	println!("\n<<< простой форматированный вывод >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print.html\n");


    println!("format!   записывает форматированный текст в String.");
    println!("print!    работает аналогично с format!, но текст выводится в консоль (io::stdout).");
    println!("println!  аналогично print!, но в конце добавляется переход на новую строку.");
    println!("eprint!   аналогично format!, но текст выводится в стандартный поток ошибок (io::stderr).");
    println!("eprintln! аналогично eprint!, но в конце добавляется переход на новую строку.\n");

	println!("Работа по тексту туториала:\n");
	
    // `{}` автоматически будет заменено на аргументы. Они будут преобразованы в строку.
    println!("{} дней", 31);

    // Без суффиксов, 31 является i32. Можно изменить тип 31, используя суффикс.

    // Существует множество способов работы с форматированным выводом. Можно указать
    // позицию для каждого аргумента.
    println!("{0}, это {1}. {1}, это {0}", "Алиса", "Боб");

    // Так же можно именовать аргументы.
    println!("{subject} {verb} {object}",
             object="ленивую собаку",
             subject="Быстрая коричневая лиса",
             verb="прыгает через");

    println!("{} из {} людей знают, что такое 10-ричный код, а остальные нет.", 17, 10);
    println!("{:o} из {:o} людей знают, что такое 8-ричный код, а остальные нет.", 17, 8);
    println!("{:b} из {:b} людей знают, что такое 2-ичный код, а остальные нет.", 17, 2);
    println!("{:x} из {:x} людей знают, что такое 16-ричный код, а остальные нет.", 17, 16);

    // Можно выравнивать текст, сдвигая его на указанную ширину.
    // Данный макрос отобразит в консоли
    // "     1". 5 пробелов и "1".
    println!("{number:>width$}", number=1, width=6);

    // Можно добавить к цифрам пару нулей. Данный макрос выведет "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Компилятор проверит, что в макрос передано правильное количество аргументов.
    println!("Меня зовут {0}, {1} {0}", "Бонд", "Джеймс");
    // ИСПРАВЬТЕ ^ Добавьте недостающий аргумент: "Джеймс"

    // Создаём структуру, которая хранит в себе `i32`. Назовём её `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // Однако, пользовательские типы данных, например, как эта структура
    // требуют более сложной обработки для вывода. Данный код не будет работать.
    //println!("Эта структура `{}` не хочет выводится на экран...", Structure(3));
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку.
}
//================================================================================================

fn title() {
	println!("\n==========================================================");
	println!("<<< Набор тестовых программ туториала: >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/index.html");
	println!("author: StarmarkLN (Kornilov LN)");
	println!("e-mail: ln.starmark@ekatra.io");
	println!("e-mail: ln.starmark@gmail.com");
	println!("date: 24.07.2023");
	println!("==========================================================\n");
}
//================================================================================================

fn main() {

    title();
    test_fmt_out();
    test_fmt_debug();
    test_fmt_display();
    test_fmt_list(); 
}    
