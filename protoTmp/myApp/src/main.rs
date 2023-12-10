use generated::company::*;
use generated::company::CompanyCategory;

fn main() {
    // Utworzenie instancji struktury Company
    let mut my_company = Company::new();
    my_company.set_code(CompanyCategory::LEGAL);
    my_company.name = "dupa".to_string();
    my_company.company_id = 69;
    print!("{}", my_company);

}
