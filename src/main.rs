use std::io::{self, Write};

fn main() {
    loop {
        print!("Horas trabalhadas no mês: ");
        io::stdout().flush().unwrap();
        let mut worked_hours: String = String::from("");
        io::stdin().read_line(&mut worked_hours).expect("Haven't typed anything");

        let worked_hours: i8 = worked_hours.trim().parse().expect("As horas deveriam ser um número inteiro.");

        print!("Valor por hora aula: ");
        io::stdout().flush().unwrap();
        let mut class_hour_value: String = String::from("");
        io::stdin().read_line(&mut class_hour_value).expect("Não digitou nada.");

        let class_hour_value: f32 = class_hour_value.trim().parse().expect("Valor por hora deveria ser um número real");

        print!("Percentual do desconto: ");
        io::stdout().flush().unwrap();
        let mut descount_percentage: String = String::from("");
        io::stdin().read_line(&mut descount_percentage).expect("Não digitou nada.");
        let descount_percentage: f32 = descount_percentage.trim().parse().expect("A porcentagem deveria ser um número");

        let log_salary: f32 = worked_hours as f32 * class_hour_value;
        let total_descount: f32 = descount_percentage / 100.0 * log_salary;

        let liquid_salary = log_salary - total_descount;

        println!("Salário bruto: {}, Salário líquido: {}", log_salary, liquid_salary);

        println!("Digite q para sair ou aperte enter para começar de novo.");
        let mut will_quit: String = String::from("");

        io::stdin().read_line(&mut will_quit).expect("");

        if will_quit.trim() == "q" {
            break;
        }
    }
}
