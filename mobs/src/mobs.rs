
pub mod boss;
pub mod member;
use boss::Boss;
use member::Member;
#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub cities: Vec<(String, u8)>,
    pub members: Vec<Member>,
    pub wealth: u32,
    
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        let new_member = member::Member::new(name, member::Role::Associate, age);
        self.members.push(new_member);
    }

    pub fn attack(&mut self, other_mob: &mut Mob) {
        let self_power = self.calculate_power();
        let other_power = other_mob.calculate_power();

        if self_power > other_power {
            if let Some(member) = other_mob.members.pop() {
                self.wealth += other_mob.wealth;
                self.cities.extend(other_mob.cities.drain(..));
                self.members.push(member);
            }
        } else if self_power < other_power {
            if let Some(member) = self.members.pop() {
                other_mob.wealth += self.wealth;
                other_mob.cities.extend(self.cities.drain(..));
                other_mob.members.push(member);
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let stolen_amount = amount.min(target.wealth);
        self.wealth += stolen_amount;
        target.wealth -= stolen_amount;
    }

    pub fn conquer_city(&mut self, mobs: &[Mob], city_name: &str, value: u8) {
        let city_already_taken = mobs.iter().any(|mob| mob.cities.iter().any(|(name, _)| name == city_name));
        if !city_already_taken {
            self.cities.push((city_name.to_string(), value));
        }
    }

    fn calculate_power(&self) -> u32 {
        self.members.iter().map(|member| member.role_value()).sum()
    }
}
