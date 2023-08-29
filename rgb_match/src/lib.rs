/* rgb_match
Instructions
Implement the struct Color with the associated function swap. This function returns a Color with the matching values swapped.

Expected functions
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {

    }
} */

#[derive(Debug, PartialEq, Clone)] // Add Clone trait
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        // copy self to new variable
        let mut new_color = self.clone();



        // Swap the fields based on values
        if new_color.r == first {
            new_color.r = second;
        } else if new_color.r == second {
            new_color.r = first;
        }

        if new_color.g == first {
            new_color.g = second;
        } else if new_color.g == second {
            new_color.g = first;
        }

        if new_color.b == first {
            new_color.b = second;
        } else if new_color.b == second {
            new_color.b = first;
        }

        if new_color.a == first {
            new_color.a = second;
        } else if new_color.a == second {
            new_color.a = first;
        }

        new_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
        // swap r and b
        assert_eq!(
            c.swap(c.r, c.b),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
    }
}
