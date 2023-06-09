use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// - ile başlayan komutları karşılar
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Yeni kitap ekleme
    Add,
    /// Tüm kitapları listelemek için
    All,
    /// Id değerine göre kitap silme
    Del,
    /// Kitapları listeleme
    List,
    /// Girilen kitabı arama
    Find,
    /// Nasıl kullanıldığı ile ilgili yardım içeriği
    Help,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-add" => Ok(Command::Add),
            "-list" => Ok(Command::List),
            "-all" => Ok(Command::All),
            "-del" => Ok(Command::Del),
            "-find" => Ok(Command::Find),
            "-help" => Ok(Command::Help),
            _ => Err(ParseError::Command),
        }
    }
}

/// -list komutundaki parametreleri karşılar.
#[derive(Debug, PartialEq)]
pub struct ListCommand {
    /// Sırlamanın yapılacağı alan adı
    pub field_name: String,
    /// Sıralamanın hangi yönde olduğu
    pub order: Order,
    /// Kaç tane gösterileceği
    pub count: u32,
}

impl ListCommand {
    pub fn init(field_name: String, order: Order, count: u32) -> Self {
        Self {
            field_name,
            order,
            count,
        }
    }
}

impl FromStr for ListCommand {
    type Err = ListCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err(ListCommandParseError::WrongParameterCount);
        }
        if let Ok(o) = Order::from_str(parts[1]) {
            if let Ok(c) = u32::from_str(parts[2]) {
                Ok(ListCommand::init(parts[0].to_string(), o, c))
            } else {
                Err(ListCommandParseError::InvalidNumber)
            }
        } else {
            Err(ListCommandParseError::InvalidOrdering)
        }
    }
}

impl Default for ListCommand {
    fn default() -> Self {
        Self {
            field_name: String::new(),
            order: Order::Asc,
            count: 10,
        }
    }
}

/// Sıralamanın hangi yönde olduğu bilgisini karşılar
#[derive(Debug, PartialEq)]
pub enum Order {
    /// A -> Z sıralama
    Asc,
    /// Z -> A sıralama
    Desc,
}

impl FromStr for Order {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err(ParseError::Order),
        }
    }
}

/// String dönüşümlerindeki hataları karşılar
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Geçersiz sıralama değeri gelirse
    Order,
    /// Geçersiz komut hatası
    Command,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Order => {
                write!(f, "Parametre sıraları hatalı")
            }
            ParseError::Command => {
                write!(f, "Komut anlaşılamadı")
            }
        }
    }
}

/// -list komut ardından gelen parametreler ile ilgili hatalar
#[derive(Debug, PartialEq)]
pub enum ListCommandParseError {
    /// Hatalı sayıda argüman
    WrongParameterCount,
    /// maksimum eleman değeri sayısal değil
    InvalidNumber,
    /// asc, desc geçerli değil
    InvalidOrdering,
}
