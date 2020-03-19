use prettytable::{Table, Row};
use prettytable::format;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TablePrint {}

impl TablePrint {
    pub fn new() -> TablePrint {
        Default::default()
    }

    pub fn print_with_title(&mut self, title: &str, mut table: Table, response: Vec<Row>) {
        let mut tables: Vec<Table> = Vec::new();
        let mut tr = Table::new();
        let format_style = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(&[], format::LineSeparator::new(' ', ' ', ' ', ' '))
            .padding(1, 1)
            .build();

        tr.add_row(row![bF->title]);
        tr.set_format(format_style);
        tables.push(tr);
        tables.push(self.create_table(table, response));

        self.print_tables(tables);
    }

    pub fn print(&mut self, mut table: Table, response: Vec<Row>) {
        let mut tables: Vec<Table> = Vec::new();

        tables.push(self.create_table(table, response));

        self.print_tables(tables);
    }

    pub fn create_table(&mut self, mut table: Table, response: Vec<Row>) -> Table {
        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(&[], format::LineSeparator::new(' ', ' ', ' ', ' '))
            .padding(2, 1)
            .build();

        table.set_format(format);
        for row in response {
            table.add_row(row);
        }

        table
    }

    pub fn print_tables(&mut self, tables: Vec<Table>) {
        for x in tables {
            x.printstd();
        }
    }
}
