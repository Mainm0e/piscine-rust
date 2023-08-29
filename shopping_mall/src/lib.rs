/* shopping_mall
Instructions
Using the mall module provided, create the following functions to help run a shopping mall:

biggest_store: receives a Mall and returns the Store with the biggest square_meters.
highest_paid_employee: receives a Mall and returns a vector containing the Employee(s) with the highest salary.
nbr_of_employees: receives a Mall and returns the number of employees and guards as a usize.
check_for_securities: receives a Mall and a vector of Guard. If there is not at least 1 guard for every 200 square meters of floor size, a guard should be added to the Mall.guards.
cut_or_raise: receives a Mall. For each employee, the salary will be raised by 10% if they work more than 10 hours, else their salary will be decreased by 10%. You can consider that guards are not employees of the mall.
 */

 pub mod mall; // Assuming this is how you import the mall module
 pub use mall::*;

 pub use crate::guard::Guard;
 pub use crate::floor::Floor;
 pub use crate::floor::store;


/* pub fn biggest_store(mall: Mall) -> Option<store::Store> {
    mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores)
        .max_by(|a, b| a.square_meters.cmp(&b.square_meters))
} */

pub fn biggest_store(mall: Mall) -> store::Store{
    mall.floors
        .into_iter()
        .flat_map(|floor| floor.stores)
        .max_by(|a, b| a.square_meters.cmp(&b.square_meters))
        .expect("No stores found in the mall.")
}

pub fn highest_paid_employee(mall: Mall) -> Vec<mall::floor::store::employee::Employee> {
    let mut highest_salary = 0.0;
    let mut highest_paid_employees: Vec<mall::floor::store::employee::Employee> = Vec::new();

    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    highest_paid_employees.clear();
                    highest_paid_employees.push(employee.clone()); // Clone the employee
                } else if employee.salary == highest_salary {
                    highest_paid_employees.push(employee.clone()); // Clone the employee
                }
            }
        }
    }

    highest_paid_employees
}





pub fn nbr_of_employees(mall: Mall) -> usize {
    let total_employees: usize = mall.floors.into_iter().flat_map(|floor| floor.stores).map(|store| store.employees.len()).sum();
    total_employees + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, additional_guards: Vec<Guard>) {
    let total_guard_capacity: u64 = mall.floors.iter().map(|floor| floor.size_limit).sum::<u64>() / 200;
    let current_guards = mall.guards.len();
    let needed_guards = total_guard_capacity as i64 - current_guards as i64;

    if needed_guards > 0 {
        let mut guards_to_add = needed_guards as usize;
        guards_to_add = guards_to_add.min(additional_guards.len());
        mall.guards.extend(additional_guards.into_iter().take(guards_to_add));
    }
}



pub fn cut_or_raise(mall: &mut Mall) {
    const DECIMAL_PLACES: u32 = 6; // Number of decimal places to round to
    
    // Iterate through each floor in the mall
    for floor in &mut mall.floors {
        // Iterate through each store on the floor
        for store in &mut floor.stores {
            // Iterate through each employee in the store
            for employee in &mut store.employees.iter_mut() {
                    // Check if the employee's working hours are more than 10
                    if employee.working_hours.1 - employee.working_hours.0 > 10 {
                        // Raise the salary by 10%
                       employee.salary += employee.salary * 0.1;
                    } else {
                        // Decrease the salary by 10%
                        employee.salary -= employee.salary * 0.1;
                    }

                    // Round the salary to the specified number of decimal places
                    employee.salary = (employee.salary * 10_f64.powi(DECIMAL_PLACES as i32)).round() / 10_f64.powi(DECIMAL_PLACES as i32);
            }
        }
    }
}
