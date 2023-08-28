// todo: macro_calculator
/* 
Instructions
Create a function named calculate_macros which receives a vector of Food structures and returns a json::JsonValue.

Food {
    name: <name>,
    calories: [<value_in_kJ>, <value_in_kcal>],
    fats: <fats_in_g>,
    carbs: <carbs_in_g>,
    proteins: <proteins_in_g>,
    nbr_of_portions: <portions>
}
The values in the calories array will be of type string, all other values will be f64.

The json returned by calculate_macros will have the following format:

"cals": <calories>,
"carbs": <carbs>,
"proteins": <proteins>,
"fats": <fats>,
Consider the number of portions, as the values of the macros refer to one portion. Each value should represent the sum of each micro-nutrient in the array. E.g. cals is the sum of all calories. Every value should be f64 and be rounded rounded to two decimal places, or one decimal place if it ends in a zero. E.g:

12.294 -> 12.29
12.295 -> 12.30 -> 12.3
Expected Function
pub struct Food {
    //expected public fields
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {

} */


use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut macros = json::JsonValue::new_object();
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        cals += food.calories[1].replace("kcal", "").parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }
    // modify cals, carbs, proteins, fats if 0.000 make it 0.00
    let cals = format!("{:.2}", cals).parse::<f64>().unwrap();
    let carbs = format!("{:.2}", carbs).parse::<f64>().unwrap();
    let proteins = format!("{:.2}", proteins).parse::<f64>().unwrap();
    let fats = format!("{:.2}", fats).parse::<f64>().unwrap();

    macros["cals"] = json::JsonValue::from(cals);
    macros["carbs"] = json::JsonValue::from(carbs);
    macros["proteins"] = json::JsonValue::from(proteins);
    macros["fats"] = json::JsonValue::from(fats);
 
    macros
}

